# ratkit

[![Crates.io](https://img.shields.io/crates/v/ratkit.svg)](https://crates.io/crates/ratkit)
[![Documentation](https://img.shields.io/docsrs/ratkit)](https://docs.rs/ratkit)
[![License](https://img.shields.io/crates/l/ratkit.svg)](LICENSE-MIT)

![ratkit Demo](demo/ratatui-toolkit-demo.gif)

Core runtime and reusable TUI components for [ratatui](https://ratatui.rs/), the Rust terminal UI library.

## Features

### Widgets (Complex Components)

| Component | Description | Feature Flag |
|-----------|-------------|--------------|
| **MarkdownWidget** | Full-featured markdown renderer with TOC, syntax highlighting, 25+ themes | `markdown-preview` |
| **AIChat** | AI chat interface with multi-line input and file attachments | `ai-chat` |
| **CodeDiff** | VS Code-style diff viewer with syntax highlighting | `code-diff` |
| **FileSystemTree** | File browser with devicons, filtering, and navigation | `file-system-tree` |
| **ThemePicker** | Modal theme selector with 25+ themes and search | `theme-picker` |
| **HotkeyFooter** | Keyboard shortcut display footer | `hotkey-footer` |

### Primitives (UI Building Blocks)

| Component | Description | Feature Flag |
|-----------|-------------|--------------|
| **TreeView** | Generic tree widget with expand/collapse, navigation, and selection | `tree-view` |
| **ResizableGrid** | Draggable split panels (vertical/horizontal) with mouse support | `resizable-grid` |
| **Dialog** | Modal dialogs (Info/Success/Warning/Error/Confirm) | `dialog` |
| **Toast** | Toast notifications with auto-expiry and severity levels | `toast` |
| **Button** | Clickable buttons with hover states | `button` |
| **Pane** | Bordered container with title, icon, and padding | `pane` |
| **MenuBar** | Horizontal menu bar with icons | `menu-bar` |
| **StatusLine** | Powerline-style status bar | `statusline` |
| **Scroll** | Scroll offset calculation utilities | `scroll` |
| **WidgetEvent** | Common event types for widget communication | `widget-event` |
| **TermTui** | Terminal emulator with mprocs-style copy mode | `termtui` |

### Services (Background Monitoring)

| Component | Description | Feature Flag |
|-----------|-------------|--------------|
| **FileWatcher** | Watch files/directories for changes | `file-watcher` |
| **GitWatcher** | Monitor git repository state changes | `git-watcher` |
| **RepoWatcher** | Combined file + git watching with git status integration | `repo-watcher` |
| **HotkeyService** | Global hotkey registration and scope-based filtering | `hotkey-service` |

## Installation

Add the core runtime to your `Cargo.toml`:

```toml
[dependencies]
ratkit = "0.2.13"
```

For the full bundle of components:

```toml
[dependencies]
ratkit = { version = "0.2.13", features = ["all"] }
```

For selected components:

```toml
[dependencies]
ratkit = { version = "0.2.13", default-features = false, features = ["markdown-preview", "tree-view", "button"] }
```

## Feature Flags

ratkit ships as a core runtime with optional components. By default only the
core runtime is enabled; opt in to specific components or use the `all` feature
to pull everything.

```toml
ratkit = { version = "0.2.13", default-features = false, features = ["tree-view", "toast"] }
```

### Feature Groups

| Feature | Description |
|---------|-------------|
| `default` | Core runtime only (Runner + Layout Manager) |
| `all` | All widgets and services |
| `full` | Alias for `all` |
| `widgets` | All UI widgets |
| `services` | All service components |

### All Features

**Widgets:**
- `markdown-preview` - Markdown preview widget (pulldown-cmark, syntect)
- `ai-chat` - AI chat widget (reqwest, serde)
- `code-diff` - Code diff widget (similar)
- `file-system-tree` - File browser (devicons)
- `theme-picker` - Theme picker widget
- `hotkey-footer` - Hotkey footer widget

**Primitives:**
- `button` - Button widget
- `pane` - Pane widget
- `dialog` - Modal dialog components
- `toast` - Toast notification system
- `statusline` - Powerline-style statusline
- `scroll` - Scrollable content helpers
- `menu-bar` - Menu bar component (enables `widget-event`)
- `resizable-grid` - Resizable split panels
- `tree-view` - Generic tree view widget (enables `widget-event`)
- `widget-event` - Widget event helpers
- `termtui` - Terminal emulator (TermTui)

**Services:**
- `file-watcher` - File watcher service (notify)
- `git-watcher` - Git watcher service (notify)
- `repo-watcher` - Repo watcher service (notify, enables file-watcher + git-watcher)
- `hotkey-service` - Hotkey service

## Quick Start

```rust
use ratkit::prelude::*;
use ratatui::Frame;

struct MyApp;

impl CoordinatorApp for MyApp {
    fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.is_escape() {
                    return Ok(CoordinatorAction::Quit);
                }
            }
            _ => {}
        }
        Ok(CoordinatorAction::Continue)
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        // Render your UI here
    }
}

fn main() -> std::io::Result<()> {
    run(MyApp, RunnerConfig::default())
}
```

## Examples

Run all examples with the interactive picker:

```bash
just demo
```

Or run specific examples with their required features:

```bash
cargo run --example <name> --features <feature>
```

### Widget Examples

Higher-level composite components with rich functionality:

#### MarkdownWidget
**Example:** `markdown_preview_markdown_preview_demo` — Full markdown viewer with TOC, syntax highlighting, and 25+ themes  
**Feature:** `markdown-preview`

- **Use when:** Rendering markdown with syntax highlighting, TOC, themes, and vim-style navigation
- **Enable:** `features = ["markdown-preview"]` (includes pulldown-cmark, syntect, notify)
- **Import:** `use ratkit::widgets::markdown_preview::*;`
- **Run:** `cargo run --example markdown_preview_markdown_preview_demo --features markdown-preview`
- **See:** `examples/markdown_preview_markdown_preview_demo.rs`

#### AIChat
**Example:** `ai_chat_ai_chat_demo` — AI chat interface with multi-line input  
**Feature:** `ai-chat`

- **Use when:** Building interactive AI chat interfaces with file attachments
- **Enable:** `features = ["ai-chat"]` (includes reqwest, serde)
- **Import:** `use ratkit::widgets::ai_chat::{AIChat, AIChatEvent};`
- **Run:** `cargo run --example ai_chat_ai_chat_demo --features ai-chat`
- **See:** `examples/ai_chat_ai_chat_demo.rs`

#### CodeDiff
**Example:** `code_diff_code_diff_demo` — VS Code-style diff viewer  
**Feature:** `code-diff`

- **Use when:** Displaying code differences with syntax highlighting
- **Enable:** `features = ["code-diff"]` (includes similar)
- **Import:** `use ratkit::widgets::code_diff::CodeDiff;`
- **Run:** `cargo run --example code_diff_code_diff_demo --features code-diff`
- **See:** `examples/code_diff_code_diff_demo.rs`

#### FileSystemTree
**Example:** `file_system_tree_file_system_tree_demo` — File browser with devicons  
**Feature:** `file-system-tree`

- **Use when:** Browsing file systems with icons and filtering
- **Enable:** `features = ["file-system-tree"]` (includes devicons)
- **Import:** `use ratkit::widgets::file_system_tree::*;`
- **Run:** `cargo run --example file_system_tree_file_system_tree_demo --features file-system-tree`
- **See:** `examples/file_system_tree_file_system_tree_demo.rs`

#### ThemePicker
**Example:** `theme_picker_theme_picker_demo` — Theme selector popup  
**Feature:** `theme-picker`

- **Use when:** Providing theme selection with search and live preview
- **Enable:** `features = ["theme-picker"]`
- **Import:** `use ratkit::widgets::theme_picker::{ThemePicker, ThemePickerEvent};`
- **Run:** `cargo run --example theme_picker_theme_picker_demo --features theme-picker`
- **See:** `examples/theme_picker_theme_picker_demo.rs`

#### HotkeyFooter
**Example:** `hotkey_footer_hotkey_footer_demo` — Keyboard shortcut display  
**Feature:** `hotkey-footer`

- **Use when:** Displaying context-aware keyboard shortcuts
- **Enable:** `features = ["hotkey-footer"]`
- **Import:** `use ratkit::widgets::hotkey_footer::{HotkeyFooter, HotkeyItem};`
- **Run:** `cargo run --example hotkey_footer_hotkey_footer_demo --features hotkey-footer`
- **See:** `examples/hotkey_footer_hotkey_footer_demo.rs`

### Primitive Examples

Core UI building blocks for custom interfaces:

#### Button
**Example:** `button_button_demo` — Interactive button with hover states  
**Feature:** `button`

- **Use when:** Clickable buttons with visual feedback
- **Enable:** `features = ["button"]`
- **Import:** `use ratkit::Button;`
- **Run:** `cargo run --example button_button_demo --features button`
- **See:** `examples/button_button_demo.rs`

#### Pane
**Example:** `pane_pane_demo` — Bordered panel containers  
**Feature:** `pane`

- **Use when:** Styled containers with title, icon, and padding
- **Enable:** `features = ["pane"]`
- **Import:** `use ratkit::Pane;`
- **Run:** `cargo run --example pane_pane_demo --features pane`
- **See:** `examples/pane_pane_demo.rs`

#### Dialog
**Example:** `dialog_dialog_demo` — Modal dialogs  
**Feature:** `dialog`

- **Use when:** Modal dialogs for confirmation or information
- **Enable:** `features = ["dialog"]`
- **Import:** `use ratkit::{Dialog, DialogState, DialogType};`
- **Run:** `cargo run --example dialog_dialog_demo --features dialog`
- **See:** `examples/dialog_dialog_demo.rs`

#### Toast
**Example:** `toast_toast_demo` — Toast notifications  
**Feature:** `toast`

- **Use when:** Auto-dismissing notification messages
- **Enable:** `features = ["toast"]`
- **Import:** `use ratkit::{ToastManager, ToastLevel};`
- **Run:** `cargo run --example toast_toast_demo --features toast`
- **See:** `examples/toast_toast_demo.rs`

#### StatusLine
**Example:** `statusline_statusline_demo` — Powerline-style status bar  
**Feature:** `statusline`

- **Use when:** Displaying compact status information in app headers/footers
- **Enable:** `features = ["statusline"]`
- **Import:** `use ratkit::StatusLine;`
- **Run:** `cargo run --example statusline_statusline_demo --features statusline`
- **See:** `examples/statusline_statusline_demo.rs`

#### Scroll
**Example:** `scroll_scroll_demo` — Scroll offset helpers  
**Feature:** `scroll`

- **Use when:** Managing viewport and cursor movement for long content
- **Enable:** `features = ["scroll"]`
- **Import:** `use ratkit::scroll::*;`
- **Run:** `cargo run --example scroll_scroll_demo --features scroll`
- **See:** `examples/scroll_scroll_demo.rs`

#### TreeView
**Example:** `tree-view_tree_view_demo` — Hierarchical tree widget  
**Feature:** `tree-view` (enables `widget-event`)

- **Use when:** Displaying hierarchical data with expand/collapse/selection
- **Enable:** `features = ["tree-view"]`
- **Import:** `use ratkit::{TreeNode, TreeView, TreeViewState, TreeNavigator};`
- **Run:** `cargo run --example tree-view_tree_view_demo --features tree-view`
- **See:** `examples/tree-view_tree_view_demo.rs`

#### ResizableGrid
**Example:** `resizable-grid_resizable_grid_demo` — Draggable split panels  
**Feature:** `resizable-grid`

- **Use when:** Multi-pane layouts with draggable dividers
- **Enable:** `features = ["resizable-grid"]`
- **Import:** `use ratkit::ResizableGrid;`
- **Run:** `cargo run --example resizable-grid_resizable_grid_demo --features resizable-grid`
- **See:** `examples/resizable-grid_resizable_grid_demo.rs`

#### MenuBar
**Example:** `menu-bar_menu_bar_demo` — Horizontal menu bar  
**Feature:** `menu-bar` (enables `widget-event`)

- **Use when:** Top application menu with icons and keyboard navigation
- **Enable:** `features = ["menu-bar"]`
- **Import:** `use ratkit::MenuBar;`
- **Run:** `cargo run --example menu-bar_menu_bar_demo --features menu-bar`
- **See:** `examples/menu-bar_menu_bar_demo.rs`

#### WidgetEvent
**Example:** `widget-event_widget_event_demo` — Shared widget event model  
**Feature:** `widget-event`

- **Use when:** Building custom widgets that exchange strongly-typed events
- **Enable:** `features = ["widget-event"]`
- **Import:** `use ratkit::WidgetEvent;`
- **Run:** `cargo run --example widget-event_widget_event_demo --features widget-event`
- **See:** `examples/widget-event_widget_event_demo.rs`

#### TermTui
**Example:** `termtui_term_mprocs_demo` — Terminal emulator  
**Feature:** `termtui`

- **Use when:** Embedded terminal with mprocs-style copy mode
- **Enable:** `features = ["termtui"]`
- **Import:** `use ratkit::termtui::Screen;`
- **Run:** `cargo run --example termtui_term_mprocs_demo --features termtui`
- **See:** `examples/termtui_term_mprocs_demo.rs`

### Service Examples

Background monitoring services:

#### FileWatcher
**Example:** `file_watcher_demo` — File change monitoring  
**Feature:** `file-watcher` (includes notify)

- **Use when:** Watching files or directories for changes
- **Enable:** `features = ["file-watcher"]`
- **Import:** `use ratkit::services::file_watcher::FileWatcher;`
- **Run:** `cargo run --example file_watcher_demo --features file-watcher`
- **See:** `examples/file_watcher_demo.rs`

#### GitWatcher
**Example:** `git_watcher_demo` — Git repository monitoring  
**Feature:** `git-watcher`

- **Use when:** Monitoring git repository state changes
- **Enable:** `features = ["git-watcher"]`
- **Import:** `use ratkit::services::git_watcher::GitWatcher;`
- **Run:** `cargo run --example git_watcher_demo --features git-watcher`
- **See:** `examples/git_watcher_demo.rs`

#### RepoWatcher
**Example:** `repo_watcher_demo` — Combined file + git watching  
**Feature:** `repo-watcher` (enables file-watcher + git-watcher)

- **Use when:** Comprehensive repository change tracking
- **Enable:** `features = ["repo-watcher"]`
- **Import:** `use ratkit::services::repo_watcher::RepoWatcher;`
- **Run:** `cargo run --example repo_watcher_demo --features repo-watcher`
- **See:** `examples/repo_watcher_demo.rs`

#### HotkeyService
**Example:** `hotkey_service_demo` — Global hotkey management  
**Feature:** `hotkey-service`

- **Use when:** Centralized hotkey registration with scope filtering
- **Enable:** `features = ["hotkey-service"]`
- **Import:** `use ratkit::services::hotkey_service::{HotkeyRegistry, HotkeyScope};`
- **Run:** `cargo run --example hotkey_service_demo --features hotkey-service`
- **See:** `examples/hotkey_service_demo.rs`

## Customizable Keybindings

Interactive components expose keybindings through configuration structs:

### TreeView

```rust
use ratkit::TreeKeyBindings;
use crossterm::event::KeyCode;

let bindings = TreeKeyBindings::new()
    .with_next(vec![KeyCode::Char('n'), KeyCode::Down])
    .with_previous(vec![KeyCode::Char('p'), KeyCode::Up])
    .with_expand(vec![KeyCode::Char('e'), KeyCode::Right])
    .with_collapse(vec![KeyCode::Char('c'), KeyCode::Left]);
```

## Architecture

All components follow ratatui's `Widget` and `StatefulWidget` patterns:

```rust
// Stateless widget
impl Widget for MyComponent {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}

// Stateful widget
impl StatefulWidget for MyComponent {
    type State = MyComponentState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) { ... }
}
```

### Mouse Support

Components support mouse interaction:

```rust
// Handle mouse events
if let Some(event) = component.handle_mouse(mouse_event, area) {
    match event {
        WidgetEvent::Clicked => { ... }
        _ => {}
    }
}
```

## License

Licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
