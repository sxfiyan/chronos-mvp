use clap::Parser;
use anyhow::{Context, Result};
use tracing::{info, Level};
use tracing_subscriber;

mod disk_image;
mod mft_parser;
mod event_log_parser;
mod prefetch_parser;
mod timeline;
mod html_generator;

use disk_image::DiskImage;
use timeline::Timeline;

#[derive(Parser, Debug)]
#[command(name = "chronos")]
#[command(about = "Forensic timeline generator for Windows 11 disk images")]
#[command(version)]
struct Args {
    /// Path to the forensic disk image file (.E01 or .dd)
    #[arg(required = true)]
    image_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Chronos MVP v0.1 - Forensic Timeline Generator");
    
    let args = Args::parse();
    
    // Validate input file
    let image_path = std::path::Path::new(&args.image_path);
    if !image_path.exists() {
        anyhow::bail!("Image file not found: {}", args.image_path);
    }
    
    let extension = image_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    if !matches!(extension.to_lowercase().as_str(), "e01" | "dd") {
        anyhow::bail!("Unsupported image format. Supported formats: .E01, .dd");
    }
    
    info!("Processing disk image: {}", args.image_path);
    
    // Load and process the disk image
    let disk_image = DiskImage::new(&args.image_path)
        .context("Failed to load disk image")?;
    
    // Create timeline
    let mut timeline = Timeline::new();
    
    // Parse MFT
    info!("Parsing Master File Table (MFT)...");
    mft_parser::parse_mft(&disk_image, &mut timeline)
        .context("Failed to parse MFT")?;
    
    // Parse Windows Event Logs
    info!("Parsing Windows Event Logs...");
    event_log_parser::parse_event_logs(&disk_image, &mut timeline)
        .context("Failed to parse event logs")?;
    
    // Parse Prefetch files
    info!("Parsing Prefetch files...");
    prefetch_parser::parse_prefetch_files(&disk_image, &mut timeline)
        .context("Failed to parse prefetch files")?;
    
    // Sort timeline chronologically
    timeline.sort();
    
    // Generate HTML output
    info!("Generating timeline.html...");
    html_generator::generate_html(&timeline)
        .context("Failed to generate HTML output")?;
    
    info!("Timeline generation completed successfully!");
    info!("Output file: timeline.html");
    
    Ok(())
}
