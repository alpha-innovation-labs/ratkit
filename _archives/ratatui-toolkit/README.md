# ratatui-toolkit (deprecated)

This crate has been renamed to [`ratkit`](https://crates.io/crates/ratkit).

`ratatui-toolkit` is now a compatibility shim that re-exports `ratkit` and provides deprecation warnings.

## Migration

Replace in `Cargo.toml`:

```toml
[dependencies]
ratkit = "0.2"
```

Replace imports:

```rust
// before
use ratatui_toolkit::prelude::*;

// after
use ratkit::prelude::*;
```

## Feature mapping

Old feature names are mapped to `ratkit` features:

| ratatui-toolkit | ratkit |
|---|---|
| `markdown` | `markdown-preview` |
| `tree` | `tree-view` |
| `dialog` | `dialog` |
| `toast` | `toast` |
| `split` | `resizable-grid` |
| `menu` | `menu-bar` |
| `statusline` | `statusline` |
| `hotkey` | `hotkey-footer` |
| `terminal` | `termtui` |
| `file-tree` | `file-system-tree` |
| `theme` | `theme-picker` |

## Notes

- New development happens in `ratkit`.
- This compatibility crate may be removed in a future release.
