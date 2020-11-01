use crate::common::{read_manifest, validate_name, Manifest, ManifestType};
use crate::{Error, Result, Visibility};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use types::*;

pub(crate) fn register_native_messaging(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let mut manifest: NativeMessagingManifest = read_manifest(manifest_path)?;
    validate_name(&manifest.name)?;
    manifest.path = manifest_path
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .join(manifest.path)
        .canonicalize()?;
    write_manifest(vis, ManifestType::NativeMessagingHosts, &manifest)
}

pub(crate) fn register_managed_storage(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let manifest: ManagedStorageManifest<JsonValue> = read_manifest(manifest_path)?;
    write_manifest(vis, ManifestType::ManagedStorage, &manifest)
}

pub(crate) fn register_pkcs11_modules(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let mut manifest: Pkcs11Manifest = read_manifest(manifest_path)?;
    validate_name(&manifest.name)?;
    manifest.path = manifest_path
        .canonicalize()
        .unwrap()
        .join(manifest.path)
        .canonicalize()?;
    write_manifest(vis, ManifestType::Pkcs11Modules, &manifest)
}

fn write_manifest<T>(vis: Visibility, ty: ManifestType, manifest: &T) -> Result<()>
where
    T: Serialize + Manifest,
{
    // Construct the path to write manifest
    let mut dest = match vis {
        Visibility::Global => PathBuf::from(paths::BASE_DIR_GLOBAL),
        Visibility::PerUser => home::home_dir()
            .ok_or(Error::UnknownHome)?
            .join(paths::BASE_DIR_PER_USER),
    };
    dest.push(paths::path_name(ty));
    dest.push(format!("{}.json", manifest.name()));
    // Create necessary directories
    fs::create_dir_all(dest.parent().unwrap())?;
    // Write the manifest into the file
    let file = File::create(&dest)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &manifest)?;
    Ok(())
}

#[cfg(target_os = "macos")]
mod paths {
    use crate::common::ManifestType;

    pub(super) const BASE_DIR_GLOBAL: &str = "/Library/Application Support/Mozilla";
    pub(super) const BASE_DIR_PER_USER: &str = "Library/Application Support/Mozilla";

    pub(super) fn path_name(ty: ManifestType) -> &'static str {
        match ty {
            ManifestType::NativeMessagingHosts => "NativeMessagingHosts",
            ManifestType::ManagedStorage => "ManagedStorage",
            ManifestType::Pkcs11Modules => "PKCS11Modules",
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod paths {
    use crate::common::ManifestType;

    pub(super) const BASE_DIR_GLOBAL: &str = "/usr/lib/mozilla";
    pub(super) const BASE_DIR_PER_USER: &str = ".mozilla";

    pub(super) fn path_name(ty: ManifestType) -> &'static str {
        match ty {
            ManifestType::NativeMessagingHosts => "native-messaging-hosts",
            ManifestType::ManagedStorage => "managed-storage",
            ManifestType::Pkcs11Modules => "pkcs11-modules",
        }
    }
}
