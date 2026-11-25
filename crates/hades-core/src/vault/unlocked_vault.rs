use crate::secret_key::SecretKey;

pub struct UnlockedVault {
    vault_key: SecretKey,
}

impl UnlockedVault {
    pub(super) fn new(vault_key: SecretKey) -> Self {
        Self { vault_key }
    }

    pub fn vault_key(&self) -> &SecretKey {
        &self.vault_key
    }

    pub fn lock(self) {
        drop(self);
    }
}
