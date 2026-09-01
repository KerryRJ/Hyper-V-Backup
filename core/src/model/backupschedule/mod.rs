use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};

use super::VmId;

mod scheduleid;

pub use scheduleid::ScheduleId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSchedule {
    pub id: ScheduleId,
    pub virtual_machine_id: VmId,
    pub destination: PathBuf,
    pub next_run_at: DateTime<Utc>,
    pub repeat_every: Option<Duration>,
    pub enabled: bool,
}

impl BackupSchedule {
    pub fn new(
        virtual_machine_id: VmId,
        destination: PathBuf,
        first_run_at: DateTime<Utc>,
        repeat_every: Option<Duration>,
    ) -> Self {
        Self {
            id: ScheduleId::new_v4(),
            virtual_machine_id,
            destination,
            next_run_at: first_run_at,
            repeat_every,
            enabled: true,
        }
    }
}
