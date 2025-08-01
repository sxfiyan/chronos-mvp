use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, Utc, TimeZone};
use std::io::{Cursor, Read};
use tracing::info;

use crate::disk_image::DiskImage;
use crate::timeline::{Timeline, EventType};

const MFT_SIGNATURE: &[u8; 4] = b"FILE";
const MFT_ENTRY_SIZE: usize = 1024;

#[derive(Debug)]
struct MftEntry {
    signature: [u8; 4],
    sequence_number: u16,
    link_count: u16,
    attribute_offset: u16,
    flags: u16,
    entry_size: u32,
    entry_allocated: u32,
    file_reference: u64,
    base_file_record: u64,
    next_attribute_id: u16,
    attributes: Vec<MftAttribute>,
}

#[derive(Debug)]
struct MftAttribute {
    attribute_type: u32,
    attribute_length: u32,
    non_resident: bool,
    name_length: u8,
    name_offset: u16,
    flags: u16,
    attribute_id: u16,
    content: Vec<u8>,
}

#[derive(Debug)]
struct FileNameAttribute {
    parent_directory: u64,
    creation_time: u64,
    last_access_time: u64,
    last_write_time: u64,
    mft_change_time: u64,
    file_size: u64,
    allocated_size: u64,
    file_flags: u32,
    filename_length: u8,
    filename: String,
}

pub fn parse_mft(disk_image: &DiskImage, timeline: &mut Timeline) -> Result<()> {
    info!("Starting MFT parsing...");
    
    // For MVP, we'll implement a simplified MFT parser
    // In a production version, this would need to handle NTFS structures more comprehensively
    
    // Look for MFT entries in the disk image
    let mut offset = 0;
    let mut events_found = 0;
    
    while offset + MFT_ENTRY_SIZE <= disk_image.size() {
        if let Ok(entry) = parse_mft_entry(disk_image, offset) {
            if let Some(file_info) = extract_file_info(&entry) {
                add_file_events_to_timeline(timeline, &file_info);
                events_found += 1;
            }
        }
        
        offset += MFT_ENTRY_SIZE;
        
        // Limit processing for MVP to avoid excessive processing time
        if events_found > 1000 {
            info!("MFT parsing limited to 1000 entries for MVP");
            break;
        }
    }
    
    info!("MFT parsing completed. Found {} file events", events_found);
    Ok(())
}

fn parse_mft_entry(disk_image: &DiskImage, offset: usize) -> Result<MftEntry> {
    let data = disk_image.get_slice(offset, MFT_ENTRY_SIZE)?;
    let mut cursor = Cursor::new(data);
    
    let mut signature = [0u8; 4];
    cursor.read_exact(&mut signature)?;
    
    if signature != *MFT_SIGNATURE {
        anyhow::bail!("Invalid MFT entry signature");
    }
    
    let sequence_number = cursor.read_u16::<LittleEndian>()?;
    let link_count = cursor.read_u16::<LittleEndian>()?;
    let attribute_offset = cursor.read_u16::<LittleEndian>()?;
    let flags = cursor.read_u16::<LittleEndian>()?;
    let entry_size = cursor.read_u32::<LittleEndian>()?;
    let entry_allocated = cursor.read_u32::<LittleEndian>()?;
    let file_reference = cursor.read_u64::<LittleEndian>()?;
    let base_file_record = cursor.read_u64::<LittleEndian>()?;
    let next_attribute_id = cursor.read_u16::<LittleEndian>()?;
    
    // Parse attributes (simplified for MVP)
    let mut attributes = Vec::new();
    let mut attr_offset = attribute_offset as usize;
    
    while attr_offset < MFT_ENTRY_SIZE - 4 {
        if let Ok(attr) = parse_attribute(&data[attr_offset..]) {
            let attr_length = attr.attribute_length as usize;
            attributes.push(attr);
            attr_offset += attr_length;
        } else {
            break;
        }
    }
    
    Ok(MftEntry {
        signature,
        sequence_number,
        link_count,
        attribute_offset,
        flags,
        entry_size,
        entry_allocated,
        file_reference,
        base_file_record,
        next_attribute_id,
        attributes,
    })
}

fn parse_attribute(data: &[u8]) -> Result<MftAttribute> {
    if data.len() < 16 {
        anyhow::bail!("Attribute data too short");
    }
    
    let mut cursor = Cursor::new(data);
    let attribute_type = cursor.read_u32::<LittleEndian>()?;
    let attribute_length = cursor.read_u32::<LittleEndian>()?;
    let non_resident = cursor.read_u8()? != 0;
    let name_length = cursor.read_u8()?;
    let name_offset = cursor.read_u16::<LittleEndian>()?;
    let flags = cursor.read_u16::<LittleEndian>()?;
    let attribute_id = cursor.read_u16::<LittleEndian>()?;
    
    let content = if non_resident {
        // For MVP, skip non-resident attributes
        Vec::new()
    } else {
        let content_offset = cursor.read_u16::<LittleEndian>()? as usize;
        let content_size = cursor.read_u16::<LittleEndian>()? as usize;
        
        if content_offset + content_size <= data.len() {
            data[content_offset..content_offset + content_size].to_vec()
        } else {
            Vec::new()
        }
    };
    
    Ok(MftAttribute {
        attribute_type,
        attribute_length,
        non_resident,
        name_length,
        name_offset,
        flags,
        attribute_id,
        content,
    })
}

fn extract_file_info(entry: &MftEntry) -> Option<FileNameAttribute> {
    // Look for $FILE_NAME attribute (0x30)
    for attr in &entry.attributes {
        if attr.attribute_type == 0x30 && !attr.content.is_empty() {
            if let Ok(file_info) = parse_filename_attribute(&attr.content) {
                return Some(file_info);
            }
        }
    }
    None
}

fn parse_filename_attribute(data: &[u8]) -> Result<FileNameAttribute> {
    if data.len() < 66 {
        anyhow::bail!("Filename attribute data too short");
    }
    
    let mut cursor = Cursor::new(data);
    let parent_directory = cursor.read_u64::<LittleEndian>()?;
    let creation_time = cursor.read_u64::<LittleEndian>()?;
    let last_access_time = cursor.read_u64::<LittleEndian>()?;
    let last_write_time = cursor.read_u64::<LittleEndian>()?;
    let mft_change_time = cursor.read_u64::<LittleEndian>()?;
    let file_size = cursor.read_u64::<LittleEndian>()?;
    let allocated_size = cursor.read_u64::<LittleEndian>()?;
    let file_flags = cursor.read_u32::<LittleEndian>()?;
    let filename_length = cursor.read_u8()?;
    
    let filename_bytes = &data[66..66 + filename_length as usize * 2];
    let filename = String::from_utf16_lossy(
        &filename_bytes.chunks(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>()
    );
    
    Ok(FileNameAttribute {
        parent_directory,
        creation_time,
        last_access_time,
        last_write_time,
        mft_change_time,
        file_size,
        allocated_size,
        file_flags,
        filename_length,
        filename,
    })
}

fn add_file_events_to_timeline(timeline: &mut Timeline, file_info: &FileNameAttribute) {
    // Convert Windows FILETIME to UTC DateTime
    let creation_time = windows_time_to_utc(file_info.creation_time);
    let access_time = windows_time_to_utc(file_info.last_access_time);
    let write_time = windows_time_to_utc(file_info.last_write_time);
    let mft_change_time = windows_time_to_utc(file_info.mft_change_time);
    
    // Add events to timeline
    if creation_time > Utc::now() - chrono::Duration::days(365) {
        timeline.add_file_event(creation_time, EventType::FileCreation, 
                              &file_info.filename, "MFT");
    }
    
    if access_time > Utc::now() - chrono::Duration::days(365) {
        timeline.add_file_event(access_time, EventType::FileAccess, 
                              &file_info.filename, "MFT");
    }
    
    if write_time > Utc::now() - chrono::Duration::days(365) {
        timeline.add_file_event(write_time, EventType::FileModification, 
                              &file_info.filename, "MFT");
    }
    
    if mft_change_time > Utc::now() - chrono::Duration::days(365) {
        timeline.add_file_event(mft_change_time, EventType::FileMftChange, 
                              &file_info.filename, "MFT");
    }
}

fn windows_time_to_utc(windows_time: u64) -> DateTime<Utc> {
    // Windows FILETIME is 100-nanosecond intervals since 1601-01-01
    // Convert to Unix timestamp (seconds since 1970-01-01)
    let unix_seconds = (windows_time as i64 - 116444736000000000) / 10000000;
    match Utc.timestamp_opt(unix_seconds, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => Utc::now(),
    }
} 