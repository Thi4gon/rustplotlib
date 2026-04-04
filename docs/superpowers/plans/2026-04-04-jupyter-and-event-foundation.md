# v5.0.0 Phase 1: Jupyter Rich Display + Event Foundation

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship Jupyter rich display (`_repr_png_`, `_repr_svg_`, `_repr_html_`) and build the event/callback foundation that all interactive backends will use.

**Architecture:** Two independent tracks that can run in parallel. Track A adds SVG string export to Rust and wires Jupyter repr methods in Python. Track B creates matplotlib-compatible event classes and a CallbackRegistry, then wires them into the existing CanvasProxy/FigureProxy so `mpl_connect` works for real.

**Tech Stack:** Rust (PyO3, tiny-skia), Python (IPython display protocol)

---

## File Map

| File | Action | Responsibility |
|------|--------|---------------|
| `src/figure.rs` | Modify | Add `render_to_svg_string` and `render_to_rgba_buffer` PyO3 methods |
| `python/rustplotlib/pyplot.py` | Modify | Add `_repr_*_` to FigureProxy, wire CanvasProxy to CallbackRegistry |
| `python/rustplotlib/backends/backend_inline.py` | Modify | Add SVG display support, figure_format config |
| `python/rustplotlib/events.py` | Create | Matplotlib-compatible event class hierarchy |
| `python/rustplotlib/callback_registry.py` | Create | Signal/callback management |
| `tests/test_jupyter_repr.py` | Create | Tests for repr methods |
| `tests/test_events.py` | Create | Tests for event system and CallbackRegistry |

---

## Track A: Jupyter Rich Display

### Task 1: Add `render_to_svg_string` to RustFigure

**Files:**
- Modify: `src/figure.rs:2329` (near the `show` method)
- Test: `tests/test_jupyter_repr.py`

- [ ] **Step 1: Write the failing test**

Create `tests/test_jupyter_repr.py`:

```python
"""Tests for Jupyter rich display and SVG/RGBA export."""
import rustplotlib.pyplot as plt


def test_render_to_svg_string():
    """RustFigure.render_to_svg_string() returns valid SVG XML."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    svg = fig._fig.render_to_svg_string()
    assert isinstance(svg, str)
    assert svg.startswith("<svg")
    assert "</svg>" in svg
    assert len(svg) > 100
    plt.close()
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py::test_render_to_svg_string -v`
Expected: FAIL with `AttributeError: 'RustFigure' object has no attribute 'render_to_svg_string'`

- [ ] **Step 3: Implement render_to_svg_string in Rust**

In `src/figure.rs`, add this method inside the `#[pymethods] impl RustFigure` block, after the `render_to_png_bytes` method (around line 1971):

```rust
    /// Render the figure as an SVG XML string (for Jupyter _repr_svg_).
    fn render_to_svg_string(&self) -> PyResult<String> {
        Ok(self.render_svg_native(None, false))
    }
```

- [ ] **Step 4: Build and run test**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && maturin develop 2>&1 | tail -5 && pytest tests/test_jupyter_repr.py::test_render_to_svg_string -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/figure.rs tests/test_jupyter_repr.py
git commit -m "feat: expose render_to_svg_string via PyO3 for Jupyter SVG display"
```

---

### Task 2: Add `render_to_rgba_buffer` to RustFigure

**Files:**
- Modify: `src/figure.rs` (near `render_to_svg_string`)
- Test: `tests/test_jupyter_repr.py`

- [ ] **Step 1: Write the failing test**

Append to `tests/test_jupyter_repr.py`:

```python
def test_render_to_rgba_buffer():
    """RustFigure.render_to_rgba_buffer() returns raw RGBA bytes + dimensions."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    result = fig._fig.render_to_rgba_buffer()
    assert isinstance(result, tuple)
    assert len(result) == 3  # (bytes, width, height)
    data, w, h = result
    assert isinstance(data, bytes)
    assert w == 640  # default width
    assert h == 480  # default height
    assert len(data) == w * h * 4  # RGBA = 4 bytes per pixel
    plt.close()
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py::test_render_to_rgba_buffer -v`
Expected: FAIL with `AttributeError: 'RustFigure' object has no attribute 'render_to_rgba_buffer'`

- [ ] **Step 3: Implement render_to_rgba_buffer in Rust**

In `src/figure.rs`, add after `render_to_svg_string`:

```rust
    /// Render the figure as raw RGBA pixel buffer (for interactive backends).
    /// Returns (bytes, width, height).
    fn render_to_rgba_buffer<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyBytes>, u32, u32)> {
        let pixmap = self.render_pixmap_opts(None, false);
        let w = pixmap.width();
        let h = pixmap.height();
        let rgba_data: Vec<u8> = pixmap.pixels().iter().flat_map(|px| {
            // tiny-skia stores premultiplied RGBA — unpremultiply for standard RGBA
            let a = px.alpha();
            if a > 0 && a < 255 {
                let r = (px.red() as u16 * 255 / a as u16).min(255) as u8;
                let g = (px.green() as u16 * 255 / a as u16).min(255) as u8;
                let b = (px.blue() as u16 * 255 / a as u16).min(255) as u8;
                [r, g, b, a]
            } else if a == 255 {
                [px.red(), px.green(), px.blue(), 255]
            } else {
                [0, 0, 0, 0]
            }
        }).collect();
        Ok((PyBytes::new_bound(py, &rgba_data), w, h))
    }
```

- [ ] **Step 4: Build and run test**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && maturin develop 2>&1 | tail -5 && pytest tests/test_jupyter_repr.py::test_render_to_rgba_buffer -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/figure.rs tests/test_jupyter_repr.py
git commit -m "feat: expose render_to_rgba_buffer via PyO3 for interactive backends"
```

---

### Task 3: Add `_repr_png_`, `_repr_svg_`, `_repr_html_` to FigureProxy

**Files:**
- Modify: `python/rustplotlib/pyplot.py:1780` (FigureProxy class)
- Test: `tests/test_jupyter_repr.py`

- [ ] **Step 1: Write the failing tests**

Append to `tests/test_jupyter_repr.py`:

```python
def test_figure_proxy_repr_png():
    """FigureProxy._repr_png_() returns PNG bytes."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    png = fig._repr_png_()
    assert isinstance(png, bytes)
    assert png[:8] == b'\x89PNG\r\n\x1a\n'  # PNG magic number
    plt.close()


def test_figure_proxy_repr_svg():
    """FigureProxy._repr_svg_() returns SVG string."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    svg = fig._repr_svg_()
    assert isinstance(svg, str)
    assert "<svg" in svg
    assert "</svg>" in svg
    plt.close()


def test_figure_proxy_repr_html():
    """FigureProxy._repr_html_() returns HTML img tag with base64 PNG."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    html = fig._repr_html_()
    assert isinstance(html, str)
    assert "<img" in html
    assert "data:image/png;base64," in html
    plt.close()
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py -k "repr_png or repr_svg or repr_html" -v`
Expected: FAIL with `AttributeError: 'FigureProxy' object has no attribute '_repr_png_'`

- [ ] **Step 3: Add repr methods to FigureProxy**

In `python/rustplotlib/pyplot.py`, add these methods to the `FigureProxy` class (after the `show` method, around line 1828):

```python
    def _repr_png_(self):
        """Jupyter rich display: render as PNG bytes."""
        return bytes(self._fig.render_to_png_bytes())

    def _repr_svg_(self):
        """Jupyter rich display: render as SVG string."""
        return self._fig.render_to_svg_string()

    def _repr_html_(self):
        """Jupyter rich display: render as HTML img tag with base64 PNG."""
        import base64
        png = self._repr_png_()
        b64 = base64.b64encode(png).decode('ascii')
        return f'<img src="data:image/png;base64,{b64}" />'
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py -v`
Expected: ALL PASS (6 tests)

- [ ] **Step 5: Commit**

```bash
git add python/rustplotlib/pyplot.py tests/test_jupyter_repr.py
git commit -m "feat: add _repr_png_, _repr_svg_, _repr_html_ to FigureProxy for Jupyter"
```

---

### Task 4: Improve backend_inline.py with SVG support

**Files:**
- Modify: `python/rustplotlib/backends/backend_inline.py`
- Test: `tests/test_jupyter_repr.py`

- [ ] **Step 1: Write the failing test**

Append to `tests/test_jupyter_repr.py`:

```python
def test_backend_inline_figure_format():
    """backend_inline supports configurable figure_format."""
    from rustplotlib.backends import backend_inline
    # Default should be 'png'
    assert backend_inline.get_figure_format() == 'png'
    # Should accept valid formats
    backend_inline.set_figure_format('svg')
    assert backend_inline.get_figure_format() == 'svg'
    # Reset
    backend_inline.set_figure_format('png')


def test_backend_inline_display_svg(monkeypatch):
    """display_figure uses SVG when figure_format is 'svg'."""
    from rustplotlib.backends import backend_inline

    displayed = []

    class FakeDisplay:
        def __init__(self, data):
            self.data = data

    def fake_display(obj):
        displayed.append(obj)

    # Monkeypatch to avoid needing real IPython
    monkeypatch.setattr(backend_inline, '_display_func', fake_display, raising=False)

    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])

    backend_inline.set_figure_format('svg')
    backend_inline.display_figure(fig._fig, display_func=fake_display)
    assert len(displayed) == 1
    # The displayed object should be SVG-wrapped
    assert hasattr(displayed[0], 'data')
    assert '<svg' in displayed[0].data

    backend_inline.set_figure_format('png')
    plt.close()
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py::test_backend_inline_figure_format -v`
Expected: FAIL with `AttributeError: module has no attribute 'get_figure_format'`

- [ ] **Step 3: Rewrite backend_inline.py**

Replace the contents of `python/rustplotlib/backends/backend_inline.py`:

```python
"""Jupyter inline display backend for rustplotlib."""

import base64

_figure_format = 'png'


def get_figure_format():
    """Get the current inline figure format ('png' or 'svg')."""
    return _figure_format


def set_figure_format(fmt):
    """Set the inline figure format ('png', 'svg', or 'retina')."""
    if fmt not in ('png', 'svg', 'retina'):
        raise ValueError(f"Unsupported figure format: {fmt!r}. Use 'png', 'svg', or 'retina'.")
    global _figure_format
    _figure_format = fmt


class SVGWrapper:
    """Wrapper for SVG string display in Jupyter."""

    def __init__(self, data):
        self.data = data

    def _repr_svg_(self):
        return self.data


class PNGWrapper:
    """Wrapper for PNG bytes display in Jupyter."""

    def __init__(self, data):
        self.data = data

    def _repr_png_(self):
        return self.data


def display_figure(fig, display_func=None):
    """Display a figure inline in Jupyter.

    Parameters
    ----------
    fig : RustFigure
        The Rust figure object (not the FigureProxy).
    display_func : callable, optional
        Custom display function (for testing). Defaults to IPython.display.display.
    """
    if display_func is None:
        try:
            from IPython.display import display as display_func
        except ImportError:
            return

    fmt = get_figure_format()

    if fmt == 'svg':
        svg_str = fig.render_to_svg_string()
        display_func(SVGWrapper(svg_str))
    else:
        # 'png' or 'retina'
        png_bytes = fig.render_to_png_bytes()
        display_func(PNGWrapper(bytes(png_bytes)))


def configure_inline_support(shell=None, backend=None):
    """Configure Jupyter to display rustplotlib figures inline."""
    pass
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_jupyter_repr.py -v`
Expected: ALL PASS

- [ ] **Step 5: Run full test suite to verify no regressions**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/ -v --tb=short 2>&1 | tail -20`
Expected: 267+ tests pass, 0 failures

- [ ] **Step 6: Commit**

```bash
git add python/rustplotlib/backends/backend_inline.py tests/test_jupyter_repr.py
git commit -m "feat: backend_inline supports SVG display and configurable figure_format"
```

---

## Track B: Event Foundation

### Task 5: Create event class hierarchy

**Files:**
- Create: `python/rustplotlib/events.py`
- Test: `tests/test_events.py`

- [ ] **Step 1: Write the failing tests**

Create `tests/test_events.py`:

```python
"""Tests for the event system (matplotlib-compatible events + CallbackRegistry)."""
from rustplotlib.events import (
    Event, LocationEvent, MouseEvent, KeyEvent,
    DrawEvent, ResizeEvent, CloseEvent,
)


def test_event_base():
    """Event stores name and source."""
    e = Event("test_event", None)
    assert e.name == "test_event"
    assert e.canvas is None
    assert e.guiEvent is None


def test_location_event():
    """LocationEvent stores pixel coords and data coords."""
    e = LocationEvent("motion_notify_event", None, x=100, y=200)
    assert e.x == 100
    assert e.y == 200
    assert e.inaxes is None
    assert e.xdata is None
    assert e.ydata is None


def test_mouse_event():
    """MouseEvent stores button and double-click info."""
    e = MouseEvent("button_press_event", None, x=50, y=60, button=1)
    assert e.button == 1
    assert e.dblclick is False
    assert e.step == 0
    assert e.x == 50
    assert e.y == 60


def test_key_event():
    """KeyEvent stores the key name."""
    e = KeyEvent("key_press_event", None, x=0, y=0, key="ctrl+z")
    assert e.key == "ctrl+z"


def test_resize_event():
    """ResizeEvent stores width and height."""
    e = ResizeEvent("resize_event", None, width=800, height=600)
    assert e.width == 800
    assert e.height == 600


def test_draw_event():
    """DrawEvent stores renderer reference."""
    e = DrawEvent("draw_event", None, renderer="fake_renderer")
    assert e.renderer == "fake_renderer"


def test_close_event():
    """CloseEvent is a basic Event."""
    e = CloseEvent("close_event", None)
    assert e.name == "close_event"
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -v`
Expected: FAIL with `ModuleNotFoundError: No module named 'rustplotlib.events'`

- [ ] **Step 3: Create events.py**

Create `python/rustplotlib/events.py`:

```python
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
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -v`
Expected: ALL PASS (7 tests)

- [ ] **Step 5: Commit**

```bash
git add python/rustplotlib/events.py tests/test_events.py
git commit -m "feat: add matplotlib-compatible event class hierarchy"
```

---

### Task 6: Create CallbackRegistry

**Files:**
- Create: `python/rustplotlib/callback_registry.py`
- Test: `tests/test_events.py` (append)

- [ ] **Step 1: Write the failing tests**

Append to `tests/test_events.py`:

```python
from rustplotlib.callback_registry import CallbackRegistry


def test_callback_registry_connect_and_process():
    """Connect a callback and verify it fires when processed."""
    registry = CallbackRegistry()
    received = []

    def on_click(event):
        received.append(event)

    cid = registry.connect("button_press_event", on_click)
    assert isinstance(cid, int)

    registry.process("button_press_event", "fake_event")
    assert len(received) == 1
    assert received[0] == "fake_event"


def test_callback_registry_disconnect():
    """Disconnecting a callback prevents it from firing."""
    registry = CallbackRegistry()
    received = []

    cid = registry.connect("button_press_event", lambda e: received.append(e))
    registry.disconnect(cid)

    registry.process("button_press_event", "fake_event")
    assert len(received) == 0


def test_callback_registry_multiple_callbacks():
    """Multiple callbacks for the same signal all fire."""
    registry = CallbackRegistry()
    results = []

    registry.connect("motion_notify_event", lambda e: results.append("a"))
    registry.connect("motion_notify_event", lambda e: results.append("b"))

    registry.process("motion_notify_event", "fake_event")
    assert results == ["a", "b"]


def test_callback_registry_different_signals():
    """Callbacks only fire for their registered signal."""
    registry = CallbackRegistry()
    clicks = []
    keys = []

    registry.connect("button_press_event", lambda e: clicks.append(e))
    registry.connect("key_press_event", lambda e: keys.append(e))

    registry.process("button_press_event", "click")
    assert clicks == ["click"]
    assert keys == []


def test_callback_registry_disconnect_invalid():
    """Disconnecting an invalid cid does nothing (no crash)."""
    registry = CallbackRegistry()
    registry.disconnect(9999)  # should not raise
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -k "callback_registry" -v`
Expected: FAIL with `ModuleNotFoundError: No module named 'rustplotlib.callback_registry'`

- [ ] **Step 3: Create callback_registry.py**

Create `python/rustplotlib/callback_registry.py`:

```python
"""Signal/callback management compatible with matplotlib.cbook.CallbackRegistry."""


class CallbackRegistry:
    """Manage callbacks for named signals.

    Compatible with matplotlib's CallbackRegistry interface:
    - connect(signal, func) -> cid
    - disconnect(cid)
    - process(signal, *args, **kwargs)
    """

    def __init__(self):
        self._callbacks = {}  # {signal: {cid: func}}
        self._next_cid = 0

    def connect(self, signal, func):
        """Register func to be called when signal is processed. Returns a connection id."""
        cid = self._next_cid
        self._next_cid += 1
        if signal not in self._callbacks:
            self._callbacks[signal] = {}
        self._callbacks[signal][cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect the callback with the given connection id."""
        for signal_cbs in self._callbacks.values():
            if cid in signal_cbs:
                del signal_cbs[cid]
                return

    def process(self, signal, *args, **kwargs):
        """Process signal by calling all connected callbacks."""
        if signal in self._callbacks:
            for func in list(self._callbacks[signal].values()):
                func(*args, **kwargs)
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -v`
Expected: ALL PASS (12 tests)

- [ ] **Step 5: Commit**

```bash
git add python/rustplotlib/callback_registry.py tests/test_events.py
git commit -m "feat: add CallbackRegistry for event signal management"
```

---

### Task 7: Wire CanvasProxy to use CallbackRegistry

**Files:**
- Modify: `python/rustplotlib/pyplot.py:1763` (CanvasProxy class)
- Test: `tests/test_events.py` (append)

- [ ] **Step 1: Write the failing tests**

Append to `tests/test_events.py`:

```python
import rustplotlib.pyplot as plt


def test_canvas_mpl_connect():
    """CanvasProxy.mpl_connect registers callbacks that can be retrieved."""
    fig, ax = plt.subplots()
    canvas = fig.canvas

    received = []
    cid = canvas.mpl_connect("button_press_event", lambda e: received.append(e))
    assert isinstance(cid, int)
    plt.close()


def test_canvas_mpl_disconnect():
    """CanvasProxy.mpl_disconnect removes callbacks."""
    fig, ax = plt.subplots()
    canvas = fig.canvas

    received = []
    cid = canvas.mpl_connect("button_press_event", lambda e: received.append(e))
    canvas.mpl_disconnect(cid)

    # Process the event directly through the registry
    canvas.callbacks.process("button_press_event", "test")
    assert len(received) == 0
    plt.close()


def test_canvas_callbacks_process():
    """CanvasProxy callbacks fire when processed."""
    fig, ax = plt.subplots()
    canvas = fig.canvas

    received = []
    canvas.mpl_connect("button_press_event", lambda e: received.append(e))
    canvas.callbacks.process("button_press_event", "click!")
    assert received == ["click!"]
    plt.close()


def test_pyplot_connect_disconnect():
    """Module-level connect/disconnect delegate to current figure canvas."""
    fig, ax = plt.subplots()

    received = []
    cid = plt.connect("button_press_event", lambda e: received.append(e))
    assert isinstance(cid, int)
    plt.disconnect(cid)
    plt.close()
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -k "canvas" -v`
Expected: FAIL — `mpl_connect` currently returns None (it's a `pass` stub)

- [ ] **Step 3: Rewrite CanvasProxy in pyplot.py**

In `python/rustplotlib/pyplot.py`, replace the CanvasProxy class (lines 1763-1777):

```python
class CanvasProxy:
    """Canvas with event callback support (matplotlib-compatible)."""

    def __init__(self):
        from rustplotlib.callback_registry import CallbackRegistry
        self.callbacks = CallbackRegistry()

    def mpl_connect(self, event_name, callback):
        """Connect a callback to an event. Returns a connection id."""
        return self.callbacks.connect(event_name, callback)

    def mpl_disconnect(self, cid):
        """Disconnect a callback by its connection id."""
        self.callbacks.disconnect(cid)

    def draw(self):
        """Request a canvas redraw."""
        self.callbacks.process("draw_event")

    def draw_idle(self):
        """Request a canvas redraw at idle time."""
        self.callbacks.process("draw_event")
```

- [ ] **Step 4: Update module-level `connect`/`disconnect` in pyplot.py**

The existing stubs at lines 2720-2725 are `pass`. Replace them:

```python
def connect(event, func):
    """Connect a callback to the current figure's canvas. Returns cid."""
    return gcf().canvas.mpl_connect(event, func)


def disconnect(cid):
    """Disconnect a callback from the current figure's canvas."""
    gcf().canvas.mpl_disconnect(cid)
```

- [ ] **Step 5: Run tests**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -v`
Expected: ALL PASS (16 tests)

- [ ] **Step 6: Run full test suite**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/ -v --tb=short 2>&1 | tail -20`
Expected: 267+ tests pass, 0 failures. The change to CanvasProxy must not break existing tests.

- [ ] **Step 7: Commit**

```bash
git add python/rustplotlib/pyplot.py tests/test_events.py
git commit -m "feat: wire CanvasProxy to CallbackRegistry for real mpl_connect support"
```

---

### Task 8: Register new modules in package __init__

**Files:**
- Modify: `python/rustplotlib/__init__.py`
- Test: `tests/test_events.py` (append)

- [ ] **Step 1: Write the failing test**

Append to `tests/test_events.py`:

```python
def test_events_importable_from_package():
    """events module is importable from rustplotlib."""
    from rustplotlib import events
    assert hasattr(events, 'MouseEvent')
    assert hasattr(events, 'KeyEvent')
    assert hasattr(events, 'Event')


def test_callback_registry_importable_from_package():
    """callback_registry module is importable from rustplotlib."""
    from rustplotlib import callback_registry
    assert hasattr(callback_registry, 'CallbackRegistry')
```

- [ ] **Step 2: Run tests to check if they already pass**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/test_events.py -k "importable" -v`

If they already pass (Python auto-discovers submodules), skip to Step 4. If not, proceed to Step 3.

- [ ] **Step 3: Add imports to __init__.py (if needed)**

In `python/rustplotlib/__init__.py`, add:

```python
from rustplotlib import events
from rustplotlib import callback_registry
```

- [ ] **Step 4: Run full test suite**

Run: `cd /Users/thi4gon/Documents/workspace/matplot && source .venv/bin/activate && pytest tests/ -v --tb=short 2>&1 | tail -20`
Expected: ALL PASS (270+ tests — original 267 + new tests)

- [ ] **Step 5: Commit**

```bash
git add python/rustplotlib/__init__.py tests/test_events.py
git commit -m "feat: register events and callback_registry modules in package"
```

---

## Verification Checkpoint

After all 8 tasks are complete:

- [ ] **Run full test suite**: `pytest tests/ -v` — expect 270+ tests, 0 failures
- [ ] **Build release**: `maturin develop --release` — should succeed
- [ ] **Manual Jupyter test** (if Jupyter available):
  ```python
  import rustplotlib.pyplot as plt
  fig, ax = plt.subplots()
  ax.plot([1, 2, 3], [1, 4, 9])
  fig  # Should display inline in Jupyter
  ```
- [ ] **Manual mpl_connect test**:
  ```python
  import rustplotlib.pyplot as plt
  fig, ax = plt.subplots()
  ax.plot([1, 2, 3], [1, 4, 9])
  cid = fig.canvas.mpl_connect('button_press_event', lambda e: print(f'Click at {e}'))
  print(f"Connected with cid={cid}")
  ```
