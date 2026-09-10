use std::collections::HashMap;
use std::sync::Arc;

use super::{BackupId, BackupStatus};

pub(crate) type StatusStore = Arc<tokio::sync::RwLock<HashMap<BackupId, BackupStatus>>>;
