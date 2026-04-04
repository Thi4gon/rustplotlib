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
