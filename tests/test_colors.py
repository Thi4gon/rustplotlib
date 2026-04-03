from rustplot._rustplot import parse_color

def test_named_color():
    assert parse_color("red") == (255, 0, 0, 255)

def test_shorthand_color():
    assert parse_color("r") == (255, 0, 0, 255)
    assert parse_color("b") == (0, 0, 255, 255)
    assert parse_color("k") == (0, 0, 0, 255)

def test_hex_color():
    assert parse_color("#FF0000") == (255, 0, 0, 255)
    assert parse_color("#f00") == (255, 0, 0, 255)

def test_rgb_tuple():
    assert parse_color((1.0, 0.0, 0.0)) == (255, 0, 0, 255)

def test_rgba_tuple():
    assert parse_color((1.0, 0.0, 0.0, 0.5)) == (255, 0, 0, 127)
