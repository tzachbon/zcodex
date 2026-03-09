use codex_protocol::config_types::ModeKind;
use codex_protocol::user_input::ByteRange;
use codex_protocol::user_input::TextElement;

const GSD_CORE_PROMPT: &str = include_str!("../templates/gsd/core.md");
const GSD_DEFAULT_PROMPT: &str = include_str!("../templates/gsd/default.md");
const GSD_PLAN_PROMPT: &str = include_str!("../templates/gsd/plan.md");
const GSD_CONVERSATION_PLAN_PROMPT: &str = include_str!("../templates/gsd/conversation_plan.md");
const GSD_EXECUTE_PROMPT: &str = include_str!("../templates/collaboration_mode/execute.md");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GsdWorkflowCommand {
    WorkflowHelp,
    NewProject,
    NewMilestone,
    MapCodebase,
    DiscussPhase,
    PlanPhase,
    ExecutePhase,
    VerifyWork,
    AuditMilestone,
    CompleteMilestone,
    Progress,
    ResumeWork,
    PauseWork,
    Quick,
    QuickPlan,
    WorkflowSettings,
    WorkflowProfile,
    WorkflowUpdate,
    WorkflowHealth,
    DebugWorkflow,
    CleanupWorkflow,
    AddPhase,
    InsertPhase,
    RemovePhase,
    PhaseAssumptions,
    PlanMilestoneGaps,
    ResearchPhase,
    ValidatePhase,
    AddTodo,
    Todos,
    ReapplyPatches,
}

#[derive(Clone, Copy)]
struct WorkflowMeta {
    visible_name: &'static str,
    upstream_name: &'static str,
    objective: &'static str,
    next_step: Option<&'static str>,
    preferred_mode: Option<ModeKind>,
    planning_only: bool,
}

pub struct RenderedWorkflowPrompt {
    pub text: String,
    pub text_elements: Vec<TextElement>,
}

struct SanitizedArgs {
    text: String,
    boundary_map: Vec<(usize, usize)>,
}

impl GsdWorkflowCommand {
    fn meta(self) -> WorkflowMeta {
        match self {
            Self::WorkflowHelp => WorkflowMeta {
                visible_name: "workflow-help",
                upstream_name: "help",
                objective: "Explain the native Codex GSD command set, current workflow state, and the next sensible command.",
                next_step: None,
                preferred_mode: None,
                planning_only: false,
            },
            Self::NewProject => WorkflowMeta {
                visible_name: "new-project",
                upstream_name: "new-project",
                objective: "Initialize a new GSD project workflow in `.planning/` through questioning, requirements, roadmap, and state setup.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::NewMilestone => WorkflowMeta {
                visible_name: "new-milestone",
                upstream_name: "new-milestone",
                objective: "Start a new milestone cycle by updating project context, resetting the current milestone state, and producing the next roadmap slice.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::MapCodebase => WorkflowMeta {
                visible_name: "map-codebase",
                upstream_name: "map-codebase",
                objective: "Analyze the existing codebase and record stack, architecture, conventions, and concerns for later GSD planning.",
                next_step: Some("/new-project"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::DiscussPhase => WorkflowMeta {
                visible_name: "discuss-phase",
                upstream_name: "discuss-phase",
                objective: "Capture implementation preferences and decisions for a planned phase before research and detailed planning.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::PlanPhase => WorkflowMeta {
                visible_name: "plan-phase",
                upstream_name: "plan-phase",
                objective: "Research, plan, and verify the requested phase and record the resulting plan artifacts in `.planning/`.",
                next_step: Some("/execute-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::ExecutePhase => WorkflowMeta {
                visible_name: "execute-phase",
                upstream_name: "execute-phase",
                objective: "Execute the current planned phase with GSD discipline, preserving state transitions and verification checkpoints.",
                next_step: Some("/verify-work"),
                preferred_mode: Some(ModeKind::Execute),
                planning_only: false,
            },
            Self::VerifyWork => WorkflowMeta {
                visible_name: "verify-work",
                upstream_name: "verify-work",
                objective: "Run the GSD verification flow for the requested work and record the outcome in the planning state.",
                next_step: Some("/progress"),
                preferred_mode: Some(ModeKind::Execute),
                planning_only: false,
            },
            Self::AuditMilestone => WorkflowMeta {
                visible_name: "audit-milestone",
                upstream_name: "audit-milestone",
                objective: "Audit milestone completion against requirements, roadmap state, and verification artifacts.",
                next_step: Some("/complete-milestone"),
                preferred_mode: None,
                planning_only: false,
            },
            Self::CompleteMilestone => WorkflowMeta {
                visible_name: "complete-milestone",
                upstream_name: "complete-milestone",
                objective: "Close the active milestone, archive its state, and prepare the project for the next milestone cycle.",
                next_step: Some("/new-milestone"),
                preferred_mode: None,
                planning_only: false,
            },
            Self::Progress => WorkflowMeta {
                visible_name: "progress",
                upstream_name: "progress",
                objective: "Read `.planning/` state and summarize where the workflow stands plus the next high-value command.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::ResumeWork => WorkflowMeta {
                visible_name: "resume-work",
                upstream_name: "resume-work",
                objective: "Restore the current workflow context from `.planning/` and continue from the last saved state.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::PauseWork => WorkflowMeta {
                visible_name: "pause-work",
                upstream_name: "pause-work",
                objective: "Save a concise handoff into `.planning/STATE.md` so work can resume cleanly in a later session.",
                next_step: Some("/resume-work"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::Quick => WorkflowMeta {
                visible_name: "quick",
                upstream_name: "quick",
                objective: "Run the GSD quick workflow for a small task with the same quick options and state tracking conventions as upstream GSD.",
                next_step: Some("/progress"),
                preferred_mode: None,
                planning_only: false,
            },
            Self::QuickPlan => WorkflowMeta {
                visible_name: "quick-plan",
                upstream_name: "quick",
                objective: "Run the planning branch of the GSD quick workflow with the same quick options as upstream GSD.",
                next_step: Some("/quick"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::WorkflowSettings => WorkflowMeta {
                visible_name: "workflow-settings",
                upstream_name: "settings",
                objective: "Inspect or update GSD workflow settings stored under `.planning/` and the current Codex session.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::WorkflowProfile => WorkflowMeta {
                visible_name: "workflow-profile",
                upstream_name: "set-profile",
                objective: "Inspect or switch the active GSD workflow profile while keeping the existing planning state consistent.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::WorkflowUpdate => WorkflowMeta {
                visible_name: "workflow-update",
                upstream_name: "update",
                objective: "Explain the vendored GSD version in this repo and any local migration steps or compatibility notes.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::WorkflowHealth => WorkflowMeta {
                visible_name: "workflow-health",
                upstream_name: "health",
                objective: "Check the health of the current `.planning/` workflow state and point out missing or inconsistent artifacts.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::DebugWorkflow => WorkflowMeta {
                visible_name: "debug-workflow",
                upstream_name: "debug",
                objective: "Run the GSD debugging workflow for the current issue while preserving workflow state and assumptions.",
                next_step: None,
                preferred_mode: None,
                planning_only: false,
            },
            Self::CleanupWorkflow => WorkflowMeta {
                visible_name: "cleanup-workflow",
                upstream_name: "cleanup",
                objective: "Perform the GSD cleanup workflow and update `.planning/STATE.md` with the resulting cleanup status.",
                next_step: Some("/progress"),
                preferred_mode: None,
                planning_only: false,
            },
            Self::AddPhase => WorkflowMeta {
                visible_name: "add-phase",
                upstream_name: "add-phase",
                objective: "Append a new phase to the roadmap and keep numbering plus state references consistent.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::InsertPhase => WorkflowMeta {
                visible_name: "insert-phase",
                upstream_name: "insert-phase",
                objective: "Insert a new phase into the roadmap at the requested point and update phase numbering safely.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::RemovePhase => WorkflowMeta {
                visible_name: "remove-phase",
                upstream_name: "remove-phase",
                objective: "Remove a future phase from the roadmap and repair numbering plus downstream references.",
                next_step: Some("/progress"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::PhaseAssumptions => WorkflowMeta {
                visible_name: "phase-assumptions",
                upstream_name: "list-phase-assumptions",
                objective: "List the current assumptions for the requested phase before deeper planning or execution.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::PlanMilestoneGaps => WorkflowMeta {
                visible_name: "plan-milestone-gaps",
                upstream_name: "plan-milestone-gaps",
                objective: "Convert identified milestone gaps into new roadmap phases and planning work.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::ResearchPhase => WorkflowMeta {
                visible_name: "research-phase",
                upstream_name: "research-phase",
                objective: "Perform research-only GSD work for the requested phase and record the findings under `.planning/`.",
                next_step: Some("/plan-phase"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::ValidatePhase => WorkflowMeta {
                visible_name: "validate-phase",
                upstream_name: "validate-phase",
                objective: "Retroactively validate the requested phase against automated verification expectations and update the planning record.",
                next_step: Some("/progress"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::AddTodo => WorkflowMeta {
                visible_name: "add-todo",
                upstream_name: "add-todo",
                objective: "Capture a workflow todo item in `.planning/STATE.md` for later work.",
                next_step: Some("/todos"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::Todos => WorkflowMeta {
                visible_name: "todos",
                upstream_name: "check-todos",
                objective: "List pending workflow todos from `.planning/STATE.md` and suggest any obvious next command.",
                next_step: None,
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
            Self::ReapplyPatches => WorkflowMeta {
                visible_name: "reapply-patches",
                upstream_name: "reapply-patches",
                objective: "Recover locally maintained workflow modifications after a GSD update or migration.",
                next_step: Some("/workflow-health"),
                preferred_mode: Some(ModeKind::Plan),
                planning_only: true,
            },
        }
    }

    pub fn preferred_mode(self) -> Option<ModeKind> {
        self.meta().preferred_mode
    }

    pub fn description(self) -> &'static str {
        self.meta().objective
    }
}

pub fn global_developer_instructions() -> &'static str {
    GSD_CORE_PROMPT
}

pub fn mode_developer_instructions(mode: ModeKind) -> Option<&'static str> {
    match mode {
        ModeKind::Default => Some(GSD_DEFAULT_PROMPT),
        ModeKind::Plan => Some(GSD_PLAN_PROMPT),
        ModeKind::ConversationPlan => Some(GSD_CONVERSATION_PLAN_PROMPT),
        ModeKind::Execute => Some(GSD_EXECUTE_PROMPT),
        ModeKind::PairProgramming => None,
    }
}

pub fn render_workflow_prompt(command: GsdWorkflowCommand, args: &str) -> String {
    render_workflow_prompt_with_elements(command, args, &[]).text
}

pub fn render_workflow_prompt_with_elements(
    command: GsdWorkflowCommand,
    args: &str,
    text_elements: &[TextElement],
) -> RenderedWorkflowPrompt {
    let meta = command.meta();
    let sanitized_args = sanitize_args_for_prompt(args);
    let args_summary = if sanitized_args.text.is_empty() {
        "No inline arguments were provided.".to_string()
    } else {
        "Inline arguments: `__CODEX_GSD_INLINE_ARGS__`.".to_string()
    };
    let plan_only = if meta.planning_only {
        "Stop after the planning artifacts and state updates are ready. Do not execute implementation tasks in this command."
    } else {
        "Carry the workflow through its normal execution path."
    };
    let next_step = meta
        .next_step
        .map(|step| {
            format!("When the workflow finishes, recommend `{step}` if it matches the new state.")
        })
        .unwrap_or_default();

    let mut text = format!(
        r#"<gsd_native_command>
visible_name: /{visible_name}
upstream_name: /gsd:{upstream_name}
planning_root: .planning
</gsd_native_command>

<objective>
{objective}
</objective>

<context>
{args_summary}
Preserve Get Shit Done workflow conventions across `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, `.planning/research/`, and `.planning/quick/`.
Prefer explicit phase, milestone, and state transitions over ad hoc planning.
{plan_only}
</context>

<process>
Act as the native Codex implementation of the vendored GSD `{upstream_name}` workflow.
Use repository context before asking questions.
Keep the `.planning/` state coherent with the work you perform.
{next_step}
</process>
"#,
        visible_name = meta.visible_name,
        upstream_name = meta.upstream_name,
        objective = meta.objective,
        args_summary = args_summary,
        plan_only = plan_only,
        next_step = next_step,
    );

    let prompt_text_elements = if sanitized_args.text.is_empty() {
        Vec::new()
    } else {
        let marker = "`__CODEX_GSD_INLINE_ARGS__`";
        let args_start = text
            .find(marker)
            .map(|start| start + 1)
            .expect("workflow prompt must contain inline arg marker");
        text = text.replacen("__CODEX_GSD_INLINE_ARGS__", &sanitized_args.text, 1);
        text_elements
            .iter()
            .map(|element| {
                element.map_range(|range| ByteRange {
                    start: args_start
                        + map_sanitized_offset(&sanitized_args.boundary_map, range.start),
                    end: args_start + map_sanitized_offset(&sanitized_args.boundary_map, range.end),
                })
            })
            .collect()
    };

    RenderedWorkflowPrompt {
        text,
        text_elements: prompt_text_elements,
    }
}

fn sanitize_args_for_prompt(args: &str) -> SanitizedArgs {
    let trimmed = args.trim();
    let mut text = String::new();
    let mut boundary_map = Vec::with_capacity(trimmed.chars().count() + 2);
    boundary_map.push((0, 0));
    for (offset, ch) in trimmed.char_indices() {
        let sanitized = match ch {
            '`' => "\\`".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            ch if ch.is_control() => " ".to_string(),
            ch => ch.to_string(),
        };
        text.push_str(&sanitized);
        boundary_map.push((offset + ch.len_utf8(), text.len()));
    }
    SanitizedArgs { text, boundary_map }
}

fn map_sanitized_offset(boundary_map: &[(usize, usize)], offset: usize) -> usize {
    boundary_map
        .iter()
        .find(|(source, _)| *source == offset)
        .map(|(_, target)| *target)
        .unwrap_or(offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const ALL_COMMANDS: &[GsdWorkflowCommand] = &[
        GsdWorkflowCommand::WorkflowHelp,
        GsdWorkflowCommand::NewProject,
        GsdWorkflowCommand::NewMilestone,
        GsdWorkflowCommand::MapCodebase,
        GsdWorkflowCommand::DiscussPhase,
        GsdWorkflowCommand::PlanPhase,
        GsdWorkflowCommand::ExecutePhase,
        GsdWorkflowCommand::VerifyWork,
        GsdWorkflowCommand::AuditMilestone,
        GsdWorkflowCommand::CompleteMilestone,
        GsdWorkflowCommand::Progress,
        GsdWorkflowCommand::ResumeWork,
        GsdWorkflowCommand::PauseWork,
        GsdWorkflowCommand::Quick,
        GsdWorkflowCommand::QuickPlan,
        GsdWorkflowCommand::WorkflowSettings,
        GsdWorkflowCommand::WorkflowProfile,
        GsdWorkflowCommand::WorkflowUpdate,
        GsdWorkflowCommand::WorkflowHealth,
        GsdWorkflowCommand::DebugWorkflow,
        GsdWorkflowCommand::CleanupWorkflow,
        GsdWorkflowCommand::AddPhase,
        GsdWorkflowCommand::InsertPhase,
        GsdWorkflowCommand::RemovePhase,
        GsdWorkflowCommand::PhaseAssumptions,
        GsdWorkflowCommand::PlanMilestoneGaps,
        GsdWorkflowCommand::ResearchPhase,
        GsdWorkflowCommand::ValidatePhase,
        GsdWorkflowCommand::AddTodo,
        GsdWorkflowCommand::Todos,
        GsdWorkflowCommand::ReapplyPatches,
    ];

    #[test]
    fn all_commands_have_non_empty_descriptions() {
        for command in ALL_COMMANDS {
            assert!(
                !command.description().is_empty(),
                "{command:?} description should be set"
            );
        }
    }

    #[test]
    fn quick_plan_prompt_is_planning_only() {
        let prompt = render_workflow_prompt(GsdWorkflowCommand::QuickPlan, "--full");
        assert!(prompt.contains("visible_name: /quick-plan"));
        assert!(prompt.contains("upstream_name: /gsd:quick"));
        assert!(prompt.contains("Inline arguments: `--full`."));
        assert!(prompt.contains("Stop after the planning artifacts"));
    }

    #[test]
    fn execute_phase_prompt_is_not_planning_only() {
        let prompt = render_workflow_prompt(GsdWorkflowCommand::ExecutePhase, "2");
        assert!(prompt.contains("visible_name: /execute-phase"));
        assert!(prompt.contains("Carry the workflow through its normal execution path."));
        assert!(!prompt.contains("Stop after the planning artifacts"));
    }

    #[test]
    fn workflow_prompt_sanitizes_backticks_and_newlines() {
        let prompt =
            render_workflow_prompt(GsdWorkflowCommand::PlanPhase, "1 `quoted`\nnext\tline");
        assert!(prompt.contains("Inline arguments: `1 \\`quoted\\`\\nnext\\tline`."));
    }

    #[test]
    fn preferred_modes_match_intent() {
        assert_eq!(
            GsdWorkflowCommand::NewProject.preferred_mode(),
            Some(ModeKind::Plan)
        );
        assert_eq!(
            GsdWorkflowCommand::QuickPlan.preferred_mode(),
            Some(ModeKind::Plan)
        );
        assert_eq!(GsdWorkflowCommand::Quick.preferred_mode(), None);
    }

    #[test]
    fn developer_instruction_layers_are_present() {
        assert!(!global_developer_instructions().is_empty());
        assert!(mode_developer_instructions(ModeKind::Default).is_some());
        assert!(mode_developer_instructions(ModeKind::Plan).is_some());
        assert!(mode_developer_instructions(ModeKind::ConversationPlan).is_some());
        assert!(mode_developer_instructions(ModeKind::Execute).is_some());
    }

    #[test]
    fn workflow_prompt_preserves_inline_text_elements() {
        let rendered = render_workflow_prompt_with_elements(
            GsdWorkflowCommand::PlanPhase,
            "@build --full",
            &[TextElement::new((0..6).into(), Some("@build".to_string()))],
        );

        assert_eq!(rendered.text_elements.len(), 1);
        let range = rendered.text_elements[0].byte_range;
        assert_eq!(&rendered.text[range.start..range.end], "@build");
    }
}
