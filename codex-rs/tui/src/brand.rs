use crate::wrapping::RtOptions;
use crate::wrapping::word_wrap_lines;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;

pub(crate) fn compact_title_spans(version: &str) -> Vec<Span<'static>> {
    vec![
        ">_ ".dim(),
        "Z".light_blue().bold(),
        "CODEX".bold(),
        " ".dim(),
        format!("(v{version})").dim(),
    ]
}

pub(crate) fn startup_hero_lines(version: &str, inner_width: usize) -> Vec<Line<'static>> {
    let banner = if fits(inner_width, &large_banner_lines()) {
        large_banner_lines()
    } else if fits(inner_width, &medium_banner_lines()) {
        medium_banner_lines()
    } else {
        vec![Line::from(compact_title_spans(version))]
    };

    let subtitle = word_wrap_lines(
        [Line::from(vec![
            "OpenAI coding agent".dim(),
            " · ".dim(),
            format!("v{version}").dim(),
        ])],
        RtOptions::new(inner_width.max(1)),
    );

    let mut lines = banner;
    lines.push(Line::from(""));
    lines.extend(subtitle);
    lines.push(Line::from(""));
    lines
}

fn fits(inner_width: usize, lines: &[Line<'static>]) -> bool {
    lines.iter().map(Line::width).max().unwrap_or(0) <= inner_width
}

fn large_banner_lines() -> Vec<Line<'static>> {
    vec![
        hero_line(
            "███████",
            " ██████",
            " █████ ",
            "██████ ",
            "███████",
            "██   ██",
        ),
        hero_line("     ██", "██", "██   ██", "██   ██", "██", " ██ ██ "),
        hero_line("   ███ ", "██", "██   ██", "██   ██", "█████", "  ███  "),
        hero_line("  ██   ", "██", "██   ██", "██   ██", "██", " ██ ██ "),
        hero_line(
            "███████",
            " ██████",
            " █████ ",
            "██████ ",
            "███████",
            "██   ██",
        ),
    ]
}

fn medium_banner_lines() -> Vec<Line<'static>> {
    vec![
        hero_line("█████", "████", "███", "████", "█████", "█ █"),
        hero_line("   ██", "█", "█ █", "█ █", "██", " █ "),
        hero_line(" ██  ", "█", "█ █", "█ █", "███", "█ █"),
        hero_line("█████", "████", "███", "████", "█████", "█ █"),
    ]
}

fn hero_line(
    z: &'static str,
    c: &'static str,
    o: &'static str,
    d: &'static str,
    e: &'static str,
    x: &'static str,
) -> Line<'static> {
    Line::from(vec![
        z.light_blue().bold(),
        "  ".into(),
        c.bold(),
        "  ".into(),
        o.bold(),
        "  ".into(),
        d.bold(),
        "  ".into(),
        e.bold(),
        "  ".into(),
        x.bold(),
    ])
}
