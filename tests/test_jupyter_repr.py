"""Tests for Jupyter rich display and SVG/RGBA export."""
import rustplotlib.pyplot as plt


def test_render_to_svg_string():
    """RustFigure.render_to_svg_string() returns valid SVG XML."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    svg = fig._fig.render_to_svg_string()
    assert isinstance(svg, str)
    # SVG may start with XML declaration or directly with <svg
    assert "<svg" in svg
    assert "</svg>" in svg
    assert len(svg) > 100
    plt.close()


def test_render_to_rgba_buffer():
    """RustFigure.render_to_rgba_buffer() returns raw RGBA bytes + dimensions."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    result = fig._fig.render_to_rgba_buffer()
    assert isinstance(result, tuple)
    assert len(result) == 3  # (bytes, width, height)
    data, w, h = result
    assert isinstance(data, bytes)
    assert w == 640  # default width
    assert h == 480  # default height
    assert len(data) == w * h * 4  # RGBA = 4 bytes per pixel
    plt.close()


def test_figure_proxy_repr_png():
    """FigureProxy._repr_png_() returns PNG bytes."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    png = fig._repr_png_()
    assert isinstance(png, bytes)
    assert png[:8] == b'\x89PNG\r\n\x1a\n'  # PNG magic number
    plt.close()


def test_figure_proxy_repr_svg():
    """FigureProxy._repr_svg_() returns SVG string."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    svg = fig._repr_svg_()
    assert isinstance(svg, str)
    assert "<svg" in svg
    assert "</svg>" in svg
    plt.close()


def test_figure_proxy_repr_html():
    """FigureProxy._repr_html_() returns HTML img tag with base64 PNG."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    html = fig._repr_html_()
    assert isinstance(html, str)
    assert "<img" in html
    assert "data:image/png;base64," in html
    plt.close()


def test_backend_inline_figure_format():
    """backend_inline supports configurable figure_format."""
    from rustplotlib.backends import backend_inline
    # Default should be 'png'
    assert backend_inline.get_figure_format() == 'png'
    # Should accept valid formats
    backend_inline.set_figure_format('svg')
    assert backend_inline.get_figure_format() == 'svg'
    # Reset
    backend_inline.set_figure_format('png')


def test_backend_inline_display_svg():
    """display_figure uses SVG when figure_format is 'svg'."""
    from rustplotlib.backends import backend_inline

    displayed = []

    def fake_display(obj):
        displayed.append(obj)

    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])

    backend_inline.set_figure_format('svg')
    backend_inline.display_figure(fig._fig, display_func=fake_display)
    assert len(displayed) == 1
    # The displayed object should have _repr_svg_
    assert hasattr(displayed[0], '_repr_svg_')
    svg = displayed[0]._repr_svg_()
    assert '<svg' in svg

    backend_inline.set_figure_format('png')
    plt.close()
