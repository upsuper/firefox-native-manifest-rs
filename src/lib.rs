use std::io;
use std::path::Path;
use thiserror::Error;

mod common;
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(unix, path = "unix.rs")]
mod inner;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
    #[error("name in the manifest is invalid")]
    InvalidName,
    #[error("cannot determine the home directory of the user")]
    UnknownHome,
}

/// Visibility of native manifests registered.
pub enum Visibility {
    Global,
    PerUser,
}

/// Register the specified manifest file for native messaging host with Firefox.
///
/// On Linux and macOS, it duplicates the content to the native directory,
/// with path canonicalized to absolute path.
///
/// On Windows, it adds the given path to corresponding key in the registry.
pub fn register_native_messaging<P>(vis: Visibility, manifest_path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    inner::register_native_messaging(vis, manifest_path.as_ref())
}

/// Register the specified manifest file for managed storage with Firefox.
///
/// On Linux and macOS, it duplicates the content to the native directory,
/// with path canonicalized to absolute path.
///
/// On Windows, it adds the given path to corresponding key in the registry.
pub fn register_managed_storage<P>(vis: Visibility, manifest_path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    inner::register_managed_storage(vis, manifest_path.as_ref())
}

/// Register the specified manifest file for PKCS #11 modules with Firefox.
///
/// On Linux and macOS, it duplicates the content to the native directory,
/// with path canonicalized to absolute path.
///
/// On Windows, it adds the given path to corresponding key in the registry.
pub fn register_pkcs11_modules<P>(vis: Visibility, manifest_path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    inner::register_pkcs11_modules(vis, manifest_path.as_ref())
}
