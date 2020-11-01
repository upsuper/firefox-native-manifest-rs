use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Manifest object for [native messaging][native-messaging]
/// by which an extension can communicate with a native app installed on the device.
///
/// [native-messaging]: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging
#[derive(Serialize, Deserialize, Debug)]
pub struct NativeMessagingManifest {
    /// Name of the native application.
    ///
    /// This must match the name passed into `runtime.connectNative()`
    /// or `runtime.sendNativeMessage()` by the extension.
    ///
    /// On MacOS and Linux, it must also match the native messaging manifest's filename
    /// (excluding the .json extension).
    ///
    /// On Windows, it must match the name of the registry key you create,
    /// that contains the location of the native messaging manifest.
    ///
    /// The name must match the following regular expression: `"^\w+(\.\w+)*$"`.
    /// This means that it may only contain
    /// (lowercase or uppercase) alphanumeric characters, underscores, and dots.
    /// It may not start or end with a dot, and a dot cannot be followed by another dot.
    pub name: String,

    /// Description of the native application.
    pub description: String,

    /// Path to the native application.
    ///
    /// On Windows, this may be relative to the manifest itself.
    /// On MacOS and Linux it must be absolute.
    pub path: PathBuf,

    /// Describes the method used to connect the extension with the app.
    #[serde(rename = "type")]
    pub ty: NativeMessagingType,

    /// An array of Add-on ID values.
    ///
    /// Each value represents an extension
    /// which is allowed to communicate with the native application.
    ///
    /// Note that this means you will probably want to include the `browser_specific_settings` key
    /// in your extension's `manifest.json` file, so you can set an explicit ID during development.
    pub allowed_extensions: Vec<String>,
}

/// Method used to connect the extension with the app.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NativeMessagingType {
    /// Messages are received by the app using standard input (`stdin`)
    /// and sent using standard output (`stdout`).
    Stdio,
}

/// Define read-only data that
/// an extension can access using the [`storage.managed`][storage-managed] API.
///
/// [storage-managed]: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/storage/managed
#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedStorageManifest<T> {
    /// The ID of the extension that can access this storage,
    /// given as the ID you've specified in the extension's `applications` key.
    pub name: String,

    /// Human readable description, ignored by Firefox.
    pub description: String,

    /// This must be "storage".
    #[serde(rename = "type")]
    pub ty: ManagedStorageType,

    /// A JSON object that may contain any valid JSON values,
    /// including strings, numbers, booleans, arrays, or objects.
    /// This will become the data in the `browser.storage.managed` storage area.
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ManagedStorageType {
    Storage,
}

/// Enable an extension to use the [`pkcs11`][pkcs11] API
/// to enumerate PKCS #11 security modules and install them in Firefox.
///
/// [pkcs11]: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/pkcs11
#[derive(Serialize, Deserialize, Debug)]
pub struct Pkcs11Manifest {
    /// Name of the PKCS #11 module.
    ///
    /// This must match the name used in the `pkcs11` API.
    ///
    /// On MacOS and Linux, it must also match the manifest's filename (excluding the extension).
    ///
    /// On Windows, it must match the name of the registry key you create,
    /// which contains the location of the manifest.
    ///
    /// The name must match the following regular expression: `"^\w+(\.\w+)*$"`.
    /// This means that it may only contain lowercase alphanumeric characters, underscores and dots.
    /// It may not start or end with a dot, and a dot cannot be followed by another dot.
    pub name: String,

    /// Description of the module.
    //
    /// This is used to set the friendly name for the module in the browser's UI
    /// (for example, the "Security Devices" dialog in Firefox).
    pub description: String,

    /// Path to the module.
    ///
    /// On Windows, this may be relative to the manifest itself.
    /// On MacOS and Linux it must be absolute.
    pub path: PathBuf,

    /// This must be "pkcs11".
    #[serde(rename = "type")]
    pub ty: Pkcs11Type,

    /// An array of Add-on ID values. Each value represents an extension which is allowed to interact with the module.
    ///
    /// Note: This means you will probably want to
    /// include the `applications` key in your extension's `manifest.json` file,
    /// so you can set an explicit ID during development.
    pub allowed_extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Pkcs11Type {
    Pkcs11,
}
