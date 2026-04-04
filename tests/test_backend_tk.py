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
