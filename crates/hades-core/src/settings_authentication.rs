use crate::error::CryptoResult;
use crate::sealed::{Sealed, open, seal};
use crate::secret_key::SecretKey;
use crate::store::VaultSettings;

const AUTHENTICATION_CONTEXT: &[u8] = b"hades:settings-authentication:v1";

fn push_field(bytes: &mut Vec<u8>, field: &[u8]) {
    bytes.extend_from_slice(&(field.len() as u64).to_be_bytes());
    bytes.extend_from_slice(field);
}

fn push_sealed(bytes: &mut Vec<u8>, sealed: &Sealed) {
    push_field(bytes, &sealed.nonce);
    push_field(bytes, &sealed.ciphertext);
}

fn signed_bytes(settings: &VaultSettings) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_field(&mut bytes, AUTHENTICATION_CONTEXT);
    push_field(&mut bytes, settings.vault_id.as_bytes());
    push_field(&mut bytes, &settings.settings_version.to_be_bytes());
    push_field(&mut bytes, &settings.created_at.to_be_bytes());
    push_field(&mut bytes, settings.kdf_salt.as_bytes());
    push_field(&mut bytes, &settings.kdf_params.memory_kib.to_be_bytes());
    push_field(&mut bytes, &settings.kdf_params.iterations.to_be_bytes());
    push_field(&mut bytes, &settings.kdf_params.lanes.to_be_bytes());
    push_sealed(&mut bytes, &settings.password_wrapped_key.0);
    push_sealed(&mut bytes, &settings.recovery_wrapped_key.0);
    bytes
}

pub fn sign_settings(vault_key: &SecretKey, settings: &VaultSettings) -> CryptoResult<Sealed> {
    seal(vault_key, &[], &signed_bytes(settings))
}

pub fn is_settings_authentic(
    vault_key: &SecretKey,
    settings: &VaultSettings,
    authentication: &Sealed,
) -> bool {
    open(vault_key, authentication, &signed_bytes(settings)).is_ok()
}
