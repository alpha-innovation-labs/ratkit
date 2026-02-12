# Runner

Main application runner that coordinates the event loop, rendering, and layout management.

## Struct

### Runner

Main application runner coordinating the event loop and rendering.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new runner with default configuration |
| `with_config` | `pub fn with_config(config: RunnerConfig) -> Self` | Create a runner with custom configuration |
| `config` | `pub fn config(&self) -> &RunnerConfig` | Get runner configuration reference |
| `coordinator` | `pub fn coordinator(&self) -> &LayoutCoordinator` | Get coordinator reference |
| `coordinator_mut` | `pub fn coordinator_mut(&mut self) -> &mut LayoutCoordinator` | Get mutable coordinator reference |
| `register_element` | `pub fn register_element(&mut self, element: Box<dyn Element>) -> ElementHandle` | Register an element |
| `unregister_element` | `pub fn unregister_element(&mut self, handle: &ElementHandle)` | Unregister an element |
| `set_visibility` | `pub fn set_visibility(&mut self, handle: &ElementHandle, visible: bool)` | Set element visibility |
| `request_focus` | `pub fn request_focus(&mut self, handle: &ElementHandle)` | Request focus for an element |
| `handle_event` | `pub fn handle_event(&mut self, event: RunnerEvent) -> LayoutResult<RunnerAction>` | Handle an event |
| `handle_coordinator_event` | `pub fn handle_coordinator_event(&mut self, event: CoordinatorEvent) -> LayoutResult<RunnerAction>` | Handle coordinator event |
| `handle_tick` | `pub fn handle_tick(&mut self) -> LayoutResult<RunnerAction>` | Handle tick event |
| `needs_redraw` | `pub fn needs_redraw(&self) -> bool` | Check if redraw is needed |
| `render` | `pub fn render(&mut self, frame: &mut Frame)` | Render all visible elements |
| `tick_count` | `pub fn tick_count(&self) -> u64` | Get current tick count |
| `is_layout_initialized` | `pub(self) fn is_layout_initialized(&self) -> bool` | Check if layout is initialized |
| `ensure_layout_initialized` | `pub(self) fn ensure_layout_initialized(&mut self)` | Ensure layout is initialized |
| `normalize_action` | `pub(self) fn normalize_action(&mut self, action: CoordinatorAction) -> RunnerAction` | Normalize coordinator action |
| `render_visible_elements` | `pub(self) fn render_visible_elements(&mut self, frame: &mut Frame)` | Render only visible elements |

## Enums

### RunnerAction

Actions the runner can take in response to events.

| Variant | Description |
|---------|-------------|
| `Continue` | Continue running the application |
| `Exit` | Exit the application |
| `RequestRedraw` | Request a redraw |

### RunnerEvent

Events that can be handled by the runner.

| Variant | Description |
|---------|-------------|
| `Keyboard(KeyboardEvent)` | Keyboard input event |
| `Mouse(MouseEvent)` | Mouse input event |
| `Resize(ResizeEvent)` | Terminal resize event |
| `Tick(TickEvent)` | Timer tick event |

**Methods:**
- `pub fn is_keyboard(&self) -> bool` - Check if event is keyboard
- `pub fn is_mouse(&self) -> bool` - Check if event is mouse
- `pub fn is_resize(&self) -> bool` - Check if event is resize
- `pub fn is_tick(&self) -> bool` - Check if event is tick
- `pub fn as_keyboard(&self) -> Option<&KeyboardEvent>` - Get keyboard event
- `pub fn as_mouse(&self) -> Option<&MouseEvent>` - Get mouse event
- `pub fn as_resize(&self) -> Option<&ResizeEvent>` - Get resize event
- `pub fn as_tick(&self) -> Option<&TickEvent>` - Get tick event

## Structs

### RunnerConfig

Configuration for the runner.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `coordinator_config` | `pub(self) fn coordinator_config(&self) -> CoordinatorConfig` | Get coordinator configuration |

## Type Aliases

| Alias | Definition |
|-------|------------|
| `RunnerEvent` | Defined in events module |
