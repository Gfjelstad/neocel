## keyinput -> Key -> Operator/Motion/Command/Digit -> Command -> CommandDispatcher -> Engine -> Render

### keyinput -> Key: Key::from(KeyEvent)

### Key -> Operator/Motion/Command/Digit

input_engine.parse_key(&engine, key)

- keymap -> operator_map/motion_map/command_map
  - each map will look for document based node, then window, then global, based on the mode?

### Operator/Motion/Command/Digit -> Command

- if the command is complete (TODO: WTF DOES THIS MEAN)
  create command from input_engine::pending

### Command -> CommandDispatcher dispatcher.dispatch(CommandContext, Command)

# Plugin API Endpoints Table

| Endpoint Group | Endpoint | Arguments / Options | Returns | Notes / Document Type | Engine / Struct |
|----------------|---------|-------------------|--------|---------------------|----------------|
| **Buffer / Document API** | `doc_get_lines(doc_id, start, end)` | `doc_id`, start/end indices | list of strings / cells | Works for text lines, spreadsheet rows, form fields | `engine` |
|  | `doc_set_lines(doc_id, start, end, lines)` | `doc_id`, lines | success/failure | Atomic edit, creates undo block | `engine` |
|  | `doc_get_cell(doc_id, row, col)` | row/col | cell content | Spreadsheet only | `engine` |
|  | `doc_set_cell(doc_id, row, col, value)` | row/col/value | success | Spreadsheet only | `engine` |
|  | `doc_insert_row(doc_id, row_index, row_data)` | row index, row content | success | Spreadsheet / text only | `engine` |
|  | `doc_delete_row(doc_id, row_index)` | row index | success | Spreadsheet / text only | `engine` |
|  | `doc_get_option(doc_id, option_name)` | option string | value | e.g., `"readonly"`, `"type"` | `engine` |
|  | `doc_set_option(doc_id, option_name, value)` | option/value | success | Document-specific options | `engine` |
|  | `doc_get_type(doc_id)`  `"text" | "spreadsheet" | "form" | Read-only | `engine` |
| **Window / Layout API** | `win_open(doc_id, row, col, width, height)` | doc ID, layout | win ID | Returns window object to view doc | `engine` |
|  | `win_close(win_id)` | win ID | success | Closes view without deleting doc | `engine` |
|  | `win_get_cursor(win_id)` | win ID | row, col | Spreadsheet: cell; Text: line/col | `engine` |
|  | `win_set_cursor(win_id, row, col)` | win ID, row, col | success | Moves cursor / selection | `engine` |
|  | `win_get_view(win_id)` | win ID | `{topline, leftcol}` | Scroll/viewport info | `engine` |
|  | `win_set_view(win_id, view)` | win ID, `{topline, leftcol}` | success | Scrolls window | `engine` |
|  | `win_get_option(win_id, option)` | win ID, option string | value | e.g., line numbers, frozen panes | `engine` |
|  | `win_set_option(win_id, option, value)` | win ID, option/value | success | e.g., wrap, zoom | `engine` |
| **Tab / Workspace API** | `tab_get_current()` | – | tab ID | Current workspace/tab | `engine` |
|  | `tab_set_current(tab_id)` | tab ID | success | Switch workspace/tab | `engine` |
|  | `tab_list_windows(tab_id)` | tab ID | list of win IDs | List windows in tab | `engine` |
| **Mode / Input API** | `get_mode()` | – | `"normal"|"insert"|"visual"` | Read-only | `input_engine` |
|  | `enter_insert_mode()` | – | success | Mode request; deferred until safe | `input_engine` |
|  | `enter_normal_mode()` | – | success | Mode request | `input_engine` |
|  | `feed_keys(keys, mode_flags)` | keys string, mode flags | success | Low-level input injection | `input_engine` |
| **Command API** | `command_register(name, callback, opts)` | name, callback, `{nargs, range, bang}` | success | Adds to dispatcher | `command_dispatcher` |
|  | `command_unregister(name)` | name | success | Remove command | `command_dispatcher` |
|  | `command_list()` | – | list of registered commands | Includes built-in + plugin commands | `command_dispatcher` |
| **Renderer / UI API** | `buf_add_highlight(doc_id, ns, hl_group, row, col_start, col_end)` | namespace, hl, start/end | success | Spreadsheet: highlight cells; Text: chars | `renderer` |
|  | `buf_set_virtual_text(doc_id, ns, row, text)` | namespace, row, string | success | Form: virtual labels | `renderer` |
|  | `win_get_dimensions(win_id)` | win ID | `{width, height}` | Required for overlays | `renderer` |
| **Event / Notification API** | `on_doc_lines(doc_id, callback)` | callback triggered on change | – | Fires on edits | `engine` |
|  | `on_cursor(win_id, callback)` | callback triggered on cursor move | – | Spreadsheet: cell moves | `engine` |
|  | `subscribe(event_name, callback)` | custom events | – | e.g., `"doc_saved"`, `"selection_changed"` | `engine` |
