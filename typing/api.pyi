# api.pyi
from typing import Optional, Callable, Any, Literal, NotRequired, TypedDict, overload
from enum import Enum
from dataclasses import dataclass

SplitDirection = Literal[
    "up",
    "down",
    "left",
    "right"]

RelativeTo = Literal[
    "editor",
    "cursor",
    "win",]

Position = Literal[
    "top_right",
    "top_left",
    "bottom_right",
    "bottom_left",
    "center",
    "absolute",]

BorderStyle = Literal[
    # Add your border style variants here
    "none",
    "single",
    "double",]
    # ... other styles

# class CreateWindowParams(TypedDict):
#     """Base class for window creation parameters"""
#     pass

class SplitWindowParams(TypedDict):
    doc: str
    enter: bool
    src_win: str
    direction: SplitDirection
    border: NotRequired[BorderStyle]
    ratio: NotRequired[float]

class FloatingWindowParams(TypedDict):
    doc: str
    enter: bool
    relative: RelativeTo
    position: Position
    win: NotRequired[str]
    width: int
    height: int
    row: NotRequired[int]
    col: NotRequired[int]
    style: NotRequired[str]
    border: NotRequired[BorderStyle]
    focusable: NotRequired[bool]
    zindex: NotRequired[int]

class GetWindowResult:
    window_id: str
    document_id: str

class EngineAPI2:
    def __init__(self) -> None:
        """Cannot be constructed directly from Python"""
        raise RuntimeError("Api cannot be constructed directly from Python")
    
    

    @overload
    def create_window(self, options: FloatingWindowParams) -> str:...
    @overload
    def create_window(self, options: SplitWindowParams) -> str:...
    def create_window(self, options) -> str:
        """Create a new window (split or floating) and return its ID"""
        ...
    
    def get_current_window(self) -> GetWindowResult:
        """Get the currently active window"""
        ...
    
    def get_window(self, win_id: str) -> GetWindowResult:
        """Get window information by ID"""
        ...
    
    def close_window(self, window_id: str) -> None:
        """Close a window by ID"""
        ...
    
    def move_window(self, direction: SplitDirection) -> Optional[str]:
        """Move focus to neighboring window in the given direction.
        Returns the new window ID if successful, None if no neighbor exists."""
        ...
    
    def kill(self) -> None:
        """Shut down the engine"""
        ...
        
class CommandRequest(TypedDict):
    id: str
    args: NotRequired[list[Any]]

class BindOptions(TypedDict):
    doc: NotRequired[str]


class KeybindAPI:
    def bind(self, mode: str,
        keybind: list[str],
        function: Optional[Callable[..., Any]] = None,
        command: Optional[CommandRequest] = None,
        options: Optional[BindOptions] = None
    ) -> None:
        """Bind a key to either a Python function or a command.
        
        Args:
            mode: The mode character (e.g. 'n' for normal, 'i' for insert)
            keybind: The key sequence (e.g. '<leader>f', '<C-n>')
            function: Optional Python callback function
            command: Optional command parameters
        """
        ...

class DocType(Enum):
    """Document types for command registration"""
    # Add your actual DocType variants here, e.g.:
    Text = "text"
    Markdown = "markdown"
    # ... other types

class CommandAPI:
    def run(self, id: str, params: Optional[list[Any]] = None) -> Optional[Any]:
        """Execute a command by ID with optional parameters.
        
        Args:
            id: Command identifier
            params: Optional list of parameters to pass to the command
            
        Returns:
            The command's return value, or None
        """
        ...
    
    def list_current(self) -> list[str]:
        """List all currently available commands and operators.
        
        Returns:
            List of command/operator IDs
        """
        ...
    
    def register(
        self,
        id: str,
        function: Callable[..., Any],
        doctype: Optional[DocType] = None,
    ) -> None:
        """Register a new command.
        
        Args:
            id: Unique command identifier
            function: Python callable to execute when command is run
            doctype: Optional document type to restrict command to.
                    If None, registers as global command.
        """
        ...

class ThemeColors(TypedDict):
    """Theme color configuration.
    
    Only 'background' and 'foreground' are required.
    All other colors are optional.
    """
    # Required colors
    background: str
    foreground: str
    
    # Optional colors
    selection_background: NotRequired[str]
    selection_foreground: NotRequired[str]
    url_color: NotRequired[str]
    cursor: NotRequired[str]
    cursor_text_color: NotRequired[str]
    
    # Tabs
    active_tab_background: NotRequired[str]
    active_tab_foreground: NotRequired[str]
    inactive_tab_background: NotRequired[str]
    inactive_tab_foreground: NotRequired[str]
    tab_bar_background: NotRequired[str]
    
    # Windows
    active_border_color: NotRequired[str]
    inactive_border_color: NotRequired[str]
    
    # Normal colors (0-7)
    color0: NotRequired[str]
    color1: NotRequired[str]
    color2: NotRequired[str]
    color3: NotRequired[str]
    color4: NotRequired[str]
    color5: NotRequired[str]
    color6: NotRequired[str]
    color7: NotRequired[str]
    
    # Bright colors (8-15)
    color8: NotRequired[str]
    color9: NotRequired[str]
    color10: NotRequired[str]
    color11: NotRequired[str]
    color12: NotRequired[str]
    color13: NotRequired[str]
    color14: NotRequired[str]
    color15: NotRequired[str]
    
    # Extended colors
    color16: NotRequired[str]
    color17: NotRequired[str]

class ConfigAPI:
     def set_theme(self, theme: ThemeColors) -> None:
        """Set the editor theme with color mappings.
        
        Args:
            theme: Dictionary mapping theme keys to color values
                  (e.g., {"background": "#1e1e1e", "foreground": "#d4d4d4"})
        """
        ...


class TextDocumentAPI:

    @overload
    def create(self, path: str) -> str: ...
    @overload
    def create(self, content: str) -> str: ...
    def create(self) -> str:
        """Creates a new text document

        args must be either content OR path, not both, not neither
        
        Args:
            Optional[content]: Text Content of the document
            Optional[path]: Path to document 
        """
        ...
    def content(self, doc_id: str) -> list[str]:
        """Gets the content (lines) of a document given the id
        
        Args:
            doc_id: Document Id
        """
        ...




class DocumentAPI:
    def change_mode(self, mode: Literal["normal", "input"]) -> None: ...
    """API for document operations"""
    # Add methods based on your DocumentAPI implementation
    ...
    def get_cursor(self) -> tuple[int, int]: ...
    """API for document operations"""
    # Add methods based on your DocumentAPI implementation
    ...
    def set_cursor(self, row: int, col: int) -> None: ...
    """API for document operations"""
    # Add methods based on your DocumentAPI implementation
    ...

# Module-level instances
engine: EngineAPI2
commands: CommandAPI
config: ConfigAPI
text: TextDocumentAPI
document: DocumentAPI
keybinds: KeybindAPI