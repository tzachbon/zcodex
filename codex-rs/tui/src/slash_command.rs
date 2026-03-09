use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Plan,
    QuickPlan,
    NewProject,
    NewMilestone,
    MapCodebase,
    DiscussPhase,
    PlanPhase,
    ExecutePhase,
    VerifyWork,
    Quick,
    Progress,
    ResumeWork,
    PauseWork,
    WorkflowSettings,
    WorkflowProfile,
    WorkflowHelp,
    Model,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-elevated-sandbox")]
    ElevateSandbox,
    Experimental,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Collab,
    Agent,
    AddPhase,
    InsertPhase,
    RemovePhase,
    PhaseAssumptions,
    PlanMilestoneGaps,
    ResearchPhase,
    ValidatePhase,
    WorkflowUpdate,
    WorkflowHealth,
    DebugWorkflow,
    CleanupWorkflow,
    AddTodo,
    Todos,
    AuditMilestone,
    CompleteMilestone,
    ReapplyPatches,
    // Undo,
    Diff,
    Mention,
    Status,
    DebugConfig,
    Statusline,
    Mcp,
    Apps,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    Personality,
    TestApproval,
}

impl SlashCommand {
    pub(crate) fn is_gsd_workflow(self) -> bool {
        matches!(
            self,
            SlashCommand::QuickPlan
                | SlashCommand::NewProject
                | SlashCommand::NewMilestone
                | SlashCommand::MapCodebase
                | SlashCommand::DiscussPhase
                | SlashCommand::PlanPhase
                | SlashCommand::ExecutePhase
                | SlashCommand::VerifyWork
                | SlashCommand::Quick
                | SlashCommand::Progress
                | SlashCommand::ResumeWork
                | SlashCommand::PauseWork
                | SlashCommand::WorkflowSettings
                | SlashCommand::WorkflowProfile
                | SlashCommand::WorkflowHelp
                | SlashCommand::AddPhase
                | SlashCommand::InsertPhase
                | SlashCommand::RemovePhase
                | SlashCommand::PhaseAssumptions
                | SlashCommand::PlanMilestoneGaps
                | SlashCommand::ResearchPhase
                | SlashCommand::ValidatePhase
                | SlashCommand::WorkflowUpdate
                | SlashCommand::WorkflowHealth
                | SlashCommand::DebugWorkflow
                | SlashCommand::CleanupWorkflow
                | SlashCommand::AddTodo
                | SlashCommand::Todos
                | SlashCommand::AuditMilestone
                | SlashCommand::CompleteMilestone
                | SlashCommand::ReapplyPatches
        )
    }

    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Plan => "open the GSD planning hub",
            SlashCommand::QuickPlan => "run GSD quick planning only",
            SlashCommand::NewProject => "start a new GSD project",
            SlashCommand::NewMilestone => "start a new GSD milestone",
            SlashCommand::MapCodebase => "map the current codebase for GSD",
            SlashCommand::DiscussPhase => "capture implementation preferences for a phase",
            SlashCommand::PlanPhase => "research and plan a GSD phase",
            SlashCommand::ExecutePhase => "execute the current GSD phase",
            SlashCommand::VerifyWork => "verify the current GSD work",
            SlashCommand::Quick => "run the GSD quick workflow",
            SlashCommand::Progress => "show GSD workflow progress",
            SlashCommand::ResumeWork => "resume GSD workflow context",
            SlashCommand::PauseWork => "save a GSD workflow handoff",
            SlashCommand::WorkflowSettings => "inspect or change GSD workflow settings",
            SlashCommand::WorkflowProfile => "switch the active GSD workflow profile",
            SlashCommand::WorkflowHelp => "show GSD workflow help",
            SlashCommand::Feedback => "send logs to maintainers",
            SlashCommand::New => "start a new chat during a conversation",
            SlashCommand::Init => "create an AGENTS.md file with instructions for Codex",
            SlashCommand::Compact => "summarize conversation to prevent hitting the context limit",
            SlashCommand::Review => "review my current changes and find issues",
            SlashCommand::Rename => "rename the current thread",
            SlashCommand::Resume => "resume a saved chat",
            SlashCommand::Fork => "fork the current chat",
            // SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit | SlashCommand::Exit => "exit Codex",
            SlashCommand::Diff => "show git diff (including untracked files)",
            SlashCommand::Mention => "mention a file",
            SlashCommand::Skills => "use skills to improve how Codex performs specific tasks",
            SlashCommand::Status => "show current session configuration and token usage",
            SlashCommand::DebugConfig => "show config layers and requirement sources for debugging",
            SlashCommand::Statusline => "configure which items appear in the status line",
            SlashCommand::Ps => "list background terminals",
            SlashCommand::Model => "choose what model and reasoning effort to use",
            SlashCommand::Personality => "choose a communication style for Codex",
            SlashCommand::Collab => "change collaboration mode (experimental)",
            SlashCommand::Agent => "switch the active agent thread",
            SlashCommand::AddPhase => "append a phase to the GSD roadmap",
            SlashCommand::InsertPhase => "insert a phase into the GSD roadmap",
            SlashCommand::RemovePhase => "remove a future GSD phase",
            SlashCommand::PhaseAssumptions => "list assumptions for a GSD phase",
            SlashCommand::PlanMilestoneGaps => "turn milestone gaps into planned work",
            SlashCommand::ResearchPhase => "run research only for a GSD phase",
            SlashCommand::ValidatePhase => "retroactively validate a GSD phase",
            SlashCommand::WorkflowUpdate => "show vendored GSD update information",
            SlashCommand::WorkflowHealth => "check GSD planning state health",
            SlashCommand::DebugWorkflow => "run the GSD debugging workflow",
            SlashCommand::CleanupWorkflow => "run the GSD cleanup workflow",
            SlashCommand::AddTodo => "add a GSD workflow todo",
            SlashCommand::Todos => "show GSD workflow todos",
            SlashCommand::AuditMilestone => "audit the current GSD milestone",
            SlashCommand::CompleteMilestone => "complete the current GSD milestone",
            SlashCommand::ReapplyPatches => "reapply local GSD workflow patches",
            SlashCommand::Approvals => "choose what Codex can do without approval",
            SlashCommand::Permissions => "choose what Codex is allowed to do",
            SlashCommand::ElevateSandbox => "set up elevated agent sandbox",
            SlashCommand::Experimental => "toggle experimental features",
            SlashCommand::Mcp => "list configured MCP tools",
            SlashCommand::Apps => "manage apps",
            SlashCommand::Logout => "log out of Codex",
            SlashCommand::Rollout => "print the rollout file path",
            SlashCommand::TestApproval => "test approval request",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    pub fn aliases(self) -> &'static [&'static str] {
        match self {
            SlashCommand::NewProject => &["gsd:new-project"],
            SlashCommand::NewMilestone => &["gsd:new-milestone"],
            SlashCommand::MapCodebase => &["gsd:map-codebase"],
            SlashCommand::DiscussPhase => &["gsd:discuss-phase"],
            SlashCommand::PlanPhase => &["gsd:plan-phase"],
            SlashCommand::ExecutePhase => &["gsd:execute-phase"],
            SlashCommand::VerifyWork => &["gsd:verify-work"],
            SlashCommand::Quick => &["gsd:quick"],
            SlashCommand::Progress => &["gsd:progress"],
            SlashCommand::ResumeWork => &["gsd:resume-work"],
            SlashCommand::PauseWork => &["gsd:pause-work"],
            SlashCommand::WorkflowSettings => &["gsd:settings"],
            SlashCommand::WorkflowProfile => &["gsd:set-profile"],
            SlashCommand::WorkflowHelp => &["gsd:help"],
            SlashCommand::AddPhase => &["gsd:add-phase"],
            SlashCommand::InsertPhase => &["gsd:insert-phase"],
            SlashCommand::RemovePhase => &["gsd:remove-phase"],
            SlashCommand::PhaseAssumptions => &["gsd:list-phase-assumptions"],
            SlashCommand::PlanMilestoneGaps => &["gsd:plan-milestone-gaps"],
            SlashCommand::ResearchPhase => &["gsd:research-phase"],
            SlashCommand::ValidatePhase => &["gsd:validate-phase"],
            SlashCommand::WorkflowUpdate => &["gsd:update"],
            SlashCommand::WorkflowHealth => &["gsd:health"],
            SlashCommand::DebugWorkflow => &["gsd:debug"],
            SlashCommand::CleanupWorkflow => &["gsd:cleanup"],
            SlashCommand::AddTodo => &["gsd:add-todo"],
            SlashCommand::Todos => &["gsd:check-todos"],
            SlashCommand::AuditMilestone => &["gsd:audit-milestone"],
            SlashCommand::CompleteMilestone => &["gsd:complete-milestone"],
            SlashCommand::ReapplyPatches => &["gsd:reapply-patches"],
            _ => &[],
        }
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::QuickPlan
                | SlashCommand::NewProject
                | SlashCommand::NewMilestone
                | SlashCommand::MapCodebase
                | SlashCommand::DiscussPhase
                | SlashCommand::PlanPhase
                | SlashCommand::ExecutePhase
                | SlashCommand::VerifyWork
                | SlashCommand::Quick
                | SlashCommand::Progress
                | SlashCommand::ResumeWork
                | SlashCommand::PauseWork
                | SlashCommand::WorkflowSettings
                | SlashCommand::WorkflowProfile
                | SlashCommand::WorkflowHelp
                | SlashCommand::AddPhase
                | SlashCommand::InsertPhase
                | SlashCommand::RemovePhase
                | SlashCommand::PhaseAssumptions
                | SlashCommand::PlanMilestoneGaps
                | SlashCommand::ResearchPhase
                | SlashCommand::ValidatePhase
                | SlashCommand::WorkflowUpdate
                | SlashCommand::WorkflowHealth
                | SlashCommand::DebugWorkflow
                | SlashCommand::CleanupWorkflow
                | SlashCommand::AddTodo
                | SlashCommand::Todos
                | SlashCommand::AuditMilestone
                | SlashCommand::CompleteMilestone
                | SlashCommand::ReapplyPatches
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::QuickPlan
            | SlashCommand::NewProject
            | SlashCommand::NewMilestone
            | SlashCommand::MapCodebase
            | SlashCommand::DiscussPhase
            | SlashCommand::PlanPhase
            | SlashCommand::ExecutePhase
            | SlashCommand::VerifyWork
            | SlashCommand::Quick
            | SlashCommand::Progress
            | SlashCommand::ResumeWork
            | SlashCommand::PauseWork
            | SlashCommand::WorkflowSettings
            | SlashCommand::WorkflowProfile
            | SlashCommand::WorkflowHelp
            | SlashCommand::AddPhase
            | SlashCommand::InsertPhase
            | SlashCommand::RemovePhase
            | SlashCommand::PhaseAssumptions
            | SlashCommand::PlanMilestoneGaps
            | SlashCommand::ResearchPhase
            | SlashCommand::ValidatePhase
            | SlashCommand::WorkflowUpdate
            | SlashCommand::WorkflowHealth
            | SlashCommand::DebugWorkflow
            | SlashCommand::CleanupWorkflow
            | SlashCommand::AddTodo
            | SlashCommand::Todos
            | SlashCommand::AuditMilestone
            | SlashCommand::CompleteMilestone
            | SlashCommand::ReapplyPatches => false,
            SlashCommand::Plan
            => false,
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::Experimental
            | SlashCommand::Review
            | SlashCommand::Logout => false,
            SlashCommand::Diff
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent => true,
            SlashCommand::Statusline => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::SlashCommand;

    #[test]
    fn gsd_workflow_commands_are_not_available_during_task() {
        for command in [
            SlashCommand::Progress,
            SlashCommand::WorkflowHelp,
            SlashCommand::PhaseAssumptions,
            SlashCommand::Todos,
            SlashCommand::PlanPhase,
            SlashCommand::QuickPlan,
        ] {
            assert!(command.is_gsd_workflow());
            assert!(!command.available_during_task());
        }
    }
}
