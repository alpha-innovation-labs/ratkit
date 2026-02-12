//! Tools module for AI Chat widget.
//!
//! This module provides specialized tool display components for rendering
//! different types of tool calls in the AI Chat interface.
//!
//! # Tool Components
//!
//! - [`InlineTool`]: Compact inline tool status display
//! - [`BlockTool`]: Expanded block-level tool display
//! - [`ToolBash`]: Bash command tool ($)
//! - [`ToolWrite`]: File write tool (←)
//! - [`ToolEdit`]: File edit tool (←)
//! - [`ToolRead`]: File read tool (→)
//! - [`ToolGlob`]: Glob pattern tool (✱)
//! - [`ToolGrep`]: Content search tool (✱)
//! - [`ToolList`]: Directory listing tool (→)
//! - [`ToolWebFetch`]: Web fetch tool (%)
//! - [`ToolWebSearch`]: Web search tool (◈)
//! - [`ToolCodeSearch`]: Code search tool (◇)
//! - [`ToolTask`]: Task/subagent tool (#)
//! - [`ToolApplyPatch`]: Patch application tool (%)
//! - [`ToolTodoWrite`]: Todo write tool (⚙)
//! - [`ToolQuestion`]: Question tool (→)
//! - [`ToolSkill`]: Skill invocation tool (→)
//! - [`GenericTool`]: Generic fallback tool (⚙)
//!
//! # Icons
//!
//! | Tool Type | Icon |
//! |-----------|------|
//! | Bash      | $    |
//! | Write     | ←    |
//! | Edit      | ←    |
//! | Read      | →    |
//! | Glob      | ✱    |
//! | Grep      | ✱    |
//! | List      | →    |
//! | WebFetch  | %    |
//! | WebSearch | ◈    |
//! | CodeSearch| ◇    |
//! | Task      | #    |
//! | ApplyPatch| %    |
//! | TodoWrite | ⚙    |
//! | Question  | →    |
//! | Skill     | →    |
//! | Generic   | ⚙    |

// Re-export commonly used types
// Note: Some tool components have issues and need fixing
pub use block_tool::BlockTool;
pub use inline_tool::InlineTool;
pub use inline_tool::ToolStatus;
// pub use generic_tool::GenericTool;
// pub use tool_apply_patch::{FilePatch, PatchOperation, PatchStats, ToolApplyPatch};
// pub use tool_bash::ToolBash;
// pub use tool_codesearch::ToolCodeSearch;
// pub use tool_edit::{Diagnostic, DiagnosticSeverity, DiffLine, DiffLineType, DiffMode, ToolEdit};
// pub use tool_glob::ToolGlob;
// pub use tool_grep::{GrepContext, ToolGrep};
// pub use tool_list::{DirectoryEntry, ToolList};
// pub use tool_question::ToolQuestion;
// pub use tool_skill::ToolSkill;
// pub use tool_task::{SubAgentToolCall, ToolTask};
// pub use tool_todo_write::{TodoItem, ToolTodoWrite};
// pub use tool_webfetch::ToolWebFetch;
// pub use tool_websearch::{SearchResult, ToolWebSearch};
// pub use tool_write::ToolWrite;

// Base components
mod block_tool;
mod inline_tool;

// Specific tool implementations - temporarily disabled due to compilation issues
// mod generic_tool;
// mod tool_apply_patch;
// mod tool_bash;
// mod tool_codesearch;
// mod tool_edit;
// mod tool_glob;
// mod tool_grep;
// mod tool_list;
// mod tool_question;
// mod tool_skill;
// mod tool_task;
// mod tool_todo_write;
// mod tool_webfetch;
// mod tool_websearch;
// mod tool_write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_status_variants() {
        assert_eq!(ToolStatus::Pending, ToolStatus::Pending);
        assert_eq!(ToolStatus::Complete, ToolStatus::Complete);
        assert_eq!(ToolStatus::Error, ToolStatus::Error);
        assert_eq!(ToolStatus::PermissionPending, ToolStatus::PermissionPending);
    }

    #[test]
    fn test_inline_tool_creation() {
        let tool = InlineTool::new("bash", ToolStatus::Pending);
        assert_eq!(tool.name, "bash");
    }

    #[test]
    fn test_block_tool_creation() {
        let tool = BlockTool::new("read", ToolStatus::Complete, "content".to_string());
        assert_eq!(tool.name, "read");
    }
}
