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
