"""Tests for backend system (base classes + Tk backend)."""
import pytest
import rustplotlib.pyplot as plt
from rustplotlib.backends.backend_base import FigureCanvasBase, FigureManagerBase


def test_canvas_base_interface():
    """FigureCanvasBase has the expected interface."""
    fig, ax = plt.subplots()
    canvas = FigureCanvasBase(fig)
    assert canvas.figure is fig
    assert hasattr(canvas, 'draw')
    assert hasattr(canvas, 'draw_idle')
    assert hasattr(canvas, 'mpl_connect')
    assert hasattr(canvas, 'mpl_disconnect')
    assert hasattr(canvas, 'get_width_height')
    plt.close()


def test_canvas_base_mpl_connect():
    """FigureCanvasBase supports event callbacks."""
    fig, ax = plt.subplots()
    canvas = FigureCanvasBase(fig)
    received = []
    cid = canvas.mpl_connect("test_event", lambda e: received.append(e))
    assert isinstance(cid, int)
    canvas.callbacks.process("test_event", "hello")
    assert received == ["hello"]
    canvas.mpl_disconnect(cid)
    plt.close()


def test_manager_base_interface():
    """FigureManagerBase has the expected interface."""
    fig, ax = plt.subplots()
    canvas = FigureCanvasBase(fig)
    manager = FigureManagerBase(canvas, 1)
    assert manager.canvas is canvas
    assert manager.num == 1
    assert hasattr(manager, 'show')
    assert hasattr(manager, 'destroy')
    assert hasattr(manager, 'set_window_title')
    plt.close()


def test_tk_canvas_creation():
    """FigureCanvasTk can be instantiated."""
    pytest.importorskip("tkinter")
    from rustplotlib.backends.backend_tk import FigureCanvasTk

    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])

    canvas = FigureCanvasTk(fig)
    assert canvas.figure is fig
    assert hasattr(canvas, 'draw')
    plt.close()


def test_tk_canvas_draw_renders():
    """FigureCanvasTk.draw() produces pixel data."""
    pytest.importorskip("tkinter")
    from rustplotlib.backends.backend_tk import FigureCanvasTk

    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])

    canvas = FigureCanvasTk(fig)
    canvas.draw()
    assert canvas._last_rgba is not None
    data, w, h = canvas._last_rgba
    assert w == 640
    assert h == 480
    assert len(data) == w * h * 4
    plt.close()


def test_tk_manager_creation():
    """FigureManagerTk can be instantiated."""
    pytest.importorskip("tkinter")
    from rustplotlib.backends.backend_tk import FigureCanvasTk, FigureManagerTk

    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])

    canvas = FigureCanvasTk(fig)
    manager = FigureManagerTk(canvas, 1)
    assert manager.canvas is canvas
    assert manager.num == 1
    plt.close()


def test_backend_registry():
    """Backend registry tracks available backends."""
    from rustplotlib.backends import get_backend, set_backend
    original = get_backend()
    # Default should be one of the valid backends
    assert original in ('agg', 'tk', 'inline')
    # Set and get
    set_backend('agg')
    assert get_backend() == 'agg'
    # Reset
    set_backend(original)


def test_backend_list():
    """list_backends returns available backends."""
    from rustplotlib.backends import list_backends
    backends = list_backends()
    assert 'agg' in backends
    assert 'inline' in backends
    assert 'tk' in backends


def test_pixel_to_data_conversion():
    """pixel_to_data converts pixel coords to data coords."""
    pytest.importorskip("tkinter")
    from rustplotlib.backends.backend_tk import FigureCanvasTk

    fig, ax = plt.subplots()
    ax.plot([0, 10], [0, 100])
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 100)

    canvas = FigureCanvasTk(fig)
    canvas.draw()

    # Center of plot area should be approximately center of data range
    # Plot area: left=70, right=620, top=40, bottom=430
    cx = (70 + 620) // 2  # pixel center x = 345
    cy = (40 + 430) // 2  # pixel center y = 235
    xdata, ydata = canvas.pixel_to_data(cx, cy)
    assert xdata is not None
    assert ydata is not None
    # Should be approximately center of data range (5, 50)
    assert abs(xdata - 5.0) < 1.0
    assert abs(ydata - 50.0) < 10.0
    plt.close()


def test_pixel_to_data_outside():
    """pixel_to_data returns None for points outside plot area."""
    pytest.importorskip("tkinter")
    from rustplotlib.backends.backend_tk import FigureCanvasTk

    fig, ax = plt.subplots()
    ax.plot([0, 10], [0, 100])

    canvas = FigureCanvasTk(fig)
    canvas.draw()

    # Point outside plot area (in margin)
    xdata, ydata = canvas.pixel_to_data(10, 10)
    assert xdata is None
    assert ydata is None
    plt.close()


def test_zoom_mode_toggle():
    """NavigationToolbar zoom mode toggles on/off."""
    from rustplotlib.backends.backend_base import NavigationToolbar2, FigureCanvasBase

    fig, ax = plt.subplots()
    canvas = FigureCanvasBase(fig)
    toolbar = NavigationToolbar2(canvas)

    assert toolbar._active_mode is None
    toolbar.zoom()
    assert toolbar._active_mode == 'zoom'
    toolbar.zoom()
    assert toolbar._active_mode is None
    plt.close()


def test_pan_mode_toggle():
    """NavigationToolbar pan mode toggles on/off."""
    from rustplotlib.backends.backend_base import NavigationToolbar2, FigureCanvasBase

    fig, ax = plt.subplots()
    canvas = FigureCanvasBase(fig)
    toolbar = NavigationToolbar2(canvas)

    toolbar.pan()
    assert toolbar._active_mode == 'pan'
    toolbar.pan()
    assert toolbar._active_mode is None
    plt.close()
