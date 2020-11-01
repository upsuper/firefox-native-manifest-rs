use super::{Error, Result, Visibility};
use crate::common::{read_manifest, validate_name, Manifest, ManifestType};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use types::*;
use winreg::enums::*;
use winreg::RegKey;

pub(crate) fn register_native_messaging(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let manifest: NativeMessagingManifest = read_manifest(manifest_path)?;
    validate_name(&manifest.name);
    write_registry(
        vis,
        ManifestType::NativeMessagingHosts,
        &manifest.name,
        manifest_path,
    )
}

pub(crate) fn register_managed_storage(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let manifest: ManagedStorageManifest<JsonValue> = read_manifest(manifest_path)?;
    write_registry(
        vis,
        ManifestType::ManagedStorage,
        &manifest.name,
        manifest_path,
    )
}

pub(crate) fn register_pkcs11_modules(vis: Visibility, manifest_path: &Path) -> Result<()> {
    let mut manifest: Pkcs11Manifest = read_manifest(manifest_path)?;
    validate_name(&manifest.name)?;
    write_registry(
        vis,
        ManifestType::Pkcs11Modules,
        &manifest.name,
        manifest_path,
    )
}

fn write_registry(
    vis: Visibility,
    ty: ManifestType,
    name: &str,
    manifest_path: &Path,
) -> Result<()> {
    let manifest_path = manifest_path.canonicalize().unwrap();
    let root_key = match vis {
        Visibility::Global => RegKey::predef(HKEY_LOCAL_MACHINE),
        Visibility::PerUser => RegKey::predef(HKEY_CURRENT_USER),
    };
    let path_name = match ty {
        ManifestType::NativeMessagingHosts => "NativeMessagingHosts",
        ManifestType::ManagedStorage => "ManagedStorage",
        ManifestType::Pkcs11Modules => "PKCS11Modules",
    };
    let key_path = format!(r#"SOFTWARE\Mozilla\{}\{}"#, path_name, name);
    let (key, _) = root_key.create_subkey(key_path)?;
    key.set_value("", manifest_path.as_os_str())?;
    Ok(())
}
