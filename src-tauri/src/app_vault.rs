use std::error::Error;
use std::fs::{self, DirBuilder, Permissions};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::Path;
use std::sync::Mutex;

use hades_core::{AutoLockMonitor, AutoLockPolicy, FileBlobStore, SqliteStore, VaultSession};

use crate::clock::current_unix_millis;
use crate::secret_clipboard::SecretClipboard;

const VAULT_FILE_NAME: &str = "vault.db";
const BLOBS_DIRECTORY_NAME: &str = "blobs";
const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;

pub struct AppVault {
    pub session: Mutex<VaultSession<SqliteStore>>,
    pub blobs: FileBlobStore,
    pub auto_lock: Mutex<AutoLockMonitor>,
    pub clipboard: SecretClipboard,
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
        let blobs = FileBlobStore::open(&data_directory.join(BLOBS_DIRECTORY_NAME))?;
        blobs.remove_temporary_files()?;
        Ok(Self {
            session: Mutex::new(VaultSession::new(store)),
            blobs,
            auto_lock: Mutex::new(AutoLockMonitor::new(
                AutoLockPolicy::default(),
                current_unix_millis(),
            )),
            clipboard: SecretClipboard::default(),
        })
    }

    pub fn remove_orphan_blobs(&self) {
        if let Ok(session) = self.session.lock() {
            let _ = session.remove_orphan_blobs(&self.blobs);
        }
    }
}
