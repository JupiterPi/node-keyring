#![deny(clippy::all)]

use napi_derive::napi;
use keyring::Entry;

/// Set a password in the keyring for a given service and account.
#[napi]
pub fn set_password(service: String, account: String, password: String) -> napi::Result<()> {
  let entry = Entry::new(&service, &account)
    .map_err(|e| napi::Error::from_reason(format!("Error trying to get keyring entry: {}", e)))?;
  entry.set_password(&password)
    .map_err(|e| napi::Error::from_reason(format!("Error trying to set keyring entry password: {}", e)))?;
  Ok(())
}

/// Get a password from the keyring for a given service and account.
#[napi]
pub fn get_password(service: String, account: String) -> napi::Result<String> {
  let entry = Entry::new(&service, &account)
    .map_err(|e| napi::Error::from_reason(format!("Error trying to get keyring entry: {}", e)))?;
  entry.get_password()
    .map_err(|e| napi::Error::from_reason(format!("Error trying to get keyring entry password: {}", e)))
}

/// Delete a password from the keyring for a given service and account.
#[napi]
pub fn delete_password(service: String, account: String) -> napi::Result<()> {
  let entry = Entry::new(&service, &account)
    .map_err(|e| napi::Error::from_reason(format!("Error trying to get keyring entry: {}", e)))?;
  entry.delete_credential()
    .map_err(|e| napi::Error::from_reason(format!("Error trying to delete keyring entry credential: {}", e)))?;
  Ok(())
}