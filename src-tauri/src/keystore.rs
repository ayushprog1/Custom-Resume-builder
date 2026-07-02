use keyring::Entry;

const SERVICE_NAME: &str = "tailor-resume";
const ACCOUNT_NAME: &str = "gemini-api-key";

pub fn save_api_key(key: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Keyring init error: {}", e))?;
    entry.set_password(key).map_err(|e| format!("Keyring save error: {}", e))?;
    Ok(())
}

pub fn load_api_key() -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Keyring init error: {}", e))?;
    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => Ok(String::new()),
        Err(e) => Err(format!("Keyring load error: {}", e)),
    }
}

pub fn delete_api_key() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Keyring init error: {}", e))?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Keyring delete error: {}", e)),
    }
}
