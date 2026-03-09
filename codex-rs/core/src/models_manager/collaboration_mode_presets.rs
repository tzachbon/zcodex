use codex_protocol::config_types::CollaborationModeMask;
use codex_protocol::config_types::ModeKind;
use codex_protocol::config_types::TUI_VISIBLE_COLLABORATION_MODES;
use codex_protocol::openai_models::ReasoningEffort;

const KNOWN_MODE_NAMES_PLACEHOLDER: &str = "{{KNOWN_MODE_NAMES}}";
const REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER: &str = "{{REQUEST_USER_INPUT_AVAILABILITY}}";

pub(super) fn builtin_collaboration_mode_presets() -> Vec<CollaborationModeMask> {
    vec![plan_preset(), default_preset()]
}

pub(super) fn builtin_internal_collaboration_mode_presets() -> Vec<CollaborationModeMask> {
    vec![conversation_plan_preset(), execute_preset()]
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
        developer_instructions: Some(Some(
            crate::gsd::mode_developer_instructions(ModeKind::Plan)
                .expect("missing GSD plan instructions")
                .to_string(),
        )),
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
    crate::gsd::mode_developer_instructions(ModeKind::Default)
        .expect("missing GSD default instructions")
        .replace(KNOWN_MODE_NAMES_PLACEHOLDER, &known_mode_names)
        .replace(
            REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER,
            &request_user_input_availability,
        )
}

fn conversation_plan_preset() -> CollaborationModeMask {
    CollaborationModeMask {
        name: ModeKind::ConversationPlan.display_name().to_string(),
        mode: Some(ModeKind::ConversationPlan),
        model: None,
        reasoning_effort: Some(Some(ReasoningEffort::Medium)),
        developer_instructions: Some(Some(
            crate::gsd::mode_developer_instructions(ModeKind::ConversationPlan)
                .expect("missing GSD conversation plan instructions")
                .to_string(),
        )),
    }
}

fn execute_preset() -> CollaborationModeMask {
    CollaborationModeMask {
        name: ModeKind::Execute.display_name().to_string(),
        mode: Some(ModeKind::Execute),
        model: None,
        reasoning_effort: Some(Some(ReasoningEffort::Medium)),
        developer_instructions: Some(Some(
            crate::gsd::mode_developer_instructions(ModeKind::Execute)
                .expect("missing GSD execute instructions")
                .to_string(),
        )),
    }
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

        assert!(!default_instructions.is_empty());
        assert!(!default_instructions.contains(KNOWN_MODE_NAMES_PLACEHOLDER));
        assert!(!default_instructions.contains(REQUEST_USER_INPUT_AVAILABILITY_PLACEHOLDER));
        assert!(default_instructions.contains("request_user_input"));
    }

    #[test]
    fn builtin_presets_exclude_hidden_modes() {
        let visible_modes: Vec<ModeKind> = builtin_collaboration_mode_presets()
            .into_iter()
            .filter_map(|preset| preset.mode)
            .collect();
        assert_eq!(visible_modes, vec![ModeKind::Plan, ModeKind::Default]);

        let hidden_modes: Vec<ModeKind> = builtin_internal_collaboration_mode_presets()
            .into_iter()
            .filter_map(|preset| preset.mode)
            .collect();
        assert_eq!(
            hidden_modes,
            vec![ModeKind::ConversationPlan, ModeKind::Execute]
        );
    }
}
