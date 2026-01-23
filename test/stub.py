# editor_types.pyi
"""
Type stubs for the editor API.
These provide IntelliSense/autocomplete for injected globals.
"""

from typing import Any, Callable, Dict, List, Literal, Optional, TypedDict, Union, overload

# ============================================================================
# Parameter Types
# ============================================================================

class WindowIdParams(TypedDict):
    win_id: str

class WindowMoveParams(TypedDict):
    dir: Literal["up", "down", "left", "right"]

class SplitWindowParams(TypedDict):
    doc: str
    enter: bool
    src_win: str
    direction: Literal["up", "down", "left", "right"]
    border: Optional[Literal["single", "double", "rounded", "thick", "none"]]
    ratio: Optional[float]

class FloatingWindowParams(TypedDict, total=False):
    doc: str
    enter: bool
    position: Literal["center", "cursor", "absolute"]
    relative: Literal["editor", "win", "cursor"]
    win: Optional[str]  # Required if relative="win"
    width: int
    height: int
    row: Optional[int]  # Required if position="absolute"
    col: Optional[int]  # Required if position="absolute"
    style: Optional[str]
    border: Optional[Literal["single", "double", "rounded", "thick", "none"]]
    focusable: Optional[bool]
    zindex: Optional[int]

class RunCommandRequest(TypedDict):
    id: str
    args: Optional[List[Dict]]


class RunCommandParams(TypedDict):
    command: RunCommandRequest

CommandCallback = Callable[["API", Optional[Dict[str, Any]]], Any]

class RegisterCommandParams(TypedDict, total=False):
    id: str
    doc_type: Optional[Literal["spread_sheet", "info", "text"]]
    function: CommandCallback

class RegisterKeybindParams(TypedDict, total=False):
    keys: List[str]
    command_id: Optional[str]
    params: Optional[List[Dict]]

class ChangeModeParams(TypedDict):
    mode: Literal["normal", "insert", "visual", "command"]

# ============================================================================
# Return Types
# ============================================================================

class WindowInfo(TypedDict):
    window: Dict[str, Any]
    document: Dict[str, Any]

class WindowMoveResult(TypedDict):
    win_id: str

# ============================================================================
# API Class
# ============================================================================

class API:
    """Main API object for interacting with the editor"""
    version: str
    
    def __init__(self) -> None: ...
    
    # ========================================================================
    # Window Methods
    # ========================================================================
    
    @overload
    def call(self, method: Literal["window.create"], data: SplitWindowParams) -> None: ...
    
    @overload
    def call(self, method: Literal["window.create"], data: FloatingWindowParams) -> None: ...
    
    @overload
    def call(self, method: Literal["window.get_current"]) -> WindowInfo: ...
    
    @overload
    def call(self, method: Literal["window.get_window"], data: WindowIdParams) -> WindowInfo: ...
    
    @overload
    def call(self, method: Literal["window.close"], data: WindowIdParams) -> None: ...
    
    @overload
    def call(self, method: Literal["window.move"], data: WindowMoveParams) -> Optional[WindowMoveResult]: ...
    
    # ========================================================================
    # Command Methods
    # ========================================================================
    
    @overload
    def call(self, method: Literal["command.run"], data: RunCommandParams) -> Any: ...
    
    @overload
    def call(self, method: Literal["command.register"], data: RegisterCommandParams) -> None: ...
    
    @overload
    def call(self, method: Literal["keybind.register"], data: RegisterKeybindParams) -> None: ...
    
    # ========================================================================
    # Document Methods
    # ========================================================================
    
    @overload
    def call(self, method: Literal["doc.changeMode"], data: ChangeModeParams) -> None: ...
    
    # ========================================================================
    # System Methods
    # ========================================================================
    
    @overload
    def call(self, method: Literal["kill"]) -> None: ...
    
    @overload
    def call(self, method: Literal["test"]) -> None: ...
    
    # ========================================================================
    # Fallback
    # ========================================================================
    
    def call(self, method: str, data: Optional[Dict[str, Any]] = None) -> Any:
        """
        Call an API method with optional parameters.
        
        Available methods:
        
        Window Management:
        - "window.create": Create a new window (split or floating)
        - "window.get_current": Get the currently active window
        - "window.get_window": Get window by ID
        - "window.close": Close a window by ID
        - "window.move": Move focus to adjacent window
        
        Command Management:
        - "command.run": Execute a command
        - "command.register": Register a new command
        - "keybind.register": Register a keybinding
        
        Document:
        - "doc.changeMode": Change editor mode
        
        System:
        - "kill": Quit the editor
        - "test": Test method (prints to console)
        
        Args:
            method: The method name to call
            data: Parameters for the method (depends on the method)
            
        Returns:
            The result of the method call (type depends on the method)
            
        Examples:
            >>> # Get current window
            >>> api.call("window.get_current")
            {'window': {...}, 'document': {...}}
            
            >>> # Create split window
            >>> api.call("window.create", {
            ...     "doc": "file.txt",
            ...     "enter": True,
            ...     "src_win": "main",
            ...     "direction": "right",
            ...     "ratio": 0.5
            ... })
            
            >>> # Move window focus
            >>> api.call("window.move", {"dir": "left"})
            {'win_id': 'window_123'}
            
            >>> # Change mode
            >>> api.call("doc.changeMode", {"mode": "insert"})
            
            >>> # Register command
            >>> def my_command():
            ...     print("Command executed!")
            >>> api.call("command.register", {
            ...     "id": "my_command",
            ...     "function": my_command
            ... })
            
            >>> # Run command
            >>> api.call("command.run", {"command": "my_command"})
        """
        ...
    
    def methods(self) -> List[str]:
        """Get a list of all available methods"""
        ...
    
    def __repr__(self) -> str: ...


# ============================================================================
# Global instances injected at runtime
# ============================================================================

api: API