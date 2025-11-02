use zed_extension_api::{self as zed, LanguageServerId};

struct OcimaticExtension;

impl zed::Extension for OcimaticExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        OcimaticExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if let Some(command) = worktree.which("ocimatic") {
            Ok(zed::Command {
                command,
                args: vec!["lsp".to_string()],
                env: vec![],
            })
        } else {
            Err("ocimatic not found".to_string())
        }
    }
}

zed::register_extension!(OcimaticExtension);
