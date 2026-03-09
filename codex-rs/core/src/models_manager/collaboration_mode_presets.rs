use codex_protocol::config_types::CollaborationModeMask;
use codex_protocol::config_types::ModeKind;
use codex_protocol::config_types::TUI_VISIBLE_COLLABORATION_MODES;
use codex_protocol::openai_models::ReasoningEffort;

const COLLABORATION_MODE_PLAN: &str = include_str!("../../templates/collaboration_mode/plan.md");
const COLLABORATION_MODE_DEFAULT: &str =
    include_str!("../../templates/collaboration_mode/default.md");
const KNOWN_MODE_NAMES_PLACEHOLDER: &str = "{{KNOWN_MODE_NAMES}}";
const REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER: &str = "{{REQUEST_USER_INPUT_AVAILABILITY}}";

pub(super) fn builtin_collaboration_mode_presets() -> Vec<CollaborationModeMask> {
    vec![plan_preset(), default_preset()]
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_builtin_collaboration_mode_presets() -> Vec<CollaborationModeMask> {
    builtin_collaboration_mode_presets()
}

fn plan_preset() -> CollaborationModeMask {
    CollaborationModeMask {
        name: ModeKind::Plan.display_name().to_string(),
        mode: Some(ModeKind::Plan),
        model: None,
        reasoning_effort: Some(Some(ReasoningEffort::Medium)),
        developer_instructions: Some(Some(COLLABORATION_MODE_PLAN.to_string())),
    }
}

fn default_preset() -> CollaborationModeMask {
    CollaborationModeMask {
        name: ModeKind::Default.display_name().to_string(),
        mode: Some(ModeKind::Default),
        model: None,
        reasoning_effort: None,
        developer_instructions: Some(Some(default_mode_instructions())),
    }
}

fn default_mode_instructions() -> String {
    let known_mode_names = format_mode_names(&TUI_VISIBLE_COLLABORATION_MODES);
    let request_user_input_availability =
        request_user_input_availability_message(ModeKind::Default);
    COLLABORATION_MODE_DEFAULT
        .replace(KNOWN_MODE_NAMES_PLACEHOLDER, &known_mode_names)
        .replace(
            REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER,
            &request_user_input_availability,
        )
}

fn format_mode_names(modes: &[ModeKind]) -> String {
    let mode_names: Vec<&str> = modes.iter().map(|mode| mode.display_name()).collect();
    match mode_names.as_slice() {
        [] => "none".to_string(),
        [mode_name] => (*mode_name).to_string(),
        [first, second] => format!("{first} and {second}"),
        [..] => mode_names.join(", "),
    }
}

fn request_user_input_availability_message(mode: ModeKind) -> String {
    let mode_name = mode.display_name();
    if mode.allows_request_user_input() {
        format!("The `request_user_input` tool is available in {mode_name} mode.")
    } else {
        format!(
            "The `request_user_input` tool is unavailable in {mode_name} mode. If you call it while in {mode_name} mode, it will return an error."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn preset_names_use_mode_display_names() {
        assert_eq!(plan_preset().name, ModeKind::Plan.display_name());
        assert_eq!(default_preset().name, ModeKind::Default.display_name());
    }

    #[test]
    fn default_mode_instructions_replace_mode_names_placeholder() {
        let default_instructions = default_preset()
            .developer_instructions
            .expect("default preset should include instructions")
            .expect("default instructions should be set");

        assert!(!default_instructions.contains(KNOWN_MODE_NAMES_PLACEHOLDER));
        assert!(!default_instructions.contains(REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER));

        let known_mode_names = format_mode_names(&TUI_VISIBLE_COLLABORATION_MODES);
        let expected_snippet = format!("Known mode names are {known_mode_names}.");
        assert!(default_instructions.contains(&expected_snippet));

        let expected_availability_message =
            request_user_input_availability_message(ModeKind::Default);
        assert!(default_instructions.contains(&expected_availability_message));
    }

    #[test]
    fn plan_mode_instructions_document_raw_tui_wrapper() {
        let plan_instructions = plan_preset()
            .developer_instructions
            .expect("plan preset should include instructions")
            .expect("plan instructions should be set");

        assert!(plan_instructions.contains("<raw_tui>"));
        assert!(plan_instructions.contains("</raw_tui>"));
        assert!(plan_instructions.contains("<proposed_plan>"));
    }

    #[test]
    fn default_mode_instructions_document_direct_answers_for_simple_formatting() {
        let default_instructions = default_preset()
            .developer_instructions
            .expect("default preset should include instructions")
            .expect("default instructions should be set");

        assert!(default_instructions.contains("can you output markdown?"));
        assert!(default_instructions.contains("show a mermaid example"));
        assert!(default_instructions.contains("Do not mention internal mode changes"));
    }

    #[test]
    fn plan_mode_instructions_allow_direct_answers_for_simple_formatting() {
        let plan_instructions = plan_preset()
            .developer_instructions
            .expect("plan preset should include instructions")
            .expect("plan instructions should be set");

        assert!(plan_instructions.contains("can you output markdown?"));
        assert!(plan_instructions.contains("show a mermaid example"));
        assert!(
            plan_instructions
                .contains("Do not use `request_user_input` for simple formatting demos")
        );
    }

    #[test]
    fn plan_mode_instructions_recommend_mermaid_for_structural_changes() {
        let plan_instructions = plan_preset()
            .developer_instructions
            .expect("plan preset should include instructions")
            .expect("plan instructions should be set");

        assert!(plan_instructions.contains("Use Mermaid diagrams"));
        assert!(plan_instructions.contains("flowchart"));
        assert!(plan_instructions.contains("sequenceDiagram"));
        assert!(plan_instructions.contains("stateDiagram-v2"));
        assert!(plan_instructions.contains("show the source and destination explicitly"));
    }
}
