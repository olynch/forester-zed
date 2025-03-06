use zed_extension_api::{self as zed, settings::LspSettings};

struct ForesterExtension {
    // ... state
}

impl zed::Extension for ForesterExtension {
    // ...
    fn new() -> Self
    where
        Self: Sized,
    {
        ForesterExtension {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command =
            format!("/home/o/g/jonsterling/ocaml-forester/_build/default/bin/forester/main.exe");
        Ok(zed::Command {
            command,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree("forester", worktree)
            .map(|lsp_settings| lsp_settings.initialization_options.clone());

        initialization_options
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("forester", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "forester": settings
        })))
    }
}

zed::register_extension!(ForesterExtension);
