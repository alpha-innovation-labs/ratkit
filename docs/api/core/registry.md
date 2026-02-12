# Registry

Element registration and lifecycle management.

## Traits

### Element

Trait for UI elements that can be registered and rendered.

#### Required Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `id` | `fn id(&self) -> ElementId` | Get element ID |
| `render` | `fn render(&self, area: Rect, buf: &mut Buffer)` | Render the element |

#### Optional Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `on_event` | `fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction>` | Handle events |
| `metadata` | `fn metadata(&self) -> ElementMetadata` | Get element metadata |
| `set_metadata` | `fn set_metadata(&mut self, metadata: ElementMetadata)` | Set element metadata |
| `on_focus` | `fn on_focus(&mut self)` | Called when element receives focus |
| `on_blur` | `fn on_blur(&mut self)` | Called when element loses focus |
| `is_focusable` | `fn is_focusable(&self) -> bool` | Check if element can receive focus |
| `z_order` | `fn z_order(&self) -> i32` | Get Z-order for rendering |

## Structs

### ElementHandle

Handle to a registered element. Can be used to interact with elements.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new(id: ElementId) -> Self` | Create a new handle |
| `id` | `pub fn id(&self) -> ElementId` | Get the element ID |
| `upgrade` | `pub fn upgrade(&self, registry: &ElementRegistry) -> Option<ElementRef>` | Get strong reference |
| `is_alive` | `pub fn is_alive(&self, registry: &ElementRegistry) -> bool` | Check if element is still registered |

### ElementRegistry

Registry for managing element lifecycle.

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Create a new empty registry |
| `register` | `pub fn register(&mut self, element: Box<dyn Element>, metadata: ElementMetadata) -> ElementHandle` | Register an element |
| `unregister` | `pub fn unregister(&mut self, id: ElementId)` | Unregister an element |
| `get_strong_ref` | `pub fn get_strong_ref(&self, id: ElementId) -> Option<ElementRef>` | Get strong reference |
| `get_weak_ref` | `pub fn get_weak_ref(&self, id: ElementId) -> Option<ElementWeakRef>` | Get weak reference |
| `get_metadata` | `pub fn get_metadata(&self, id: ElementId) -> Option<&ElementMetadata>` | Get element metadata |
| `get_metadata_mut` | `pub fn get_metadata_mut(&mut self, id: ElementId) -> Option<&mut ElementMetadata>` | Get mutable metadata |
| `update_metadata` | `pub fn update_metadata(&mut self, id: ElementId, metadata: ElementMetadata)` | Update metadata |
| `set_visibility` | `pub fn set_visibility(&mut self, id: ElementId, visible: bool)` | Set visibility |
| `set_z_order` | `pub fn set_z_order(&mut self, id: ElementId, z_order: i32)` | Set Z-order |
| `len` | `pub fn len(&self) -> usize` | Get number of registered elements |
| `is_empty` | `pub fn is_empty(&self) -> bool` | Check if registry is empty |
| `all_ids` | `pub fn all_ids(&self) -> Vec<ElementId>` | Get all element IDs |
| `cleanup_dead_refs` | `pub fn cleanup_dead_refs(&mut self)` | Remove dead weak references |
| `focusable_elements` | `pub fn focusable_elements(&self) -> Vec<ElementId>` | Get focusable element IDs |
| `elements_by_region` | `pub fn elements_by_region(&self, region: Region) -> Vec<ElementId>` | Get elements in region |

## Type Aliases

| Alias | Definition |
|-------|------------|
| `ElementRef` | `Arc<RwLock<Box<dyn Element>>>` |
| `ElementWeakRef` | `Weak<RwLock<Box<dyn Element>>>` |
