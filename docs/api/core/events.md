# Events

Event types for handling user input and system events.

## Structs

### KeyboardEvent

Keyboard input event.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `from_crossterm` | `pub fn from_crossterm(event: crossterm::event::KeyEvent) -> Self` | Convert from crossterm event |
| `is_key_down` | `pub fn is_key_down(&self) -> bool` | Check if key was pressed |
| `is_key_up` | `pub fn is_key_up(&self) -> bool` | Check if key was released |
| `has_modifier` | `pub fn has_modifier(&self, modifier: KeyModifiers) -> bool` | Check for modifier key |
| `is_char` | `pub fn is_char(&self, c: char) -> bool` | Check if key is specific character |
| `is_code` | `pub fn is_code(&self, code: KeyCode) -> bool` | Check if key is specific code |
| `is_enter` | `pub fn is_enter(&self) -> bool` | Check if Enter key |
| `is_escape` | `pub fn is_escape(&self) -> bool` | Check if Escape key |
| `is_space` | `pub fn is_space(&self) -> bool` | Check if Space key |
| `is_tab` | `pub fn is_tab(&self) -> bool` | Check if Tab key |
| `is_backtab` | `pub fn is_backtab(&self) -> bool` | Check if Backtab (Shift+Tab) |
| `is_backspace` | `pub fn is_backspace(&self) -> bool` | Check if Backspace key |
| `is_delete` | `pub fn is_delete(&self) -> bool` | Check if Delete key |

### MouseEvent

Mouse input event.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `from_crossterm` | `pub fn from_crossterm(event: crossterm::event::MouseEvent) -> Self` | Convert from crossterm event |
| `position` | `pub fn position(&self) -> (u16, u16)` | Get mouse position (x, y) |
| `x` | `pub fn x(&self) -> u16` | Get x coordinate |
| `y` | `pub fn y(&self) -> u16` | Get y coordinate |
| `is_click` | `pub fn is_click(&self) -> bool` | Check if event is a click |
| `is_drag` | `pub fn is_drag(&self) -> bool` | Check if event is a drag |
| `is_scroll` | `pub fn is_scroll(&self) -> bool` | Check if event is scroll |
| `is_inside` | `pub fn is_inside(&self, rect: Rect) -> bool` | Check if position is inside rect |

### ResizeEvent

Terminal resize event.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new(width: u16, height: u16) -> Self` | Create new resize event |
| `area` | `pub fn area(&self) -> Rect` | Get area as Rect |

### TickEvent

Timer tick event for periodic updates.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new(tick_count: u64) -> Self` | Create new tick event |

## Enums

### RunnerEvent

Union of all possible runner events.

| Variant | Description |
|---------|-------------|
| `Keyboard(KeyboardEvent)` | Keyboard event |
| `Mouse(MouseEvent)` | Mouse event |
| `Resize(ResizeEvent)` | Resize event |
| `Tick(TickEvent)` | Tick event |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `is_keyboard` | `pub fn is_keyboard(&self) -> bool` | Check if keyboard event |
| `is_mouse` | `pub fn is_mouse(&self) -> bool` | Check if mouse event |
| `is_resize` | `pub fn is_resize(&self) -> bool` | Check if resize event |
| `is_tick` | `pub fn is_tick(&self) -> bool` | Check if tick event |
| `as_keyboard` | `pub fn as_keyboard(&self) -> Option<&KeyboardEvent>` | Get as keyboard event |
| `as_mouse` | `pub fn as_mouse(&self) -> Option<&MouseEvent>` | Get as mouse event |
| `as_resize` | `pub fn as_resize(&self) -> Option<&ResizeEvent>` | Get as resize event |
| `as_tick` | `pub fn as_tick(&self) -> Option<&TickEvent>` | Get as tick event |

## Example

```rust
use ratkit::prelude::*;

fn handle_event(event: RunnerEvent) {
    match event {
        RunnerEvent::Keyboard(k) if k.is_enter() => println!("Enter pressed"),
        RunnerEvent::Mouse(m) if m.is_click() => println!("Mouse clicked at {:?}", m.position()),
        _ => {}
    }
}
```
