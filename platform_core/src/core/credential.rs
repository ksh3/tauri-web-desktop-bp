use base62;
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha512;
use uuid::Uuid;
use zeroize::Zeroizing;

const __PBKDF2_ROUNDS: u32 = 64;

pub trait CredentialManager: Send + Sync + 'static {
    // NOTE: Do not change the default rounds value.
    // The logic for generating unique database IDs and passwords is based on this value.
    // It will not be reproductive if the value has changed.
    fn generate_tenant_id(&self) -> String;
    fn generate_db_name(&self, tenant_name: &str, tenant_id: &str) -> String;
    fn generate_db_password(&self, tenant_id: &str) -> String;
}

pub struct PlatformCredentialManager {
    secret_key: Zeroizing<Vec<u8>>,
}

impl PlatformCredentialManager {
    pub fn new(secret_key: Zeroizing<Vec<u8>>) -> Self {
        // e.g.: Load the secret key from GCP Secret Manager.
        Self { secret_key }
    }
}

impl CredentialManager for PlatformCredentialManager {
    fn generate_tenant_id(&self) -> String {
        let uuid = Uuid::new_v4();
        let num = uuid.as_u128();
        base62::encode(num)
    }

    fn generate_db_name(&self, tenant_name: &str, tenant_id: &str) -> String {
        format!("{}_{}", tenant_name, tenant_id)
    }

    fn generate_db_password(&self, tenant_id: &str) -> String {
        let salt = tenant_id.as_bytes();
        // NOTE: Never change the byte length[64] of the derived key.
        // The logic for generating unique database passwords is based on this value.
        // It will not be reproductive if the value has changed.
        let derived: [u8; 64] = pbkdf2_hmac_array::<Sha512, 64>(
            &self.secret_key,
            salt,
            __PBKDF2_ROUNDS,
        );
        general_purpose::STANDARD.encode(derived)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn setup_credential_manager() -> PlatformCredentialManager {
        let secret_key = Zeroizing::new(b"supersecretkey1234567890".to_vec());
        PlatformCredentialManager::new(secret_key)
    }

    fn generate<F: Fn(&PlatformCredentialManager, &str)>(check: F) {
        let manager = setup_credential_manager();

        for _ in 0..10 {
            let tenant_id = manager.generate_tenant_id();
            check(&manager, &tenant_id);
        }
    }

    #[test]
    fn test_db_name() {
        const TENANT_ID_MAX_LENGTH: usize = 32;
        const DB_NAME_MAX_LENGTH: usize = 63;
        let allow_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

        generate(|manager, tenant_id| {
            let db_name = manager.generate_db_name("test", tenant_id);
            assert!(
                tenant_id.len() <= TENANT_ID_MAX_LENGTH,
                "tenant_id length is greater than 32: {}",
                tenant_id
            );
            assert!(
                db_name.len() <= DB_NAME_MAX_LENGTH,
                "db_name length is greater than 64: {}",
                db_name
            );
            assert!(
                allow_regex.is_match(&db_name),
                "db_name is not safe: {}",
                db_name
            );
        });
    }

    #[test]
    fn test_db_user_id() {
        const DB_USER_ID_MAX_LENGTH: usize = 63;
        let allow_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

        generate(|manager, tenant_id| {
            let db_user_id = tenant_id;
            assert!(
                db_user_id.len() <= DB_USER_ID_MAX_LENGTH,
                "db_user_id length is greater than 63: {}",
                db_user_id
            );
            assert!(
                allow_regex.is_match(&db_user_id),
                "db_user_id is not safe: {}",
                db_user_id
            );
        });
    }

    #[test]
    fn test_db_pass() {
        const DB_PASS_MAX_LENGTH: usize = 100;
        let allow_regex =
            Regex::new(r"^[a-zA-Z0-9!@#$%^&*()\-_=+\[\]{}:;,.?/]+$").unwrap();

        generate(|manager, tenant_id| {
            let db_pass = manager.generate_db_password(tenant_id);
            assert!(
                db_pass.len() <= DB_PASS_MAX_LENGTH,
                "db_pass length is greater than 100: {}",
                db_pass
            );
            assert!(
                allow_regex.is_match(&db_pass),
                "db_pass should contain symbols (base64 standard): {}",
                db_pass
            );
        });
    }
}
