"""Tests for functional widgets (Slider, Button, CheckButtons, RadioButtons, TextBox)."""
import rustplotlib.pyplot as plt
from rustplotlib.widgets import (
    Slider, RangeSlider, Button, CheckButtons, RadioButtons, TextBox,
    SpanSelector, RectangleSelector, Cursor,
)


def test_slider_basic():
    """Slider stores value and fires callbacks on set_val."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0, valinit=5.0)
    assert slider.val == 5.0
    assert slider.valmin == 0.0
    assert slider.valmax == 10.0
    plt.close()


def test_slider_on_changed():
    """Slider.on_changed registers callbacks that fire on set_val."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0, valinit=5.0)

    received = []
    cid = slider.on_changed(lambda val: received.append(val))
    assert isinstance(cid, int)

    slider.set_val(7.0)
    assert slider.val == 7.0
    assert received == [7.0]

    slider.set_val(3.0)
    assert received == [7.0, 3.0]
    plt.close()


def test_slider_disconnect():
    """Slider.disconnect removes callbacks."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0)

    received = []
    cid = slider.on_changed(lambda val: received.append(val))
    slider.disconnect(cid)

    slider.set_val(5.0)
    assert received == []
    plt.close()


def test_slider_clamp():
    """Slider clamps value to [valmin, valmax]."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0)

    slider.set_val(15.0)
    assert slider.val == 10.0

    slider.set_val(-5.0)
    assert slider.val == 0.0
    plt.close()


def test_slider_valstep():
    """Slider snaps to valstep increments."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0, valstep=2.0)

    slider.set_val(3.3)
    assert slider.val == 4.0  # snaps to nearest multiple of 2

    slider.set_val(6.8)
    assert slider.val == 6.0
    plt.close()


def test_slider_inactive():
    """Inactive slider ignores set_val."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0, valinit=5.0)
    slider.set_active(False)

    slider.set_val(8.0)
    assert slider.val == 5.0  # unchanged
    plt.close()


def test_slider_multiple_callbacks():
    """Multiple callbacks all fire on value change."""
    fig, ax = plt.subplots()
    slider = Slider(ax, "Test", 0.0, 10.0)

    a, b = [], []
    slider.on_changed(lambda v: a.append(v))
    slider.on_changed(lambda v: b.append(v))

    slider.set_val(5.0)
    assert a == [5.0]
    assert b == [5.0]
    plt.close()


def test_button_on_clicked():
    """Button.on_clicked fires callbacks on click."""
    fig, ax = plt.subplots()
    btn = Button(ax, "Click Me")

    received = []
    cid = btn.on_clicked(lambda event: received.append("clicked"))
    assert isinstance(cid, int)

    btn.click()
    assert received == ["clicked"]
    plt.close()


def test_button_disconnect():
    """Button.disconnect removes callbacks."""
    fig, ax = plt.subplots()
    btn = Button(ax, "Click Me")

    received = []
    cid = btn.on_clicked(lambda event: received.append("clicked"))
    btn.disconnect(cid)

    btn.click()
    assert received == []
    plt.close()


def test_button_inactive():
    """Inactive button ignores clicks."""
    fig, ax = plt.subplots()
    btn = Button(ax, "Click Me")

    received = []
    btn.on_clicked(lambda event: received.append("clicked"))
    btn.set_active(False)

    btn.click()
    assert received == []
    plt.close()


def test_checkbuttons():
    """CheckButtons toggles individual buttons and fires callbacks."""
    fig, ax = plt.subplots()
    cb = CheckButtons(ax, ["A", "B", "C"], [True, False, True])

    assert cb.get_status() == [True, False, True]

    toggled = []
    cb.on_clicked(lambda label: toggled.append(label))

    cb.set_active(1)  # toggle B
    assert cb.get_status() == [True, True, True]
    assert toggled == ["B"]

    cb.set_active(0)  # toggle A
    assert cb.get_status() == [False, True, True]
    assert toggled == ["B", "A"]
    plt.close()


def test_radiobuttons():
    """RadioButtons selects one option and fires callbacks."""
    fig, ax = plt.subplots()
    rb = RadioButtons(ax, ["Red", "Green", "Blue"], active=0)

    assert rb.value_selected == "Red"

    selected = []
    rb.on_clicked(lambda label: selected.append(label))

    rb.set_active(2)
    assert rb.value_selected == "Blue"
    assert selected == ["Blue"]
    plt.close()


def test_textbox():
    """TextBox handles text changes and submit."""
    fig, ax = plt.subplots()
    tb = TextBox(ax, "Input", initial="hello")

    assert tb.text == "hello"

    changes = []
    submits = []
    tb.on_text_change(lambda t: changes.append(t))
    tb.on_submit(lambda t: submits.append(t))

    tb.set_val("world")
    assert tb.text == "world"
    assert changes == ["world"]
    assert submits == []

    tb.submit()
    assert submits == ["world"]
    plt.close()


def test_range_slider():
    """RangeSlider supports tuple values."""
    fig, ax = plt.subplots()
    rs = RangeSlider(ax, "Range", 0.0, 100.0, valinit=(20.0, 80.0))

    assert rs.val == (20.0, 80.0)

    received = []
    rs.on_changed(lambda val: received.append(val))

    rs.set_val((30.0, 70.0))
    assert rs.val == (30.0, 70.0)
    assert received == [(30.0, 70.0)]
    plt.close()


def test_span_selector():
    """SpanSelector fires onselect callback."""
    fig, ax = plt.subplots()
    received = []
    ss = SpanSelector(ax, lambda vmin, vmax: received.append((vmin, vmax)), 'horizontal')

    ss._select(2.0, 5.0)
    assert received == [(2.0, 5.0)]
    plt.close()


def test_cursor_creation():
    """Cursor can be created without error."""
    fig, ax = plt.subplots()
    c = Cursor(ax, horizOn=True, vertOn=False)
    assert c.horizOn is True
    assert c.vertOn is False
    plt.close()


def test_widget_backward_compat():
    """Old-style widget creation still works (no crashes)."""
    fig, ax = plt.subplots()

    # These are how users create widgets in matplotlib
    s = Slider(ax, "S", 0, 1)
    s.on_changed(lambda v: None)

    b = Button(ax, "B")
    b.on_clicked(lambda e: None)

    cb = CheckButtons(ax, ["x", "y"])
    cb.on_clicked(lambda l: None)

    rb = RadioButtons(ax, ["a", "b", "c"])
    rb.on_clicked(lambda l: None)

    tb = TextBox(ax, "T", initial="test")
    tb.on_submit(lambda t: None)

    plt.close()
