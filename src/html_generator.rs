use anyhow::{Context, Result};
use maud::{html, Markup, DOCTYPE};
use std::fs::File;
use std::io::Write;
use tracing::info;

use crate::timeline::Timeline;

pub fn generate_html(timeline: &Timeline) -> Result<()> {
    info!("Generating HTML timeline...");
    
    let html_content = create_timeline_html(timeline);
    
    let mut file = File::create("timeline.html")
        .context("Failed to create timeline.html file")?;
    
    file.write_all(html_content.0.as_bytes())
        .context("Failed to write HTML content")?;
    
    info!("HTML timeline generated successfully");
    Ok(())
}

fn create_timeline_html(timeline: &Timeline) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Chronos Forensic Timeline" }
                style { (get_css_styles()) }
                script { (get_javascript()) }
            }
            body {
                div class="container" {
                    h1 { "Chronos Forensic Timeline" }
                    p class="summary" {
                        "Generated " (timeline.events.len()) " events from forensic disk image analysis."
                    }
                    table id="timeline-table" class="timeline-table" {
                        thead {
                            tr {
                                th class="sortable" data-sort="timestamp" { "Timestamp (UTC)" }
                                th class="sortable" data-sort="event-type" { "Event Type" }
                                th class="sortable" data-sort="description" { "Description" }
                                th class="sortable" data-sort="source" { "Source Artifact" }
                            }
                        }
                        tbody {
                            @for event in &timeline.events {
                                tr {
                                    td class="timestamp" { (format_timestamp(event.timestamp)) }
                                    td class="event-type" { (event.event_type.to_string()) }
                                    td class="description" { (event.description) }
                                    td class="source" { (event.source_artifact) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_timestamp(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn get_css_styles() -> &'static str {
    r#"
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        
        h1 {
            color: #2c3e50;
            text-align: center;
            margin-bottom: 10px;
        }
        
        .summary {
            text-align: center;
            color: #7f8c8d;
            margin-bottom: 30px;
        }
        
        .timeline-table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
            font-size: 14px;
        }
        
        .timeline-table th {
            background-color: #34495e;
            color: white;
            padding: 12px 8px;
            text-align: left;
            cursor: pointer;
            user-select: none;
        }
        
        .timeline-table th:hover {
            background-color: #2c3e50;
        }
        
        .timeline-table th.sortable::after {
            content: " ↕";
            font-size: 12px;
            opacity: 0.7;
        }
        
        .timeline-table th.sort-asc::after {
            content: " ↑";
        }
        
        .timeline-table th.sort-desc::after {
            content: " ↓";
        }
        
        .timeline-table td {
            padding: 8px;
            border-bottom: 1px solid #ecf0f1;
        }
        
        .timeline-table tr:hover {
            background-color: #f8f9fa;
        }
        
        .timestamp {
            font-family: 'Courier New', monospace;
            font-size: 12px;
            color: #2c3e50;
        }
        
        .event-type {
            font-weight: bold;
            color: #e74c3c;
        }
        
        .description {
            max-width: 400px;
            word-wrap: break-word;
        }
        
        .source {
            font-family: 'Courier New', monospace;
            font-size: 12px;
            color: #7f8c8d;
        }
        
        .file-creation { color: #27ae60; }
        .file-modification { color: #f39c12; }
        .file-access { color: #3498db; }
        .mft-change { color: #9b59b6; }
        .user-logon { color: #e74c3c; }
        .service-installation { color: #e67e22; }
        .program-execution { color: #1abc9c; }
    "#
}

fn get_javascript() -> &'static str {
    r#"
        document.addEventListener('DOMContentLoaded', function() {
            const table = document.getElementById('timeline-table');
            const headers = table.querySelectorAll('th.sortable');
            
            headers.forEach(header => {
                header.addEventListener('click', function() {
                    const column = this.dataset.sort;
                    const isAsc = this.classList.contains('sort-asc');
                    
                    // Remove all sort classes
                    headers.forEach(h => {
                        h.classList.remove('sort-asc', 'sort-desc');
                    });
                    
                    // Add appropriate sort class
                    this.classList.add(isAsc ? 'sort-desc' : 'sort-asc');
                    
                    // Sort the table
                    sortTable(column, !isAsc);
                });
            });
            
            function sortTable(column, ascending) {
                const tbody = table.querySelector('tbody');
                const rows = Array.from(tbody.querySelectorAll('tr'));
                
                rows.sort((a, b) => {
                    const aValue = getCellValue(a, column);
                    const bValue = getCellValue(b, column);
                    
                    if (column === 'timestamp') {
                        return ascending ? 
                            new Date(aValue) - new Date(bValue) :
                            new Date(bValue) - new Date(aValue);
                    } else {
                        return ascending ? 
                            aValue.localeCompare(bValue) :
                            bValue.localeCompare(aValue);
                    }
                });
                
                // Reorder rows
                rows.forEach(row => tbody.appendChild(row));
            }
            
            function getCellValue(row, column) {
                const cellIndex = {
                    'timestamp': 0,
                    'event-type': 1,
                    'description': 2,
                    'source': 3
                };
                
                return row.cells[cellIndex[column]].textContent.trim();
            }
        });
    "#
} 