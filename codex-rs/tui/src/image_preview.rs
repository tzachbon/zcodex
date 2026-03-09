use std::io::ErrorKind;
use std::path::Path;
use std::process::Stdio;

use codex_core::config::Config;
use tokio::process::Command;

use crate::tui;

pub(crate) const IMAGE_PREVIEW_FAILURE_TITLE: &str = "Image preview unavailable.";
pub(crate) const IMAGE_PREVIEW_INSTALL_HINT: &str =
    "Install `timg` to enable terminal image preview.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ImagePreviewOutcome {
    Shown,
    SkippedDisabled,
    MissingBinary,
    MissingPath,
    Failed(String),
}

pub(crate) async fn preview_image(
    tui: &mut tui::Tui,
    config: &Config,
    path: &Path,
) -> ImagePreviewOutcome {
    if !config.tui_image_preview {
        return ImagePreviewOutcome::SkippedDisabled;
    }
    if !path.is_file() {
        return ImagePreviewOutcome::MissingPath;
    }

    let command = preview_command(config);
    let path = path.to_path_buf();
    tracing::debug!(
        command,
        path = %path.display(),
        "starting terminal image preview"
    );

    let status = tui
        .with_restored(tui::RestoreMode::Full, || async move {
            Command::new(command)
                .arg(&path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await
        })
        .await;

    match status {
        Ok(status) if status.success() => ImagePreviewOutcome::Shown,
        Ok(status) => ImagePreviewOutcome::Failed(match status.code() {
            Some(code) => format!("timg exited with status {code}"),
            None => "timg terminated before finishing".to_string(),
        }),
        Err(err) if err.kind() == ErrorKind::NotFound => ImagePreviewOutcome::MissingBinary,
        Err(err) => ImagePreviewOutcome::Failed(err.to_string()),
    }
}

fn preview_command(config: &Config) -> &str {
    config
        .tui_image_preview_command
        .as_deref()
        .filter(|command| !command.is_empty())
        .unwrap_or("timg")
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_core::config::ConfigBuilder;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn preview_command_uses_default_when_override_missing() {
        let config = ConfigBuilder::default().build().await.expect("config");
        assert_eq!(preview_command(&config), "timg");
    }

    #[tokio::test]
    async fn preview_command_uses_override_when_present() {
        let mut config = ConfigBuilder::default().build().await.expect("config");
        config.tui_image_preview_command = Some("custom-timg".to_string());
        assert_eq!(preview_command(&config), "custom-timg");
    }
}
