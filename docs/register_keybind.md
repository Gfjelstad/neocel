# Register Keybind

Register a custom keybinding that maps a sequence of key presses to a command.

## Method Signature

```rust
pub fn register_keybind(state: &mut APIMethodParams) -> APIMethodResult
```

## Parameters

The method accepts a `RegisterKeybind` struct with the following fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `keys` | `Vec<String>` | Yes | Array of key strings representing the key sequence |
| `command_id` | `Option<String>` | No | The ID of the command to execute when the key sequence is triggered |
| `params` | `Value` | No | JSON parameters to pass to the command |

## Key Sequence Format

Key sequences are specified as an array of strings where each string represents a single key press. Keys can include modifiers separated by `+`.

### Supported Modifiers

- `Ctrl` or `Control`
- `Alt`
- `Shift`
- `Super`, `Meta`, or `Cmd`

### Special Keys

- `Space`
- `Enter` or `Return`
- `Esc` or `Escape`
- `Backspace`
- `Tab`
- `BackTab`
- `Up`, `Down`, `Left`, `Right`
- `F1` through `F12` (function keys)

### Character Keys

Any single character can be used as a key (e.g., `"a"`, `"h"`, `"1"`).

### Key Format Rules

- Modifiers are case-insensitive
- Multiple modifiers can be combined (e.g., `"Ctrl+Shift+A"`)
- Key sequences create a tree structure, allowing prefix-based keybindings

## Examples

### Basic Keybinding

Register a simple keybinding that executes a command when Space followed by 'f' is pressed:

```json
{
  "keys": ["Space", "f"],
  "command_id": "find_file",
  "params": {}
}
```

### Modified Key

Register a keybinding with modifiers:

```json
{
  "keys": ["Ctrl+s"],
  "command_id": "save_file",
  "params": {}
}
```

### Multi-Key Sequence

Register a complex key sequence (like Vim's leader keys):

```json
{
  "keys": ["Space", "g", "d"],
  "command_id": "goto_definition",
  "params": {}
}
```

### With Parameters

Register a keybinding that passes parameters to the command:

```json
{
  "keys": ["Shift+H"],
  "command_id": "move_cursor",
  "params": {
    "direction": "left",
    "distance": 10
  }
}
```

### Function Keys

Register a function key:

```json
{
  "keys": ["F5"],
  "command_id": "refresh",
  "params": {}
}
```

### Combined Modifiers

Register a keybinding with multiple modifiers:

```json
{
  "keys": ["Ctrl+Shift+P"],
  "command_id": "command_palette",
  "params": {}
}
```

## Behavior

- **Key Sequence Tree**: Keybindings are stored in a tree structure, allowing you to create hierarchical key sequences. For example, you can have both `["Space", "f"]` and `["Space", "b"]` registered simultaneously.

- **Prefix Matching**: When a key sequence is in progress but not complete, the editor waits for the next key press. For example, if you have `["Space", "f", "d"]` registered and press Space then 'f', the editor waits for the next key.

- **Command Execution**: Only complete key sequences trigger command execution. The command is executed with the provided parameters when the full sequence is matched.

- **Overwriting**: If you register a keybinding with a key sequence that already exists, it will overwrite the previous binding at that exact sequence endpoint.

## Error Handling

The method returns an error if:

- The key sequence is empty
- A key string contains an unknown modifier
- A key string contains an unknown special key
- A key string is malformed (e.g., invalid function key number)
- The command_id is `None` (no action to bind)

## Return Value

Returns `Ok(None)` on success, indicating the keybinding was successfully registered.

## Notes

- Keybindings are mode-specific and registered to the current input mode's keymap
- The keymap is cloned, modified, and the changes are persisted back to the input engine
- Case-insensitive parsing means `"ctrl+s"`, `"Ctrl+S"`, and `"CTRL+s"` are equivalent