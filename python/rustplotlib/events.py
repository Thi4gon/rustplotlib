"""Matplotlib-compatible event classes for rustplotlib.

Event hierarchy matches matplotlib.backend_bases:
  Event
  ├── LocationEvent
  │   ├── MouseEvent
  │   └── KeyEvent
  ├── DrawEvent
  ├── ResizeEvent
  └── CloseEvent
"""


class Event:
    """Base event class. All events have a name and source canvas."""

    def __init__(self, name, canvas, guiEvent=None):
        self.name = name
        self.canvas = canvas
        self.guiEvent = guiEvent


class LocationEvent(Event):
    """Event with pixel location and optional data coordinates."""

    def __init__(self, name, canvas, x=0, y=0, guiEvent=None):
        super().__init__(name, canvas, guiEvent)
        self.x = x
        self.y = y
        self.inaxes = None
        self.xdata = None
        self.ydata = None


class MouseEvent(LocationEvent):
    """Mouse button press/release/scroll event."""

    def __init__(self, name, canvas, x=0, y=0, button=None,
                 dblclick=False, step=0, guiEvent=None):
        super().__init__(name, canvas, x, y, guiEvent)
        self.button = button
        self.dblclick = dblclick
        self.step = step


class KeyEvent(LocationEvent):
    """Keyboard key press/release event."""

    def __init__(self, name, canvas, x=0, y=0, key=None, guiEvent=None):
        super().__init__(name, canvas, x, y, guiEvent)
        self.key = key


class PickEvent(Event):
    """Event fired when an artist is picked."""

    def __init__(self, name, canvas, mouseevent=None, artist=None, guiEvent=None):
        super().__init__(name, canvas, guiEvent)
        self.mouseevent = mouseevent
        self.artist = artist


class DrawEvent(Event):
    """Event fired after the canvas is drawn."""

    def __init__(self, name, canvas, renderer=None, guiEvent=None):
        super().__init__(name, canvas, guiEvent)
        self.renderer = renderer


class ResizeEvent(Event):
    """Event fired when the canvas is resized."""

    def __init__(self, name, canvas, width=0, height=0, guiEvent=None):
        super().__init__(name, canvas, guiEvent)
        self.width = width
        self.height = height


class CloseEvent(Event):
    """Event fired when the figure window is closed."""
    pass
