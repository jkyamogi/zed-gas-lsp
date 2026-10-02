use std::fs;
use zed_extension_api::{self as zed, serde_json::json, LanguageServerId, Result};

const TYPES_PACKAGE: &str = "@types/google-apps-script";

struct GoogleAppsScriptExtension;

impl zed::Extension for GoogleAppsScriptExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // ensure GAS type definitions are installed in the extensions's directory
        ensure_types_installed(language_server_id)?;

        // intentionally return an error for now so Zed falls back
        // to the built-in JavaScript/TypeScript language servers instead of starting our own
        Err("Use the built-in TypeScript / vtsls language server".into())

        // TO-DO LATER --> install a thin wrapper or force a specific TS LS.
    }

    // give TypeScript servers extra config so they prefer the GAS types
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        // enforce idempotency -> ensure types are present
        let _ = ensure_types_installed(language_server_id);

        Ok(Some(json!({
            "javascript": {
                "suggest": {
                    "enabled": true
                },
                "preferences": {
                    // leave empty --> encourage server to use ambient types
                }
            },
            "typescript": {
                "preferences": {
                    // leave empty --> encourage server to use ambient types
                }
            }
        })))
    }
}

/// downloads and installs @types/google-apps-script into extension's dir if missing or outdated
fn ensure_types_installed(language_server_id: &LanguageServerId) -> Result<()> {
    let latest = zed::npm_package_latest_version(TYPES_PACKAGE)?;

    let installed = zed::npm_package_installed_version(TYPES_PACKAGE)?;

    if installed.as_ref() == Some(&latest) {
        return Ok(());
    }

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    zed::npm_install_package(TYPES_PACKAGE, &latest)?;

    let _ = fs::write(
        "jsconfig.json",
        r#"{
            "compilerOptions": {
                "allowJs": true,
                "checkJs": true,
                "noEmit": true,
                "target": "ES2020",
                "types": ["google-apps-script"]
            }
        }"#,
    );

    Ok(())
}

zed::register_extension!(GoogleAppsScriptExtension);
