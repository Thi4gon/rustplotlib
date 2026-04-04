"""Base classes for rustplotlib backends.

Defines the interface that all backends must implement.
Compatible with matplotlib's backend_bases module.
"""

from rustplotlib.callback_registry import CallbackRegistry


class FigureCanvasBase:
    """Base class for figure canvases. All backends implement this.

    The canvas is responsible for:
    - Rendering the figure (draw/draw_idle)
    - Event handling (mpl_connect/mpl_disconnect)
    - Size queries (get_width_height)
    """

    def __init__(self, figure):
        self.figure = figure
        self.callbacks = CallbackRegistry()
        self._is_idle_drawing = False

    def draw(self):
        """Render the figure. Subclasses must implement."""
        pass

    def draw_idle(self):
        """Request a draw at the next idle time."""
        if not self._is_idle_drawing:
            self._is_idle_drawing = True
            self.draw()
            self._is_idle_drawing = False

    def mpl_connect(self, event_name, callback):
        """Connect a callback to an event. Returns connection id."""
        return self.callbacks.connect(event_name, callback)

    def mpl_disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self.callbacks.disconnect(cid)

    def get_width_height(self):
        """Return canvas width and height in pixels."""
        fig = self.figure
        if hasattr(fig, '_fig'):
            # FigureProxy wrapping RustFigure
            rust_fig = fig._fig
            # RustFigure stores width/height directly
            return (640, 480)  # default, overridden by subclasses with actual render info
        return (640, 480)

    def flush_events(self):
        """Process pending GUI events."""
        pass

    def start_event_loop(self, timeout=0):
        """Start a blocking event loop."""
        pass

    def stop_event_loop(self):
        """Stop the current event loop."""
        pass


class FigureManagerBase:
    """Base class for figure window managers.

    The manager is responsible for:
    - Creating and managing the GUI window
    - Embedding the canvas in the window
    - Window operations (show, destroy, resize, title)
    """

    def __init__(self, canvas, num):
        self.canvas = canvas
        self.num = num
        self._window_title = f"Figure {num}"

    def show(self):
        """Show the figure window."""
        pass

    def destroy(self):
        """Destroy the figure window."""
        pass

    def set_window_title(self, title):
        """Set the window title."""
        self._window_title = title

    def resize(self, w, h):
        """Resize the window."""
        pass


class NavigationToolbar2:
    """Base class for navigation toolbars (zoom, pan, home, save).

    Manages a stack of view limits for back/forward navigation.
    """

    def __init__(self, canvas):
        self.canvas = canvas
        self._nav_stack = []
        self._current_idx = -1
        self._active_mode = None  # None, 'zoom', 'pan'

    def home(self):
        """Reset to the original view."""
        if self._nav_stack:
            self._current_idx = 0
            self._apply_view(self._nav_stack[0])

    def back(self):
        """Go to previous view in the stack."""
        if self._current_idx > 0:
            self._current_idx -= 1
            self._apply_view(self._nav_stack[self._current_idx])

    def forward(self):
        """Go to next view in the stack."""
        if self._current_idx < len(self._nav_stack) - 1:
            self._current_idx += 1
            self._apply_view(self._nav_stack[self._current_idx])

    def push_current(self):
        """Save current view limits to the stack."""
        fig = self.canvas.figure
        if hasattr(fig, '_axes') and fig._axes:
            axes_list = fig._axes if isinstance(fig._axes, list) else [fig._axes]
            views = []
            for ax in axes_list:
                xlim = ax.get_xlim() if hasattr(ax, 'get_xlim') else None
                ylim = ax.get_ylim() if hasattr(ax, 'get_ylim') else None
                views.append((xlim, ylim))
            self._nav_stack = self._nav_stack[:self._current_idx + 1]
            self._nav_stack.append(views)
            self._current_idx = len(self._nav_stack) - 1

    def _apply_view(self, views):
        """Apply saved view limits."""
        fig = self.canvas.figure
        if hasattr(fig, '_axes') and fig._axes:
            axes_list = fig._axes if isinstance(fig._axes, list) else [fig._axes]
            for ax, (xlim, ylim) in zip(axes_list, views):
                if xlim is not None and hasattr(ax, 'set_xlim'):
                    ax.set_xlim(*xlim)
                if ylim is not None and hasattr(ax, 'set_ylim'):
                    ax.set_ylim(*ylim)
            self.canvas.draw_idle()

    def zoom(self):
        """Toggle zoom mode."""
        self._active_mode = None if self._active_mode == 'zoom' else 'zoom'

    def pan(self):
        """Toggle pan mode."""
        self._active_mode = None if self._active_mode == 'pan' else 'pan'

    def save_figure(self, filename=None):
        """Save the figure to a file."""
        if filename:
            self.canvas.figure.savefig(filename)
