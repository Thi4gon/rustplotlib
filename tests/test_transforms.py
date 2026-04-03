from rustplotlib._rustplotlib import Transform

def test_data_to_pixel():
    t = Transform(
        data_xlim=(0.0, 10.0),
        data_ylim=(0.0, 100.0),
        pixel_left=80.0,
        pixel_right=780.0,
        pixel_top=60.0,
        pixel_bottom=560.0,
    )
    px, py = t.transform(0.0, 0.0)
    assert abs(px - 80.0) < 0.01
    assert abs(py - 560.0) < 0.01

    px, py = t.transform(10.0, 100.0)
    assert abs(px - 780.0) < 0.01
    assert abs(py - 60.0) < 0.01

def test_transform_midpoint():
    t = Transform(
        data_xlim=(0.0, 10.0),
        data_ylim=(0.0, 100.0),
        pixel_left=0.0,
        pixel_right=100.0,
        pixel_top=0.0,
        pixel_bottom=100.0,
    )
    px, py = t.transform(5.0, 50.0)
    assert abs(px - 50.0) < 0.01
    assert abs(py - 50.0) < 0.01
