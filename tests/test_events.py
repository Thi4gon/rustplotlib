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
