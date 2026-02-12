# Focus Management

Focus handling and navigation for UI elements.

## Struct

### FocusManager

Manages focus state and navigation between focusable elements.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new focus manager |
| `registry` | `pub fn registry(&self) -> &ElementRegistry` | Get registry reference |
| `registry_mut` | `pub fn registry_mut(&mut self) -> &mut ElementRegistry` | Get mutable registry |
| `focused` | `pub fn focused(&self) -> Option<ElementId>` | Get currently focused element ID |
| `is_focused` | `pub fn is_focused(&self, id: ElementId) -> bool` | Check if element is focused |
| `captured_by` | `pub fn captured_by(&self) -> Option<ElementId>` | Get element that captured focus |
| `capture_focus` | `pub fn capture_focus(&mut self, id: ElementId) -> LayoutResult<()>` | Capture focus to element |
| `release_capture` | `pub fn release_capture(&mut self)` | Release focus capture |
| `rebuild_focus_stack` | `pub fn rebuild_focus_stack(&mut self)` | Rebuild focus order |
| `handle_request` | `pub fn handle_request(&mut self, request: FocusRequest) -> LayoutResult<()>` | Handle focus request |
| `remove_element` | `pub fn remove_element(&mut self, id: ElementId)` | Remove element from focus |

**Internal Methods:**
- `focus_next` - Move focus to next element
- `focus_previous` - Move focus to previous element
- `focus_first` - Focus first element
- `focus_last` - Focus last element
- `focus_to` - Focus specific element
- `release_focus` - Release current focus
- `notify_focus_change` - Notify of focus change

## Enums

### FocusRequest

Requests to change focus.

| Variant | Description |
|---------|-------------|
| `Next` | Move to next focusable element |
| `Previous` | Move to previous focusable element |
| `First` | Move to first focusable element |
| `Last` | Move to last focusable element |
| `To(ElementId)` | Move focus to specific element |
| `Capture(ElementId)` | Capture focus to element |
| `Release` | Release captured focus |

## Example

```rust
use ratkit::prelude::*;

let mut focus_manager = FocusManager::new();
focus_manager.handle_request(FocusRequest::Next)?;
```
