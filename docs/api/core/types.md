# Types

Core type definitions used throughout ratkit.

## Structs

### ElementId

Unique identifier for elements (UUID-based).

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new random ID |
| `as_uuid` | `pub fn as_uuid(&self) -> Uuid` | Get underlying UUID |
| `from_uuid` | `pub fn from_uuid(uuid: Uuid) -> Self` | Create from UUID |

### ElementMetadata

Metadata associated with registered elements.

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `visible` | `Visibility` | Element visibility state |
| `can_receive_focus` | `bool` | Whether element can be focused |
| `z_order` | `i32` | Rendering order (higher = on top) |
| `fixed_height` | `Option<u16>` | Optional fixed height |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create default metadata |
| `is_visible` | `pub fn is_visible(&self) -> bool` | Check if element is visible |
| `can_receive_focus` | `pub fn can_receive_focus(&self) -> bool` | Check if focusable |
| `with_visibility` | `pub fn with_visibility(mut self, visible: Visibility) -> Self` | Set visibility |
| `with_focusable` | `pub fn with_focusable(mut self, focusable: bool) -> Self` | Set focusable |
| `with_z_order` | `pub fn with_z_order(mut self, z_order: i32) -> Self` | Set Z-order |
| `with_fixed_height` | `pub fn with_fixed_height(mut self, height: u16) -> Self` | Set fixed height |

### DirtyFlags

Flags tracking dirty state of layout and elements.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `clean` | `pub fn clean() -> Self` | Create clean state |
| `all_dirty` | `pub fn all_dirty() -> Self` | Create all dirty state |
| `is_dirty` | `pub fn is_dirty(&self) -> bool` | Check if anything is dirty |
| `needs_redraw` | `pub fn needs_redraw(&self) -> bool` | Check if redraw needed |
| `set_layout_dirty` | `pub fn set_layout_dirty(&mut self)` | Mark layout dirty |
| `set_elements_dirty` | `pub fn set_elements_dirty(&mut self)` | Mark elements dirty |
| `clear` | `pub fn clear(&mut self)` | Clear all dirty flags |

### LayoutState

State of the layout computation.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create new state |

### MouseSnapshot

Snapshot of mouse state for routing decisions.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new(element_id: ElementId) -> Self` | Create new snapshot |
| `is_stale` | `pub fn is_stale(&self, now: Instant) -> bool` | Check if snapshot is stale |

### DiagnosticInfo

Diagnostic information for debugging.

### LayoutStats

Statistics about layout computation.

## Enums

### Visibility

Element visibility state.

| Variant | Description |
|---------|-------------|
| `Visible` | Element is visible |
| `Hidden` | Element is hidden but still computed |
| `Collapsed` | Element is collapsed (not computed) |

### Region

Screen regions for layout.

| Variant | Description |
|---------|-------------|
| `Full` | Full screen |
| `Top(u16)` | Top section with given height |
| `Bottom(u16)` | Bottom section with given height |
| `Left(u16)` | Left section with given width |
| `Right(u16)` | Right section with given width |
| `Center` | Center area |

### MouseCaptureState

State of mouse capture.

| Variant | Description |
|---------|-------------|
| `None` | No mouse capture |
| `Captured { element_id, expires_at }` | Mouse captured by element |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `is_captured` | `pub fn is_captured(&self) -> bool` | Check if mouse is captured |
| `is_expired` | `pub fn is_expired(&self, now: Instant) -> bool` | Check if capture expired |
| `element_id` | `pub fn element_id(&self) -> Option<ElementId>` | Get capturing element ID |
| `remaining_time` | `pub fn remaining_time(&self, now: Instant) -> Option<Duration>` | Get remaining capture time |

### ResizeDebounceState

State for debouncing resize events.

| Variant | Description |
|---------|-------------|
| `Idle` | No pending resize |
| `Pending { new_size, deadline }` | Resize pending with deadline |
| `Processing` | Currently processing resize |

### LayoutError

Error types for layout operations.

| Variant | Description |
|---------|-------------|
| `ElementNotFound(ElementId)` | Element not found in registry |
| `ElementAlreadyRegistered(ElementId)` | Element already exists |
| `InvalidRegion(String)` | Invalid region specification |
| `LayoutComputation(String)` | Layout computation error |
| `EventRouting(String)` | Event routing error |
| `Focus(String)` | Focus management error |
| `MouseCapture(String)` | Mouse capture error |
| `TerminalTooSmall` | Terminal too small |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `element_not_found` | `pub fn element_not_found(id: ElementId) -> Self` | Create element not found error |
| `element_already_registered` | `pub fn element_already_registered(id: ElementId) -> Self` | Create already registered error |
| `invalid_region` | `pub fn invalid_region(msg: impl Into<String>) -> Self` | Create invalid region error |
| `layout_computation` | `pub fn layout_computation(msg: impl Into<String>) -> Self` | Create layout computation error |
| `event_routing` | `pub fn event_routing(msg: impl Into<String>) -> Self` | Create event routing error |
| `focus` | `pub fn focus(msg: impl Into<String>) -> Self` | Create focus error |
| `mouse_capture` | `pub fn mouse_capture(msg: impl Into<String>) -> Self` | Create mouse capture error |
| `terminal_too_small` | `pub fn terminal_too_small() -> Self` | Create terminal too small error |

## Type Aliases

| Alias | Definition |
|-------|------------|
| `LayoutResult<T>` | `Result<T, LayoutError>` |
