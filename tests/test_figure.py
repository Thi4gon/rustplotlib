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
    fig.axes_grid(ax_id, True, {})
    fig.axes_legend(ax_id, {})
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
    # Native SVG: should contain real vector elements, not embedded PNG
    assert "<polyline" in content or "<line" in content
    assert "<text" in content
    assert "data:image/png;base64," not in content
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


def test_log_scale():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [1.0, 10.0, 100.0, 1000.0], [1.0, 10.0, 100.0, 1000.0], {})
    fig.axes_set_xscale(ax_id, "log")
    fig.axes_set_yscale(ax_id, "log")
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_log_scale_x_only():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_plot(ax_id, [0.1, 1.0, 10.0, 100.0], [1.0, 2.0, 3.0, 4.0], {})
    fig.axes_set_xscale(ax_id, "log")
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_errorbar():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_errorbar(
        ax_id,
        [1.0, 2.0, 3.0, 4.0],
        [10.0, 20.0, 15.0, 25.0],
        {"yerr": [1.0, 2.0, 1.5, 3.0], "marker": "o", "capsize": 5.0, "label": "data"},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_errorbar_xerr():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_errorbar(
        ax_id,
        [1.0, 2.0, 3.0],
        [10.0, 20.0, 15.0],
        {"xerr": [0.5, 0.3, 0.7], "yerr": [1.0, 2.0, 1.5]},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_barh():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_barh(
        ax_id,
        [1.0, 2.0, 3.0],
        [10.0, 20.0, 15.0],
        {"color": "blue", "label": "hbar"},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_boxplot():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_boxplot(
        ax_id,
        [
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 100.0],
            [10.0, 20.0, 30.0, 40.0, 50.0],
        ],
        {},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'


def test_boxplot_single():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_boxplot(
        ax_id,
        [[5.0, 8.0, 12.0, 15.0, 18.0, 20.0, 25.0, 30.0]],
        {"widths": 0.7},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0


def test_stem():
    fig = RustFigure(800, 600, 100)
    ax_id = fig.add_axes()
    fig.axes_stem(
        ax_id,
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [1.0, 4.0, 2.0, 5.0, 3.0],
        {"marker": "o", "label": "stem data"},
    )
    data = fig.render_to_png_bytes()
    assert len(data) > 0
    assert data[:4] == b'\x89PNG'
