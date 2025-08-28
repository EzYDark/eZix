use winreg::RegKey;
use winreg::enums::*;

pub fn set_dword_policy(key_path: &str, name: &str, value: u32) -> Result<(), anyhow::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm.create_subkey(key_path)?;
    key.set_value(name, &value)?;
    Ok(())
}

pub fn set_string_policy(key_path: &str, name: &str, value: &str) -> Result<(), anyhow::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm.create_subkey(key_path)?;
    key.set_value(name, &value)?;
    Ok(())
}

pub fn set_string_list_policy(
    base_key_path: &str,
    subkey_name: &str,
    values: &[String],
) -> Result<(), anyhow::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let full_key_path = format!("{}\\{}", base_key_path, subkey_name);
    // Ensure the subkey is clean before writing new values
    hklm.delete_subkey_all(&full_key_path).ok();
    let (key, _) = hklm.create_subkey(&full_key_path)?;
    for (i, value) in values.iter().enumerate() {
        key.set_value((i + 1).to_string(), value)?;
    }
    Ok(())
}
