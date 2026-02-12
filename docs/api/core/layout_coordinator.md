# LayoutCoordinator

Manages the layout computation, element registration, and rendering coordination.

## Struct

### LayoutCoordinator

Central coordinator for layout management and event dispatching.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new coordinator |
| `with_config` | `pub fn with_config(config: CoordinatorConfig) -> Self` | Create with custom config |
| `app` | `pub fn app(&self) -> Option<&dyn CoordinatorApp>` | Get app reference |
| `app_mut` | `pub fn app_mut(&mut self) -> Option<&mut dyn CoordinatorApp>` | Get mutable app reference |
| `layout` | `pub fn layout(&self) -> &LayoutManager` | Get layout manager reference |
| `layout_mut` | `pub fn layout_mut(&mut self) -> &mut LayoutManager` | Get mutable layout manager |
| `mouse` | `pub fn mouse(&self) -> &MouseRouter` | Get mouse router reference |
| `mouse_mut` | `pub fn mouse_mut(&mut self) -> &mut MouseRouter` | Get mutable mouse router |
| `focus` | `pub fn focus(&self) -> &FocusManager` | Get focus manager reference |
| `focus_mut` | `pub fn focus_mut(&mut self) -> &mut FocusManager` | Get mutable focus manager |
| `handle_event` | `pub fn handle_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction>` | Handle an event |
| `is_dirty` | `pub fn is_dirty(&self) -> bool` | Check if layout needs recomputation |
| `set_dirty` | `pub fn set_dirty(&mut self)` | Mark layout as dirty |
| `clear_dirty` | `pub fn clear_dirty(&mut self)` | Clear dirty flag |
| `invalidate_layout` | `pub fn invalidate_layout(&mut self)` | Invalidate layout (force recompute) |
| `invalidate_elements` | `pub fn invalidate_elements(&mut self)` | Invalidate elements |
| `get_diagnostic_info` | `pub fn get_diagnostic_info(&self) -> DiagnosticInfo` | Get diagnostic information |

**Internal Methods:**
- `handle_diagnostic_request` - Handle diagnostic request
- `handle_focus` - Handle focus events
- `handle_keyboard` - Handle keyboard events
- `handle_mouse` - Handle mouse events
- `handle_register` - Handle element registration
- `handle_unregister` - Handle element unregistration
- `handle_resize` - Handle resize events
- `handle_set_visibility` - Handle visibility changes
- `handle_tick` - Handle tick events
- `process_resize` - Process pending resize

## Enums

### CoordinatorAction

Actions the coordinator can request.

| Variant | Description |
|---------|-------------|
| `Continue` | Continue normal operation |
| `Exit` | Request application exit |
| `RequestRedraw` | Request a redraw |
| `FocusNext` | Move focus to next element |
| `FocusPrevious` | Move focus to previous element |

### CoordinatorEvent

Events handled by the coordinator.

| Variant | Description |
|---------|-------------|
| `Keyboard(KeyboardEvent)` | Keyboard event |
| `Mouse(MouseEvent)` | Mouse event |
| `Resize(ResizeEvent)` | Resize event |
| `Tick(TickEvent)` | Timer tick event |
| `RegisterElement { handle, element }` | Element registration |
| `UnregisterElement(ElementId)` | Element unregistration |
| `SetVisibility { id, visible }` | Visibility change |
| `FocusRequest(FocusRequest)` | Focus request |

## Traits

### CoordinatorApp

Trait for applications using the coordinator pattern.

#### Required Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `on_event` | `fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction>` | Handle events |
| `on_draw` | `fn on_draw(&mut self, frame: &mut Frame)` | Render the application |

## Structs

### CoordinatorConfig

Configuration for the coordinator.

## Example

```rust
use ratkit::prelude::*;

let coordinator = LayoutCoordinator::new();
```
