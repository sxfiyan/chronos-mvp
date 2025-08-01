# Chronos MVP v0.1

A forensic timeline generator for Windows 11 disk images that automates the initial phase of forensic analysis by parsing key artifacts and generating a human-readable event timeline.

## Project Overview

Chronos MVP validates the hypothesis that a specialized tool can automate the initial, time-consuming phase of forensic analysis, providing significant value by reducing manual labor from hours to minutes.

## Features

### Core Functionality
- **Single Executable**: Self-contained CLI tool with no external dependencies
- **Windows 11 Support**: Optimized for Windows 11 Pro (x64) forensic analysis
- **Multiple Artifact Parsing**: 
  - Master File Table (MFT) - File/folder timestamps (MACB)
  - Windows Event Logs - User logons (4624) and service installations (7045)
  - Prefetch Files - Program execution timestamps
- **Chronological Timeline**: All events sorted by timestamp
- **HTML Output**: Sortable timeline table with modern UI

### Supported Input Formats
- `.E01` (EnCase) disk images
- `.dd` (raw) disk images

### Performance
- Designed to process 256GB disk images in under 20 minutes
- Optimized for standard machines (4-core CPU, 8GB RAM)

## Installation

### Prerequisites
- Windows 10/11 x64
- No additional dependencies required

### Building from Source
```bash
# Clone the repository
git clone <repository-url>
cd chronos-mvp

# Build the release version
cargo build --release

# The executable will be available at target/release/chronos.exe
```

## Usage

### Basic Usage
```bash
chronos.exe <path-to-disk-image>
```

### Examples
```bash
# Process an EnCase image
chronos.exe evidence.E01

# Process a raw disk image
chronos.exe disk.dd
```

### Output
The tool generates a `timeline.html` file in the current directory containing:
- Chronologically sorted events
- Sortable columns (Timestamp, Event Type, Description, Source Artifact)
- Modern, responsive web interface

## Technical Architecture

### Core Components

#### 1. Disk Image Handler (`disk_image.rs`)
- Memory-mapped file access for efficient processing
- Support for .E01 and .dd formats
- Bounds checking and error handling

#### 2. MFT Parser (`mft_parser.rs`)
- Parses Master File Table entries
- Extracts file/folder names and MACB timestamps
- Handles NTFS file system structures

#### 3. Event Log Parser (`event_log_parser.rs`)
- Parses Windows Event Logs (.evtx files)
- Extracts user logon events (Event ID 4624)
- Extracts service installation events (Event ID 7045)

#### 4. Prefetch Parser (`prefetch_parser.rs`)
- Parses Windows Prefetch files (.pf)
- Extracts executable names and run timestamps
- Handles prefetch file format

#### 5. Timeline Manager (`timeline.rs`)
- Centralized event storage and management
- Chronological sorting
- Event type categorization

#### 6. HTML Generator (`html_generator.rs`)
- Generates sortable HTML timeline
- Modern CSS styling
- JavaScript-powered table sorting

### Event Types

| Event Type | Description | Source Artifact |
|------------|-------------|-----------------|
| File Creation | File was created | MFT |
| File Modification | File was modified | MFT |
| File Access | File was accessed | MFT |
| MFT Entry Changed | MFT entry was modified | MFT |
| User Logon | User successfully logged on | Security.evtx |
| Service Installation | Service was installed | System.evtx |
| Program Execution | Executable was run | Prefetch files |

## Error Handling

The tool handles various error conditions gracefully:
- Invalid or missing disk image files
- Unsupported image formats
- Corrupted or unreadable data
- Memory allocation failures

All errors are logged with clear, actionable messages.

## Performance Considerations

### Memory Usage
- Memory-mapped file access for large disk images
- Efficient data structures for timeline storage
- Streaming processing to avoid loading entire image into memory

### Processing Speed
- Parallel processing where applicable
- Optimized parsing algorithms
- Limited processing scope for MVP (1000 MFT entries)

## Development Status

### MVP Limitations
For the MVP version, some features are simulated rather than fully implemented:
- Event log parsing uses sample data
- Prefetch parsing uses sample data
- MFT parsing is limited to 1000 entries

### Production Roadmap
Future versions will include:
- Full NTFS file system traversal
- Complete .evtx file parsing
- Complete prefetch file parsing
- Support for additional artifact types
- Performance optimizations for larger images

## Testing

### Test Cases

#### Test Case 1: Execution
- Tool executes against Windows 11 test image without crashing
- Proper error handling for invalid inputs

#### Test Case 2: Accuracy
- Correctly identifies known events from test image
- Accurate timestamp extraction and conversion
- Proper event categorization

#### Test Case 3: Performance
- Meets 20-minute processing time for 256GB images
- Efficient memory usage
- Responsive user interface

## License

This project is developed as part of the Chronos MVP initiative.

## Contributing

This is an MVP version for validation purposes. Future development will focus on:
- Full implementation of artifact parsing
- Performance optimizations
- Additional artifact support
- Enhanced error handling

## Support

For issues and questions related to the MVP, please refer to the project documentation or create an issue in the repository. 