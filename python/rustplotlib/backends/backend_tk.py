"""Tkinter backend for rustplotlib — interactive figure display.

Uses tkinter (included with Python) for cross-platform window display.
The Rust engine renders to an RGBA buffer which is converted to a Tk PhotoImage.
"""

from rustplotlib.backends.backend_base import (
    FigureCanvasBase, FigureManagerBase, NavigationToolbar2,
)
from rustplotlib.events import MouseEvent, KeyEvent, ResizeEvent, CloseEvent


# Map tkinter button numbers to matplotlib button numbers
_TK_BUTTON_MAP = {1: 1, 2: 2, 3: 3}

# Map tkinter key names to matplotlib key names
_TK_KEY_MAP = {
    'Escape': 'escape', 'Return': 'enter', 'Tab': 'tab',
    'BackSpace': 'backspace', 'Delete': 'delete',
    'Left': 'left', 'Right': 'right', 'Up': 'up', 'Down': 'down',
    'Home': 'home', 'End': 'end',
    'Prior': 'pageup', 'Next': 'pagedown',
    'Control_L': 'control', 'Control_R': 'control',
    'Shift_L': 'shift', 'Shift_R': 'shift',
    'Alt_L': 'alt', 'Alt_R': 'alt',
    'Super_L': 'super', 'Super_R': 'super',
}


class FigureCanvasTk(FigureCanvasBase):
    """Tkinter canvas that displays a rustplotlib figure."""

    def __init__(self, figure):
        super().__init__(figure)
        self._tk_canvas = None
        self._photo_image = None
        self._last_rgba = None  # (bytes, width, height)

    def draw(self):
        """Render the figure to RGBA buffer and update the Tk canvas."""
        rust_fig = self.figure._fig
        result = rust_fig.render_to_rgba_buffer()
        self._last_rgba = result
        data, w, h = result

        if self._tk_canvas is not None:
            self._update_photo(data, w, h)

        self.callbacks.process("draw_event")

    def _update_photo(self, rgba_data, width, height):
        """Convert RGBA buffer to Tk PhotoImage and display it."""
        import tkinter as tk

        if self._photo_image is None or self._photo_image.width() != width or self._photo_image.height() != height:
            self._photo_image = tk.PhotoImage(width=width, height=height)

        # Convert RGBA bytes to PPM format (Tk PhotoImage doesn't support RGBA directly)
        header = f"P6 {width} {height} 255 ".encode('ascii')
        rgb_data = bytearray(width * height * 3)
        for i in range(width * height):
            offset = i * 4
            rgb_data[i * 3] = rgba_data[offset]
            rgb_data[i * 3 + 1] = rgba_data[offset + 1]
            rgb_data[i * 3 + 2] = rgba_data[offset + 2]

        ppm_data = header + bytes(rgb_data)
        self._photo_image.configure(data=ppm_data)

        self._tk_canvas.delete("all")
        self._tk_canvas.create_image(0, 0, anchor=tk.NW, image=self._photo_image)

    def _bind_events(self, tk_canvas):
        """Bind tkinter events to matplotlib-compatible events."""
        self._tk_canvas = tk_canvas

        tk_canvas.bind("<Button>", self._on_button_press)
        tk_canvas.bind("<ButtonRelease>", self._on_button_release)
        tk_canvas.bind("<Motion>", self._on_motion)
        tk_canvas.bind("<MouseWheel>", self._on_scroll)
        tk_canvas.bind("<Button-4>", self._on_scroll_up)
        tk_canvas.bind("<Button-5>", self._on_scroll_down)
        tk_canvas.bind("<Key>", self._on_key_press)
        tk_canvas.bind("<KeyRelease>", self._on_key_release)
        tk_canvas.bind("<Configure>", self._on_resize)

        tk_canvas.focus_set()

    def _translate_key(self, event):
        """Translate a tkinter key event to matplotlib key name."""
        key = _TK_KEY_MAP.get(event.keysym, event.keysym.lower())
        mods = []
        if event.state & 0x4:
            mods.append('ctrl')
        if event.state & 0x8:
            mods.append('alt')
        if event.state & 0x1 and len(key) > 1:
            mods.append('shift')
        if mods:
            key = '+'.join(mods + [key])
        return key

    def _on_button_press(self, event):
        button = _TK_BUTTON_MAP.get(event.num, event.num)
        me = MouseEvent("button_press_event", self, x=event.x, y=event.y,
                        button=button, guiEvent=event)
        self.callbacks.process("button_press_event", me)

    def _on_button_release(self, event):
        button = _TK_BUTTON_MAP.get(event.num, event.num)
        me = MouseEvent("button_release_event", self, x=event.x, y=event.y,
                        button=button, guiEvent=event)
        self.callbacks.process("button_release_event", me)

    def _on_motion(self, event):
        me = MouseEvent("motion_notify_event", self, x=event.x, y=event.y,
                        guiEvent=event)
        self.callbacks.process("motion_notify_event", me)

    def _on_scroll(self, event):
        step = event.delta / 120
        me = MouseEvent("scroll_event", self, x=event.x, y=event.y,
                        step=step, guiEvent=event)
        self.callbacks.process("scroll_event", me)

    def _on_scroll_up(self, event):
        me = MouseEvent("scroll_event", self, x=event.x, y=event.y,
                        step=1, guiEvent=event)
        self.callbacks.process("scroll_event", me)

    def _on_scroll_down(self, event):
        me = MouseEvent("scroll_event", self, x=event.x, y=event.y,
                        step=-1, guiEvent=event)
        self.callbacks.process("scroll_event", me)

    def _on_key_press(self, event):
        key = self._translate_key(event)
        ke = KeyEvent("key_press_event", self, x=0, y=0, key=key, guiEvent=event)
        self.callbacks.process("key_press_event", ke)

    def _on_key_release(self, event):
        key = self._translate_key(event)
        ke = KeyEvent("key_release_event", self, x=0, y=0, key=key, guiEvent=event)
        self.callbacks.process("key_release_event", ke)

    def _on_resize(self, event):
        re = ResizeEvent("resize_event", self, width=event.width, height=event.height,
                         guiEvent=event)
        self.callbacks.process("resize_event", re)

    def get_width_height(self):
        """Return figure dimensions from last render."""
        if self._last_rgba is not None:
            _, w, h = self._last_rgba
            return (w, h)
        return super().get_width_height()


class FigureManagerTk(FigureManagerBase):
    """Tkinter window manager for displaying figures."""

    def __init__(self, canvas, num):
        super().__init__(canvas, num)
        self._root = None
        self._tk_canvas = None
        self._toolbar = None

    def show(self):
        """Create Tk window and display the figure."""
        import tkinter as tk

        self._root = tk.Tk()
        self._root.title(self._window_title)
        self._root.protocol("WM_DELETE_WINDOW", self._on_close)

        w, h = self.canvas.get_width_height()

        self._toolbar = NavigationToolbarTk(self.canvas, self._root)

        self._tk_canvas = tk.Canvas(self._root, width=w, height=h,
                                     highlightthickness=0)
        self._tk_canvas.pack(fill=tk.BOTH, expand=True)

        self.canvas._bind_events(self._tk_canvas)
        self.canvas.draw()

        self._root.mainloop()

    def destroy(self):
        """Destroy the Tk window."""
        if self._root is not None:
            self._root.quit()
            self._root.destroy()
            self._root = None

    def _on_close(self):
        """Handle window close."""
        ce = CloseEvent("close_event", self.canvas)
        self.canvas.callbacks.process("close_event", ce)
        self.destroy()

    def set_window_title(self, title):
        super().set_window_title(title)
        if self._root is not None:
            self._root.title(title)

    def resize(self, w, h):
        if self._root is not None:
            self._root.geometry(f"{w}x{h}")


class NavigationToolbarTk(NavigationToolbar2):
    """Tkinter navigation toolbar with Home, Back, Forward, Pan, Zoom, Save buttons."""

    def __init__(self, canvas, root):
        super().__init__(canvas)
        import tkinter as tk

        self._frame = tk.Frame(root)
        self._frame.pack(side=tk.TOP, fill=tk.X)

        buttons = [
            ("Home", self.home),
            ("Back", self.back),
            ("Fwd", self.forward),
            ("Pan", self.pan),
            ("Zoom", self.zoom),
            ("Save", self._save_dialog),
        ]

        for label, command in buttons:
            btn = tk.Button(self._frame, text=label, command=command)
            btn.pack(side=tk.LEFT, padx=2, pady=2)

        self._status = tk.Label(self._frame, text="", anchor=tk.W)
        self._status.pack(side=tk.RIGHT, fill=tk.X, expand=True, padx=4)

        canvas.mpl_connect("motion_notify_event", self._update_status)
        self.push_current()

    def _update_status(self, event):
        if event.x is not None and event.y is not None:
            self._status.config(text=f"x={event.x} y={event.y}")

    def _save_dialog(self):
        import tkinter.filedialog as fd
        filename = fd.asksaveasfilename(
            defaultextension=".png",
            filetypes=[
                ("PNG files", "*.png"),
                ("SVG files", "*.svg"),
                ("PDF files", "*.pdf"),
            ],
        )
        if filename:
            self.save_figure(filename)
