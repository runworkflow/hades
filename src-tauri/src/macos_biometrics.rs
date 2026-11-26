use std::ffi::{CString, c_char};

use hades_core::{BiometricEnrollment, BiometricError, BiometricProvider, SecretKey};
use zeroize::Zeroizing;

const BLOB_CAPACITY: usize = 4096;
const PUBLIC_KEY_LENGTH: usize = 65;
const KEY_LENGTH: usize = 32;

const STATUS_OK: i32 = 0;
const STATUS_CANCELLED: i32 = 1;
const STATUS_UNAVAILABLE: i32 = 2;

unsafe extern "C" {
    fn hades_biometrics_available() -> i32;
    fn hades_biometrics_enroll(
        reason: *const c_char,
        blob_out: *mut u8,
        blob_capacity: usize,
        blob_length: *mut usize,
        public_key_out: *mut u8,
        public_key_capacity: usize,
        key_out: *mut u8,
    ) -> i32;
    fn hades_biometrics_derive(
        reason: *const c_char,
        blob: *const u8,
        blob_length: usize,
        public_key: *const u8,
        public_key_length: usize,
        key_out: *mut u8,
    ) -> i32;
}

fn error_for(status: i32) -> BiometricError {
    match status {
        STATUS_CANCELLED => BiometricError::Cancelled,
        STATUS_UNAVAILABLE => BiometricError::Unavailable,
        _ => BiometricError::Failed,
    }
}

pub struct MacosBiometrics;

impl BiometricProvider for MacosBiometrics {
    fn is_available(&self) -> bool {
        unsafe { hades_biometrics_available() == 1 }
    }

    fn enroll(&self, reason: &str) -> Result<BiometricEnrollment, BiometricError> {
        let reason = CString::new(reason).map_err(|_| BiometricError::Failed)?;
        let mut blob = vec![0u8; BLOB_CAPACITY];
        let mut blob_length: usize = 0;
        let mut public_key = vec![0u8; PUBLIC_KEY_LENGTH];
        let mut key = Zeroizing::new([0u8; KEY_LENGTH]);

        let status = unsafe {
            hades_biometrics_enroll(
                reason.as_ptr(),
                blob.as_mut_ptr(),
                blob.len(),
                &mut blob_length,
                public_key.as_mut_ptr(),
                public_key.len(),
                key.as_mut_ptr(),
            )
        };
        if status != STATUS_OK {
            return Err(error_for(status));
        }

        blob.truncate(blob_length);
        let wrapping_key = SecretKey::from_bytes(&key[..]).map_err(|_| BiometricError::Failed)?;
        Ok(BiometricEnrollment {
            key_blob: blob,
            ephemeral_public_key: public_key,
            wrapping_key,
        })
    }

    fn derive_key(
        &self,
        key_blob: &[u8],
        ephemeral_public_key: &[u8],
        reason: &str,
    ) -> Result<SecretKey, BiometricError> {
        let reason = CString::new(reason).map_err(|_| BiometricError::Failed)?;
        let mut key = Zeroizing::new([0u8; KEY_LENGTH]);

        let status = unsafe {
            hades_biometrics_derive(
                reason.as_ptr(),
                key_blob.as_ptr(),
                key_blob.len(),
                ephemeral_public_key.as_ptr(),
                ephemeral_public_key.len(),
                key.as_mut_ptr(),
            )
        };
        if status != STATUS_OK {
            return Err(error_for(status));
        }
        SecretKey::from_bytes(&key[..]).map_err(|_| BiometricError::Failed)
    }
}
