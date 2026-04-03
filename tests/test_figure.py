from rustplotlib._rustplotlib import RustFigure


def test_create_figure():
    fig = RustFigure(800, 600, 100)
    assert fig is not None


def test_add_axes_and_plot():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_savefig():
    import os
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {})
    fig.savefig("/tmp/test_rustplot.png")
    assert os.path.exists("/tmp/test_rustplot.png")
    os.remove("/tmp/test_rustplot.png")


def test_scatter():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_scatter(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_bar():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_bar(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_hist():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_hist(ax_id, [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_imshow():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    data_2d = [[0.0, 0.5, 1.0], [0.3, 0.7, 0.2], [1.0, 0.0, 0.5]]
    fig.axes_imshow(ax_id, data_2d, {})
    png = fig.render_to_png_bytes()
    assert len(png) > 0


def test_axes_options():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {"label": "my line"})
    fig.axes_set_title(ax_id, "Test Title")
    fig.axes_set_xlabel(ax_id, "X Axis")
    fig.axes_set_ylabel(ax_id, "Y Axis")
    fig.axes_set_xlim(ax_id, 0.0, 4.0)
    fig.axes_set_ylim(ax_id, 0.0, 8.0)
    fig.axes_grid(ax_id, True)
    fig.axes_legend(ax_id)
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_subplots():
    fig = RustFigure(800, 600, 100)
    fig.setup_subplots(1, 2)
    assert fig.num_axes() == 2
    fig.axes_plot(0, [1.0, 2.0], [3.0, 4.0], {})
    fig.axes_plot(1, [1.0, 2.0], [5.0, 6.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_set_size_inches():
    fig = RustFigure(800, 600, 100)
    fig.set_size_inches(10.0, 8.0)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 2.0], [3.0, 4.0], {})
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_savefig_svg():
    import os
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], {})
    fig.savefig("/tmp/test_rustplot.svg")
    assert os.path.exists("/tmp/test_rustplot.svg")
    with open("/tmp/test_rustplot.svg", "r") as f:
        content = f.read()
    assert "<svg" in content
    assert "data:image/png;base64," in content
    os.remove("/tmp/test_rustplot.svg")


def test_plot_with_kwargs():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(
        ax_id,
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 4.0, 2.0, 5.0],
        {
            "color": "red",
            "linewidth": 2.5,
            "linestyle": "--",
            "marker": "o",
            "markersize": 8.0,
            "label": "test line",
            "alpha": 0.7,
        },
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0
