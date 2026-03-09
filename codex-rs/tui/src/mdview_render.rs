use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use codex_ansi_escape::ansi_escape;
use ratatui::text::Line;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MarkdownSurface {
    AgentStream,
    ProposedPlanFinal,
    #[allow(dead_code)]
    ProposedPlanStream,
    ReasoningSummary,
    Tip,
    Explanation,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct MarkdownRenderRequest<'a> {
    pub(crate) source: &'a str,
    pub(crate) width: Option<usize>,
    pub(crate) surface: MarkdownSurface,
}

pub(crate) fn render_markdown_lines(request: MarkdownRenderRequest<'_>) -> Vec<Line<'static>> {
    render_markdown_lines_with_runner(request, run_mdview)
}

fn render_markdown_lines_with_runner<F>(
    request: MarkdownRenderRequest<'_>,
    run_mdview_fn: F,
) -> Vec<Line<'static>>
where
    F: FnOnce(&MarkdownRenderRequest<'_>) -> Result<String, String>,
{
    if let Some(raw_source) = strip_raw_tui_wrapper(request.source) {
        return native_render_lines(raw_source, request.width);
    }
    if !matches!(
        request.surface,
        MarkdownSurface::ProposedPlanFinal | MarkdownSurface::AgentStream
    ) {
        return native_render_lines(request.source, request.width);
    }

    match run_mdview_fn(&request) {
        Ok(output) => match parse_mdview_output(&output) {
            Ok(lines) => lines,
            Err(err) => {
                tracing::warn!("failed to parse mdview output: {err}");
                native_render_lines(request.source, request.width)
            }
        },
        Err(err) => {
            tracing::warn!("failed to render markdown with mdview: {err}");
            native_render_lines(request.source, request.width)
        }
    }
}

fn native_render_lines(source: &str, width: Option<usize>) -> Vec<Line<'static>> {
    crate::markdown_render::render_markdown_text_with_width(source, width).lines
}

fn run_mdview(request: &MarkdownRenderRequest<'_>) -> Result<String, String> {
    let mut command = Command::new("mdview");
    command.arg("-").arg("--paging=never");
    if matches!(request.surface, MarkdownSurface::AgentStream) {
        command.arg("--plain");
    }
    let force_color = match request.surface {
        MarkdownSurface::AgentStream
        | MarkdownSurface::ProposedPlanFinal
        | MarkdownSurface::ProposedPlanStream
        | MarkdownSurface::ReasoningSummary
        | MarkdownSurface::Tip
        | MarkdownSurface::Explanation => "1",
    };
    command.env("FORCE_COLOR", force_color);
    if let Some(width) = request.width {
        command.env("COLUMNS", width.max(1).to_string());
    }
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|err| format!("spawn mdview: {err}"))?;
    let Some(mut stdin) = child.stdin.take() else {
        return Err("mdview stdin unavailable".to_string());
    };
    stdin
        .write_all(request.source.as_bytes())
        .map_err(|err| format!("write mdview stdin: {err}"))?;
    drop(stdin);

    let output = child
        .wait_with_output()
        .map_err(|err| format!("wait for mdview: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let status = output
            .status
            .code()
            .map_or_else(|| "signal".to_string(), |code| code.to_string());
        let detail = if stderr.is_empty() {
            format!("exit status {status}")
        } else {
            format!("exit status {status}: {stderr}")
        };
        return Err(detail);
    }

    String::from_utf8(output.stdout).map_err(|err| format!("mdview stdout utf8: {err}"))
}

fn parse_mdview_output(output: &str) -> Result<Vec<Line<'static>>, String> {
    let mut lines = std::panic::catch_unwind(|| ansi_escape(output).lines)
        .map_err(|_| "ansi parser panicked".to_string())?;
    for line in &mut lines {
        if crate::render::line_utils::is_blank_line_spaces_only(line) {
            *line = Line::from("");
        }
    }
    while lines
        .last()
        .is_some_and(crate::render::line_utils::is_blank_line_spaces_only)
    {
        lines.pop();
    }
    Ok(lines)
}

fn strip_raw_tui_wrapper(source: &str) -> Option<&str> {
    let trimmed = source.trim();
    let inner = trimmed
        .strip_prefix("<raw_tui>")?
        .strip_suffix("</raw_tui>")?;
    Some(inner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn lines_to_strings(lines: &[Line<'static>]) -> Vec<String> {
        lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.clone())
                    .collect::<String>()
            })
            .collect()
    }

    #[test]
    fn mdview_output_is_parsed() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "# Title\n\n- one\n",
                width: Some(20),
                surface: MarkdownSurface::ProposedPlanFinal,
            },
            |_| Ok("\u{1b}[1mTitle\u{1b}[0m\nitem\n".to_string()),
        );

        assert_eq!(lines_to_strings(&lines), vec!["Title", "item"]);
        assert!(
            lines[0].spans[0]
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::BOLD)
        );
    }

    #[test]
    fn raw_wrapper_bypasses_mdview() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "<raw_tui>\n# Title\n</raw_tui>",
                width: None,
                surface: MarkdownSurface::ProposedPlanFinal,
            },
            |_| Err("should not call mdview".to_string()),
        );

        assert_eq!(lines_to_strings(&lines), vec!["# Title"]);
    }

    #[test]
    fn invalid_wrapper_is_plain_text() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "prefix <raw_tui>Title</raw_tui>",
                width: None,
                surface: MarkdownSurface::ProposedPlanFinal,
            },
            |_| Ok("fallback text\n".to_string()),
        );

        assert_eq!(lines_to_strings(&lines), vec!["fallback text"]);
    }

    #[test]
    fn mdview_failure_falls_back_to_native_renderer() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "1. item\n",
                width: None,
                surface: MarkdownSurface::ProposedPlanFinal,
            },
            |_| Err("boom".to_string()),
        );

        assert_eq!(lines_to_strings(&lines), vec!["1. item"]);
    }

    #[test]
    fn agent_stream_uses_mdview_renderer() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "# Title\n",
                width: None,
                surface: MarkdownSurface::AgentStream,
            },
            |_| Ok("rendered via mdview\n".to_string()),
        );

        assert_eq!(lines_to_strings(&lines), vec!["rendered via mdview"]);
    }

    #[test]
    fn empty_mdview_output_is_supported() {
        let lines = render_markdown_lines_with_runner(
            MarkdownRenderRequest {
                source: "plain text\n",
                width: None,
                surface: MarkdownSurface::ProposedPlanFinal,
            },
            |_| Ok(String::new()),
        );

        assert!(lines.is_empty());
    }
}
