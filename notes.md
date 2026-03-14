# Notes: UI Workspace Refactor

## Current Findings

### Workspace state
- `cargo check --workspace` passes on the current branch.
- The tree is already dirty, including new component/page files for calendar, chart, date picker, and table.

### Component seams
- Overlay-style widgets (`dialog`, `sheet`, `popover`, `sonner`, and `date_picker` bridge logic) all expose similar open/close APIs but duplicate action emission and state sync.
- Expandable widgets (`accordion`, `collapsible`) share animator setup and action semantics.
- Large stateful widgets mix pure domain logic with rendering and event handling.

### Gallery seams
- Sidebar items, router routes, command palette entries, and snippet lookup are duplicated by component name.
- Page modules repeatedly define the same preview shell structure with only content and snippet text changing.

### Constraints
- Avoid destructive cleanup because there are unrelated user changes in the same files.
- Follow Makepad v2 `script_mod!` registration patterns.
