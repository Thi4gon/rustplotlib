"""Interactive widgets for rustplotlib (matplotlib-compatible).

Widgets work with the event system to provide interactive controls.
Slider and Button fire callbacks when their state changes.
"""


class Widget:
    """Base class for all widgets."""

    def __init__(self, ax):
        self.ax = ax
        self._active = True

    def set_active(self, active):
        """Set whether the widget is active."""
        self._active = active

    @property
    def active(self):
        return self._active


class Slider(Widget):
    """A slider widget.

    Call `on_changed(func)` to register callbacks that fire when the value changes.
    Call `set_val(val)` to programmatically change the value (also fires callbacks).

    Compatible with matplotlib.widgets.Slider.
    """

    def __init__(self, ax, label, valmin, valmax, valinit=None, valstep=None,
                 orientation='horizontal', **kwargs):
        super().__init__(ax)
        self.label = label
        self.valmin = valmin
        self.valmax = valmax
        self._valinit = valinit if valinit is not None else valmin
        self.val = self._valinit
        self.valstep = valstep
        self.orientation = orientation
        self._observers = {}
        self._next_cid = 0
        self.poly = None  # matplotlib compat
        self.vline = None  # matplotlib compat

    def on_changed(self, func):
        """Register a callback for value changes. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._observers.pop(cid, None)

    def set_val(self, val):
        """Set the slider value and fire all registered callbacks."""
        if not self._active:
            return
        # Clamp to range
        val = max(self.valmin, min(self.valmax, val))
        # Apply valstep
        if self.valstep is not None:
            val = self.valmin + round((val - self.valmin) / self.valstep) * self.valstep
            val = max(self.valmin, min(self.valmax, val))
        if val == self.val:
            return
        self.val = val
        # Fire callbacks
        for func in list(self._observers.values()):
            func(val)

    def reset(self):
        """Reset to initial value."""
        self.set_val(self._valinit)


class RangeSlider(Widget):
    """A range slider with two handles.

    Compatible with matplotlib.widgets.RangeSlider.
    """

    def __init__(self, ax, label, valmin, valmax, valinit=None, valstep=None, **kwargs):
        super().__init__(ax)
        self.label = label
        self.valmin = valmin
        self.valmax = valmax
        self.val = valinit if valinit is not None else (valmin, valmax)
        self.valstep = valstep
        self._observers = {}
        self._next_cid = 0

    def on_changed(self, func):
        """Register a callback for value changes. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._observers.pop(cid, None)

    def set_val(self, val):
        """Set the range slider value (tuple) and fire callbacks."""
        if not self._active:
            return
        lo = max(self.valmin, min(self.valmax, val[0]))
        hi = max(self.valmin, min(self.valmax, val[1]))
        if lo > hi:
            lo, hi = hi, lo
        new_val = (lo, hi)
        if new_val == self.val:
            return
        self.val = new_val
        for func in list(self._observers.values()):
            func(new_val)


class Button(Widget):
    """A clickable button widget.

    Call `on_clicked(func)` to register callbacks that fire on click.

    Compatible with matplotlib.widgets.Button.
    """

    def __init__(self, ax, label, **kwargs):
        super().__init__(ax)
        self.label = label
        self._observers = {}
        self._next_cid = 0
        self.color = kwargs.get('color', '0.85')
        self.hovercolor = kwargs.get('hovercolor', '0.95')

    def on_clicked(self, func):
        """Register a callback for click events. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._observers.pop(cid, None)

    def click(self):
        """Programmatically trigger a click (fires all callbacks)."""
        if not self._active:
            return
        from rustplotlib.events import MouseEvent
        event = MouseEvent("button_press_event", None, x=0, y=0, button=1)
        for func in list(self._observers.values()):
            func(event)


class CheckButtons(Widget):
    """A set of toggle buttons.

    Compatible with matplotlib.widgets.CheckButtons.
    """

    def __init__(self, ax, labels, actives=None, **kwargs):
        super().__init__(ax)
        self.labels = list(labels)
        self.actives = list(actives) if actives is not None else [False] * len(labels)
        self._observers = {}
        self._next_cid = 0

    def on_clicked(self, func):
        """Register a callback for toggle events. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._observers.pop(cid, None)

    def set_active(self, index):
        """Toggle the button at the given index."""
        if 0 <= index < len(self.actives):
            self.actives[index] = not self.actives[index]
            label = self.labels[index]
            for func in list(self._observers.values()):
                func(label)

    def get_status(self):
        """Return list of booleans for each button state."""
        return list(self.actives)


class RadioButtons(Widget):
    """A set of mutually exclusive buttons.

    Compatible with matplotlib.widgets.RadioButtons.
    """

    def __init__(self, ax, labels, active=0, **kwargs):
        super().__init__(ax)
        self.labels = list(labels)
        self.value_selected = labels[active] if labels else None
        self._active_index = active
        self._observers = {}
        self._next_cid = 0

    def on_clicked(self, func):
        """Register a callback for selection events. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._observers.pop(cid, None)

    def set_active(self, index):
        """Select the button at the given index."""
        if 0 <= index < len(self.labels):
            self._active_index = index
            self.value_selected = self.labels[index]
            for func in list(self._observers.values()):
                func(self.value_selected)


class TextBox(Widget):
    """A text input box.

    Compatible with matplotlib.widgets.TextBox.
    """

    def __init__(self, ax, label, initial='', **kwargs):
        super().__init__(ax)
        self.label = label
        self.text = initial
        self._submit_observers = {}
        self._change_observers = {}
        self._next_cid = 0

    def on_submit(self, func):
        """Register a callback for submit (Enter key). Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._submit_observers[cid] = func
        return cid

    def on_text_change(self, func):
        """Register a callback for text changes. Returns connection id."""
        cid = self._next_cid
        self._next_cid += 1
        self._change_observers[cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect a callback by connection id."""
        self._submit_observers.pop(cid, None)
        self._change_observers.pop(cid, None)

    def set_val(self, val):
        """Set the text value and fire change callbacks."""
        self.text = str(val)
        for func in list(self._change_observers.values()):
            func(self.text)

    def submit(self, text=None):
        """Programmatically submit. Fires submit callbacks."""
        if text is not None:
            self.text = str(text)
        for func in list(self._submit_observers.values()):
            func(self.text)


class Cursor(Widget):
    """A crosshair cursor that follows the mouse.

    Compatible with matplotlib.widgets.Cursor.
    Tracks mouse motion and stores current data coordinates.
    """

    def __init__(self, ax, horizOn=True, vertOn=True, useblit=False,
                 color='red', linewidth=1, linestyle='--', **kwargs):
        super().__init__(ax)
        self.horizOn = horizOn
        self.vertOn = vertOn
        self.useblit = useblit
        self.color = color
        self.linewidth = linewidth
        self.linestyle = linestyle
        self._x = None
        self._y = None
        self._callbacks = []

    @property
    def x(self):
        """Current x data coordinate."""
        return self._x

    @property
    def y(self):
        """Current y data coordinate."""
        return self._y

    def on_moved(self, func):
        """Register a callback when cursor moves. func(x, y)."""
        self._callbacks.append(func)

    def _update(self, x, y):
        """Update cursor position (called from events)."""
        if not self._active:
            return
        self._x = x
        self._y = y
        for cb in self._callbacks:
            cb(x, y)

    def clear(self):
        """Clear cursor position."""
        self._x = None
        self._y = None


class MultiCursor(Widget):
    """Cursor that spans multiple axes simultaneously.

    Compatible with matplotlib.widgets.MultiCursor.
    """

    def __init__(self, canvas, axes, horizOn=True, vertOn=True, **kwargs):
        # MultiCursor takes a canvas + list of axes
        super().__init__(axes[0] if axes else None)
        self.canvas = canvas
        self.axes = axes
        self.horizOn = horizOn
        self.vertOn = vertOn
        self._x = None
        self._y = None

    @property
    def x(self):
        return self._x

    @property
    def y(self):
        return self._y


class SpanSelector(Widget):
    """Select a span (range) on one axis.

    Compatible with matplotlib.widgets.SpanSelector.
    """

    def __init__(self, ax, onselect, direction, **kwargs):
        super().__init__(ax)
        self.onselect = onselect
        self.direction = direction

    def _select(self, vmin, vmax):
        """Programmatically trigger a selection."""
        if self._active and self.onselect:
            self.onselect(vmin, vmax)


class RectangleSelector(Widget):
    """Select a rectangular region.

    Compatible with matplotlib.widgets.RectangleSelector.
    """

    def __init__(self, ax, onselect, **kwargs):
        super().__init__(ax)
        self.onselect = onselect

    def _select(self, eclick, erelease):
        """Programmatically trigger a selection."""
        if self._active and self.onselect:
            self.onselect(eclick, erelease)


class LassoSelector(Widget):
    """Select a free-form region.

    Compatible with matplotlib.widgets.LassoSelector.
    """

    def __init__(self, ax, onselect, **kwargs):
        super().__init__(ax)
        self.onselect = onselect

    def _select(self, verts):
        """Programmatically trigger a selection."""
        if self._active and self.onselect:
            self.onselect(verts)
