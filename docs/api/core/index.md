# Core Runtime

The ratkit core runtime provides the foundation for building TUI applications with event handling, layout management, and rendering coordination.

## Modules

- [Runner](./runner.md) - Main application runner
- [Layout Coordinator](./layout_coordinator.md) - Layout management and element coordination
- [Event System](./events.md) - Input event handling
- [Types](./types.md) - Core type definitions
- [Registry](./registry.md) - Element registration and lifecycle
- [Focus Management](./focus.md) - Focus handling and navigation
- [Mouse Router](./mouse_router.md) - Mouse event routing

## Re-exports

These types are exported from the crate root for convenience:

### Core Types

| Type | Description |
|------|-------------|
| `Runner` | Main application runner |
| `RunnerConfig` | Configuration for the runner |
| `RunnerAction` | Actions the runner can take |
| `RunnerEvent` | Events handled by the runner |
| `CoordinatorApp` | Trait for coordinator-based applications |
| `CoordinatorAction` | Actions the coordinator can take |
| `CoordinatorEvent` | Events handled by the coordinator |
| `LayoutCoordinator` | Manages layout and rendering |
| `Element` | Trait for UI elements |
| `ElementHandle` | Handle to a registered element |
| `ElementId` | Unique identifier for elements |
| `ElementMetadata` | Metadata for registered elements |
| `Visibility` | Element visibility state |

### Event Types

| Type | Description |
|------|-------------|
| `KeyboardEvent` | Keyboard input events |
| `MouseEvent` | Mouse input events |
| `ResizeEvent` | Terminal resize events |
| `TickEvent` | Timer tick events |

### Result Types

| Type | Description |
|------|-------------|
| `LayoutResult<T>` | Result type for layout operations |
| `LayoutError` | Error type for layout operations |

### Configuration Types

| Type | Description |
|------|-------------|
| `MouseRouterConfig` | Configuration for mouse event routing |
| `CoordinatorConfig` | Configuration for the coordinator |
| `RedrawSignal` | Signal for requesting redraws |

### Focus Types

| Type | Description |
|------|-------------|
| `FocusRequest` | Request to change focus |

## Quick Start

```rust
use ratkit::prelude::*;
use ratatui::Frame;

struct MyApp;

impl CoordinatorApp for MyApp {
    fn on_event(&mut self, _event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        Ok(CoordinatorAction::Continue)
    }

    fn on_draw(&mut self, _frame: &mut Frame) {}
}

fn main() -> std::io::Result<()> {
    run(MyApp, RunnerConfig::default())
}
```
