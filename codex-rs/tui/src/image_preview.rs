use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

use codex_core::config::Config;
use crossterm::terminal;
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
    let geometry = preview_geometry();
    let path = path.to_path_buf();
    tracing::debug!(
        command,
        geometry = geometry.as_deref().unwrap_or("<auto>"),
        path = %path.display(),
        "starting terminal image preview"
    );

    let status: std::io::Result<std::process::ExitStatus> = tui
        .with_restored(tui::RestoreMode::Full, || async move {
            let mut process = Command::new(command);
            if let Some(geometry) = geometry.as_deref() {
                process.arg(geometry);
            }
            let status = process
                .arg(&path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await?;
            if status.success() && preview_requires_acknowledgement(command) {
                let _ = wait_for_preview_acknowledgement().await;
            }
            Ok(status)
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

fn preview_requires_acknowledgement(command: &str) -> bool {
    Path::new(command)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("timg")
}

fn preview_geometry() -> Option<String> {
    terminal::size()
        .ok()
        .and_then(|(width, height)| format_preview_geometry(width, height))
}

fn format_preview_geometry(width: u16, height: u16) -> Option<String> {
    (width > 0 && height > 0).then(|| format!("-g{width}x{height}"))
}

async fn wait_for_preview_acknowledgement() -> std::io::Result<()> {
    tokio::task::spawn_blocking(|| {
        let mut stderr = std::io::stderr();
        stderr.write_all(b"\nPress Enter to return...")?;
        stderr.flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        stderr.write_all(b"\r\x1b[2K")?;
        stderr.flush()
    })
    .await
    .map_err(std::io::Error::other)?
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

    #[test]
    fn preview_geometry_formats_terminal_size() {
        assert_eq!(Some("-g80x24".to_string()), format_preview_geometry(80, 24));
        assert_eq!(None, format_preview_geometry(0, 24));
        assert_eq!(None, format_preview_geometry(80, 0));
    }

    #[test]
    fn preview_requires_acknowledgement_for_timg_only() {
        assert!(preview_requires_acknowledgement("timg"));
        assert!(preview_requires_acknowledgement("/usr/bin/timg"));
        assert!(!preview_requires_acknowledgement("custom-preview"));
    }
}
