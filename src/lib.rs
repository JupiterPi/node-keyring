#![deny(clippy::all)]

use keyring::Entry;
use napi_derive::napi;

/// Set a password in the keyring for a given service and account.
#[napi]
pub fn set_password(service: String, account: String, password: String) -> napi::Result<()> {
  (|| {
    let entry = Entry::new(&service, &account)?;
    entry.set_password(&password)?;
    Ok(())
  })()
  .map_err(|e: keyring::Error| {
    napi::Error::from_reason(format!("Error trying to set keyring entry: {e}"))
  })
}

/// Get a password from the keyring for a given service and account.
#[napi]
pub fn get_password(service: String, account: String) -> napi::Result<Option<String>> {
  (|| {
    let entry = Entry::new(&service, &account)?;
    if let Ok(password) = entry.get_password() {
      Ok(Some(password))
    } else {
      Ok(None)
    }
  })()
  .map_err(|e: keyring::Error| {
    napi::Error::from_reason(format!("Error trying to get keyring entry: {e}"))
  })
}

/// Delete a password from the keyring for a given service and account.
#[napi]
pub fn delete_password(service: String, account: String) -> napi::Result<()> {
  (|| {
    let entry = Entry::new(&service, &account)?;
    entry.delete_credential()?;
    Ok(())
  })()
  .map_err(|e: keyring::Error| {
    napi::Error::from_reason(format!("Error trying to delete keyring entry: {e}"))
  })
}
