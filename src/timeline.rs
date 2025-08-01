use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub description: String,
    pub source_artifact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    FileCreation,
    FileModification,
    FileAccess,
    FileMftChange,
    UserLogon,
    ServiceInstallation,
    ProgramExecution,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::FileCreation => write!(f, "File Creation"),
            EventType::FileModification => write!(f, "File Modification"),
            EventType::FileAccess => write!(f, "File Access"),
            EventType::FileMftChange => write!(f, "MFT Entry Changed"),
            EventType::UserLogon => write!(f, "User Logon"),
            EventType::ServiceInstallation => write!(f, "Service Installation"),
            EventType::ProgramExecution => write!(f, "Program Execution"),
        }
    }
}

pub struct Timeline {
    pub events: Vec<TimelineEvent>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            events: Vec::new(),
        }
    }
    
    pub fn add_event(&mut self, event: TimelineEvent) {
        self.events.push(event);
    }
    
    pub fn add_file_event(&mut self, timestamp: DateTime<Utc>, event_type: EventType, 
                         file_path: &str, source: &str) {
        let description = match event_type {
            EventType::FileCreation => format!("File '{}' was created.", file_path),
            EventType::FileModification => format!("File '{}' was modified.", file_path),
            EventType::FileAccess => format!("File '{}' was accessed.", file_path),
            EventType::FileMftChange => format!("MFT entry for '{}' was changed.", file_path),
            _ => format!("File '{}' event occurred.", file_path),
        };
        
        self.events.push(TimelineEvent {
            timestamp,
            event_type,
            description,
            source_artifact: source.to_string(),
        });
    }
    
    pub fn add_user_logon(&mut self, timestamp: DateTime<Utc>, username: &str, 
                          source_ip: &str) {
        self.events.push(TimelineEvent {
            timestamp,
            event_type: EventType::UserLogon,
            description: format!("User '{}' logged on from source IP {}", username, source_ip),
            source_artifact: "Security.evtx".to_string(),
        });
    }
    
    pub fn add_service_installation(&mut self, timestamp: DateTime<Utc>, 
                                   service_name: &str) {
        self.events.push(TimelineEvent {
            timestamp,
            event_type: EventType::ServiceInstallation,
            description: format!("Service '{}' was installed.", service_name),
            source_artifact: "System.evtx".to_string(),
        });
    }
    
    pub fn add_program_execution(&mut self, timestamp: DateTime<Utc>, 
                                executable_name: &str, prefetch_file: &str) {
        self.events.push(TimelineEvent {
            timestamp,
            event_type: EventType::ProgramExecution,
            description: format!("Executable '{}' was run.", executable_name),
            source_artifact: prefetch_file.to_string(),
        });
    }
    
    pub fn sort(&mut self) {
        self.events.sort_by(|a, b| {
            a.timestamp.cmp(&b.timestamp)
        });
    }
    
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
} 