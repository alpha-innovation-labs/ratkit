# Mouse Router

Mouse event routing and capture management.

## Struct

### MouseRouter

Routes mouse events to the appropriate elements.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new mouse router |
| `with_config` | `pub fn with_config(config: MouseRouterConfig) -> Self` | Create with config |
| `config` | `pub fn config(&self) -> &MouseRouterConfig` | Get configuration |
| `capture` | `pub fn capture(&mut self, element_id: ElementId, duration: Duration)` | Capture mouse |
| `release_capture` | `pub fn release_capture(&mut self)` | Release mouse capture |
| `is_captured` | `pub fn is_captured(&self) -> bool` | Check if mouse is captured |
| `captured_element` | `pub fn captured_element(&self) -> Option<ElementId>` | Get capturing element |
| `capture_state` | `pub fn capture_state(&self) -> &MouseCaptureState` | Get capture state |
| `remaining_capture_time` | `pub fn remaining_capture_time(&self) -> Option<Duration>` | Get remaining time |
| `check_capture_expired` | `pub fn check_capture_expired(&mut self) -> bool` | Check and clear expired capture |
| `validate_capture` | `pub fn validate_capture(&mut self) -> bool` | Validate capture is still valid |
| `last_update` | `pub fn last_update(&self) -> Instant` | Get last update time |
| `snapshot` | `pub fn snapshot(&self) -> Option<&MouseSnapshot>` | Get current snapshot |
| `take_snapshot` | `pub fn take_snapshot(&mut self, element_id: ElementId)` | Take new snapshot |
| `is_snapshot_stale` | `pub fn is_snapshot_stale(&self) -> bool` | Check if snapshot is stale |
| `should_reroute_mouse` | `pub fn should_reroute_mouse(&self) -> bool` | Check if mouse should be rerouted |
| `route_mouse_event` | `pub fn route_mouse_event(&mut self, event: MouseEvent, layout: &LayoutManager) -> LayoutResult<Option<ElementId>>` | Route event to element |
| `handle_click_outside` | `pub fn handle_click_outside(&mut self) -> bool` | Handle click outside captured area |

## Structs

### MouseRouterConfig

Configuration for mouse routing behavior.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `capture_duration` | `Duration` | 5 seconds | How long mouse capture lasts |
| `snapshot_ttl` | `Duration` | 100 ms | How long snapshots remain valid |
| `reroute_threshold` | `Duration` | 50 ms | Threshold for rerouting decisions |

## Example

```rust
use ratkit::prelude::*;

let mut router = MouseRouter::with_config(MouseRouterConfig {
    capture_duration: Duration::from_secs(5),
    ..Default::default()
});
```
