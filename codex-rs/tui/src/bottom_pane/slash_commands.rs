//! Shared helpers for filtering and matching built-in slash commands.
//!
//! The same sandbox- and feature-gating rules are used by both the composer
//! and the command popup. Centralizing them here keeps those call sites small
//! and ensures they stay in sync.
use codex_common::fuzzy_match::fuzzy_match;

use crate::slash_command::SlashCommand;
use crate::slash_command::built_in_slash_commands;

/// Return the built-ins that should be visible/usable for the current input.
pub(crate) fn builtins_for_input(
    collaboration_modes_enabled: bool,
    connectors_enabled: bool,
    personality_command_enabled: bool,
    allow_elevate_sandbox: bool,
) -> Vec<(&'static str, SlashCommand)> {
    built_in_slash_commands()
        .into_iter()
        .filter(|(_, cmd)| allow_elevate_sandbox || *cmd != SlashCommand::ElevateSandbox)
        .filter(|(_, cmd)| {
            collaboration_modes_enabled
                || (!cmd.is_gsd_workflow()
                    && !matches!(*cmd, SlashCommand::Collab | SlashCommand::Plan))
        })
        .filter(|(_, cmd)| connectors_enabled || *cmd != SlashCommand::Apps)
        .filter(|(_, cmd)| personality_command_enabled || *cmd != SlashCommand::Personality)
        .collect()
}

/// Find a single built-in command by exact name, after applying the gating rules.
pub(crate) fn find_builtin_command(
    name: &str,
    collaboration_modes_enabled: bool,
    connectors_enabled: bool,
    personality_command_enabled: bool,
    allow_elevate_sandbox: bool,
) -> Option<SlashCommand> {
    builtins_for_input(
        collaboration_modes_enabled,
        connectors_enabled,
        personality_command_enabled,
        allow_elevate_sandbox,
    )
    .into_iter()
    .find(|(command_name, cmd)| *command_name == name || cmd.aliases().contains(&name))
    .map(|(_, cmd)| cmd)
}

/// Whether any visible built-in fuzzily matches the provided prefix.
pub(crate) fn has_builtin_prefix(
    name: &str,
    collaboration_modes_enabled: bool,
    connectors_enabled: bool,
    personality_command_enabled: bool,
    allow_elevate_sandbox: bool,
) -> bool {
    builtins_for_input(
        collaboration_modes_enabled,
        connectors_enabled,
        personality_command_enabled,
        allow_elevate_sandbox,
    )
    .into_iter()
    .any(|(command_name, cmd)| {
        fuzzy_match(command_name, name).is_some()
            || cmd
                .aliases()
                .iter()
                .any(|alias| fuzzy_match(alias, name).is_some())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slash_command::SlashCommand;

    #[test]
    fn alias_lookup_resolves_hidden_gsd_commands() {
        assert_eq!(
            find_builtin_command("gsd:new-project", true, true, true, true),
            Some(SlashCommand::NewProject)
        );
        assert_eq!(
            find_builtin_command("gsd:plan-phase", true, true, true, true),
            Some(SlashCommand::PlanPhase)
        );
        assert_eq!(
            find_builtin_command("gsd:check-todos", true, true, true, true),
            Some(SlashCommand::Todos)
        );
    }

    #[test]
    fn builtin_prefix_matches_aliases() {
        assert!(has_builtin_prefix("gsd:new-proj", true, true, true, true));
        assert!(has_builtin_prefix("gsd:plan-ph", true, true, true, true));
    }

    #[test]
    fn builtins_list_shows_native_names_only() {
        let names: Vec<&str> = builtins_for_input(true, true, true, true)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert!(names.contains(&"new-project"));
        assert!(names.contains(&"quick-plan"));
        assert!(!names.contains(&"gsd:new-project"));
    }

    #[test]
    fn gsd_commands_are_hidden_when_collaboration_modes_disabled() {
        let names: Vec<&str> = builtins_for_input(false, true, true, true)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert!(!names.contains(&"new-project"));
        assert!(!names.contains(&"progress"));
        assert!(!names.contains(&"workflow-help"));
        assert!(!names.contains(&"plan"));
    }

    #[test]
    fn gsd_alias_lookup_is_disabled_without_collaboration_modes() {
        assert_eq!(
            find_builtin_command("new-project", false, true, true, true),
            None
        );
        assert_eq!(
            find_builtin_command("gsd:new-project", false, true, true, true),
            None
        );
        assert_eq!(
            find_builtin_command("progress", false, true, true, true),
            None
        );
    }
}
