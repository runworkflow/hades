use std::error::Error;
use std::fs::{self, DirBuilder, Permissions};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::Path;
use std::sync::Mutex;

use hades_core::{AutoLockMonitor, AutoLockPolicy, SqliteStore, VaultSession};

use crate::clock::current_unix_millis;

const VAULT_FILE_NAME: &str = "vault.db";
const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;

pub struct AppVault {
    pub session: Mutex<VaultSession<SqliteStore>>,
    pub auto_lock: Mutex<AutoLockMonitor>,
}

impl AppVault {
    pub fn open(data_directory: &Path) -> Result<Self, Box<dyn Error>> {
        DirBuilder::new()
            .recursive(true)
            .mode(OWNER_ONLY_DIRECTORY_MODE)
            .create(data_directory)?;
        fs::set_permissions(
            data_directory,
            Permissions::from_mode(OWNER_ONLY_DIRECTORY_MODE),
        )?;

        let store = SqliteStore::open(&data_directory.join(VAULT_FILE_NAME))?;
        Ok(Self {
            session: Mutex::new(VaultSession::new(store)),
            auto_lock: Mutex::new(AutoLockMonitor::new(
                AutoLockPolicy::default(),
                current_unix_millis(),
            )),
        })
    }
}
