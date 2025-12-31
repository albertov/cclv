//! Conversation view widget - shared by main and subagent panes.
//!
//! PLACEHOLDER: This is a minimal implementation showing agent info.
//! Full conversation rendering (messages, markdown, syntax highlighting)
//! will be implemented in bead cclv-07v.4.2.

use crate::model::{AgentConversation, ContentBlock, ToolCall};
use crate::state::ScrollState;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render a conversation view for either main agent or subagent.
///
/// This is the shared widget used by both panes. It takes an AgentConversation
/// reference and renders it consistently regardless of which pane it's in.
///
/// # Arguments
/// * `frame` - The ratatui frame to render into
/// * `area` - The area to render within
/// * `conversation` - The agent conversation to display
/// * `_scroll` - Scroll state (unused in placeholder, prefix with _ to avoid warning)
/// * `focused` - Whether this pane currently has focus (affects border color)
pub fn render_conversation_view(
    frame: &mut Frame,
    area: Rect,
    conversation: &AgentConversation,
    _scroll: &ScrollState,
    focused: bool,
) {
    let entry_count = conversation.entries().len();

    // Build title with agent info
    let title = if let Some(agent_id) = conversation.agent_id() {
        // Subagent conversation
        let model_info = conversation
            .model()
            .map(|m| format!(" [{}]", m.display_name()))
            .unwrap_or_default();
        format!("Subagent {}{} ({} entries)", agent_id, model_info, entry_count)
    } else {
        // Main agent conversation
        let model_info = conversation
            .model()
            .map(|m| format!(" [{}]", m.display_name()))
            .unwrap_or_default();
        format!("Main Agent{} ({} entries)", model_info, entry_count)
    };

    // Style based on focus
    let border_color = if focused { Color::Cyan } else { Color::Gray };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(border_color));

    // Placeholder content
    let placeholder_text = if entry_count == 0 {
        "No messages yet...".to_string()
    } else {
        format!("Conversation with {} messages", entry_count)
    };

    let paragraph = Paragraph::new(placeholder_text).block(block);
    frame.render_widget(paragraph, area);
}

// ===== Content Block Rendering =====

#[allow(dead_code)] // Stub - tests will use this
/// Render a ContentBlock::ToolUse as formatted lines.
///
/// Displays:
/// - Tool name as header
/// - Tool input/parameters below header
///
/// # Arguments
/// * `tool_call` - The tool call to render
///
/// # Returns
/// Vector of ratatui `Line` objects representing the rendered tool use block
pub fn render_tool_use(_tool_call: &ToolCall) -> Vec<Line<'static>> {
    todo!("render_tool_use")
}

#[allow(dead_code)] // Stub - tests will use this
/// Render a ContentBlock::ToolResult as formatted lines.
///
/// Displays:
/// - Output content
/// - Error styling (red) when is_error=true
///
/// # Arguments
/// * `content` - The tool result content string
/// * `is_error` - Whether this result represents an error
///
/// # Returns
/// Vector of ratatui `Line` objects representing the rendered tool result
pub fn render_tool_result(_content: &str, _is_error: bool) -> Vec<Line<'static>> {
    todo!("render_tool_result")
}

#[allow(dead_code)] // Stub - tests will use this
/// Render any ContentBlock variant as formatted lines.
///
/// Delegates to specific rendering functions based on block type.
///
/// # Arguments
/// * `block` - The content block to render
///
/// # Returns
/// Vector of ratatui `Line` objects representing the rendered block
pub fn render_content_block(_block: &ContentBlock) -> Vec<Line<'static>> {
    todo!("render_content_block")
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ToolName, ToolUseId};
    use ratatui::style::Stylize;

    // ===== render_tool_use Tests =====

    #[test]
    fn render_tool_use_displays_tool_name() {
        let id = ToolUseId::new("tool-123").expect("valid id");
        let tool_call = ToolCall::new(
            id,
            ToolName::Read,
            serde_json::json!({"file_path": "/test.txt"}),
        );

        let lines = render_tool_use(&tool_call);

        // Tool name should be visible in the output
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Read"),
            "Tool name 'Read' should be visible in rendered output"
        );
    }

    #[test]
    fn render_tool_use_displays_input_parameters() {
        let id = ToolUseId::new("tool-456").expect("valid id");
        let tool_call = ToolCall::new(
            id,
            ToolName::Bash,
            serde_json::json!({"command": "ls -la"}),
        );

        let lines = render_tool_use(&tool_call);

        // Parameters should be visible
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("command") || text.contains("ls -la"),
            "Tool parameters should be visible in rendered output"
        );
    }

    #[test]
    fn render_tool_use_handles_different_tool_types() {
        let tools = vec![
            (ToolName::Read, serde_json::json!({"file": "a.txt"})),
            (ToolName::Write, serde_json::json!({"file": "b.txt"})),
            (ToolName::Grep, serde_json::json!({"pattern": "TODO"})),
            (ToolName::Bash, serde_json::json!({"command": "pwd"})),
        ];

        for (name, input) in tools {
            let id = ToolUseId::new("test-id").expect("valid id");
            let tool_call = ToolCall::new(id, name.clone(), input);

            let lines = render_tool_use(&tool_call);

            // Should produce output for each tool type
            assert!(
                !lines.is_empty(),
                "Should render output for tool: {:?}",
                name
            );
        }
    }

    // ===== render_tool_result Tests =====

    #[test]
    fn render_tool_result_displays_output_content() {
        let content = "File contents:\nLine 1\nLine 2";

        let lines = render_tool_result(content, false);

        // Output content should be visible
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Line 1") || text.contains("File contents"),
            "Tool result content should be visible"
        );
    }

    #[test]
    fn render_tool_result_applies_error_style_when_is_error_true() {
        let content = "Error: file not found";

        let lines = render_tool_result(content, true);

        // Error results should have red styling
        let has_red_style = lines.iter().any(|line| {
            line.spans
                .iter()
                .any(|span| span.style.fg == Some(Color::Red))
        });

        assert!(
            has_red_style,
            "Error tool results should be styled with red color"
        );
    }

    #[test]
    fn render_tool_result_does_not_apply_error_style_when_is_error_false() {
        let content = "Success output";

        let lines = render_tool_result(content, false);

        // Non-error results should not have red styling
        let has_red_style = lines.iter().any(|line| {
            line.spans
                .iter()
                .any(|span| span.style.fg == Some(Color::Red))
        });

        assert!(
            !has_red_style,
            "Non-error tool results should not be styled with red color"
        );
    }

    // ===== render_content_block Tests =====

    #[test]
    fn render_content_block_handles_tool_use() {
        let id = ToolUseId::new("test-tool").expect("valid id");
        let tool_call = ToolCall::new(id, ToolName::Read, serde_json::json!({"file": "x.txt"}));
        let block = ContentBlock::ToolUse(tool_call);

        let lines = render_content_block(&block);

        // Should render tool name
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Read"),
            "Should render ToolUse block with tool name visible"
        );
    }

    #[test]
    fn render_content_block_handles_tool_result() {
        let id = ToolUseId::new("result-123").expect("valid id");
        let block = ContentBlock::ToolResult {
            tool_use_id: id,
            content: "Output data".to_string(),
            is_error: false,
        };

        let lines = render_content_block(&block);

        // Should render result content
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Output data"),
            "Should render ToolResult block with content visible"
        );
    }

    #[test]
    fn render_content_block_handles_text() {
        let block = ContentBlock::Text {
            text: "Plain text message".to_string(),
        };

        let lines = render_content_block(&block);

        // Should render text content
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Plain text message"),
            "Should render Text block with text visible"
        );
    }

    #[test]
    fn render_content_block_handles_thinking() {
        let block = ContentBlock::Thinking {
            thinking: "Analyzing the problem...".to_string(),
        };

        let lines = render_content_block(&block);

        // Should render thinking content
        let text: String = lines.iter().map(|l| l.to_string()).collect();
        assert!(
            text.contains("Analyzing"),
            "Should render Thinking block with content visible"
        );
    }
}
