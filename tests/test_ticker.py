from rustplot._rustplot import auto_ticks, format_tick

def test_auto_ticks_simple():
    ticks = auto_ticks(0.0, 10.0)
    assert 0.0 in ticks
    assert 10.0 in ticks
    assert len(ticks) >= 3
    assert len(ticks) <= 12

def test_auto_ticks_negative():
    ticks = auto_ticks(-5.0, 5.0)
    assert 0.0 in ticks

def test_auto_ticks_small_range():
    ticks = auto_ticks(0.0, 1.0)
    assert len(ticks) >= 3
    for t in ticks:
        assert t >= 0.0
        assert t <= 1.0

def test_auto_ticks_large_range():
    ticks = auto_ticks(0.0, 10000.0)
    assert len(ticks) >= 3
    assert len(ticks) <= 12

def test_format_tick():
    assert format_tick(0.0) == "0"
    assert format_tick(1.5) == "1.5"
    assert format_tick(1000.0) == "1000"
    assert format_tick(0.001) == "0.001"
