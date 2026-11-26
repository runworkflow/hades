use std::error::Error;
use std::fs::{self, DirBuilder, Permissions};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use hades_core::{
    AutoLockMonitor, AutoLockPolicy, FileBlobStore, SqliteStore, UnlockThrottle, VaultSession,
};

use crate::clock::current_unix_millis;
use crate::secret_clipboard::SecretClipboard;
use crate::sync_folder_file;
use crate::sync_state::SyncState;

const VAULT_FILE_NAME: &str = "vault.db";
const BLOBS_DIRECTORY_NAME: &str = "blobs";
const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;

pub struct AppVault {
    pub session: Mutex<VaultSession<SqliteStore>>,
    pub blobs: FileBlobStore,
    pub auto_lock: Mutex<AutoLockMonitor>,
    pub unlock_throttle: Mutex<UnlockThrottle>,
    pub clipboard: SecretClipboard,
    pub sync: SyncState,
    pub data_directory: PathBuf,
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
            unlock_throttle: Mutex::new(UnlockThrottle::default()),
            clipboard: SecretClipboard::default(),
            sync: SyncState::with_folder(sync_folder_file::load(data_directory)),
            data_directory: data_directory.to_path_buf(),
        })
    }

    pub fn after_unlock(&self) {
        self.remove_orphan_blobs();
        self.sync.request();
    }

    pub fn note_change(&self) {
        self.sync.note_change(current_unix_millis());
    }

    pub fn remove_orphan_blobs(&self) {
        if let Ok(session) = self.session.lock() {
            let _ = session.remove_orphan_blobs(&self.blobs);
        }
    }
}
