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


def test_log_transform():
    import math
    # data bounds in log space: log10(1)=0, log10(1000)=3
    t = Transform(
        data_xlim=(0.0, 3.0),  # log10(1) to log10(1000)
        data_ylim=(0.0, 3.0),
        pixel_left=0.0,
        pixel_right=300.0,
        pixel_top=0.0,
        pixel_bottom=300.0,
        log_x=True,
        log_y=True,
    )
    # x=1 -> log10(1)=0 -> pixel 0
    px, py = t.transform(1.0, 1.0)
    assert abs(px - 0.0) < 0.01
    # x=1000 -> log10(1000)=3 -> pixel 300
    px, py = t.transform(1000.0, 1000.0)
    assert abs(px - 300.0) < 0.01
    # x=10 -> log10(10)=1 -> pixel 100
    px, py = t.transform(10.0, 10.0)
    assert abs(px - 100.0) < 0.01


def test_log_x_only():
    import math
    t = Transform(
        data_xlim=(0.0, 2.0),  # log10(1) to log10(100)
        data_ylim=(0.0, 100.0),
        pixel_left=0.0,
        pixel_right=200.0,
        pixel_top=0.0,
        pixel_bottom=200.0,
        log_x=True,
        log_y=False,
    )
    # x=10 -> log10(10)=1 -> pixel 100
    px, py = t.transform(10.0, 50.0)
    assert abs(px - 100.0) < 0.01
    assert abs(py - 100.0) < 0.01
