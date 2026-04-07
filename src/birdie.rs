use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct BirdieExtension {}

impl BirdieExtension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("gleam") {
            Ok(path)
        } else {
            Err("cannot find Gleam executable".into())
        }
    }
}

impl zed::Extension for BirdieExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![
                "run".to_string(),
                "-m".to_string(),
                "birdie".to_string(),
                "lsp".to_string(),
            ],
            env: vec![("ERL_FLAGS".to_string(), "-noinput -noshell".to_string())],
        })
    }
}

zed::register_extension!(BirdieExtension);
