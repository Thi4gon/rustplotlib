"""Jupyter inline display backend for rustplotlib."""

import base64

_figure_format = 'png'


def get_figure_format():
    """Get the current inline figure format ('png' or 'svg')."""
    return _figure_format


def set_figure_format(fmt):
    """Set the inline figure format ('png', 'svg', or 'retina')."""
    if fmt not in ('png', 'svg', 'retina'):
        raise ValueError(f"Unsupported figure format: {fmt!r}. Use 'png', 'svg', or 'retina'.")
    global _figure_format
    _figure_format = fmt


class SVGWrapper:
    """Wrapper for SVG string display in Jupyter."""

    def __init__(self, data):
        self.data = data

    def _repr_svg_(self):
        return self.data


class PNGWrapper:
    """Wrapper for PNG bytes display in Jupyter."""

    def __init__(self, data):
        self.data = data

    def _repr_png_(self):
        return self.data


def display_figure(fig, display_func=None):
    """Display a figure inline in Jupyter.

    Parameters
    ----------
    fig : RustFigure
        The Rust figure object (not the FigureProxy).
    display_func : callable, optional
        Custom display function (for testing). Defaults to IPython.display.display.
    """
    if display_func is None:
        try:
            from IPython.display import display as display_func
        except ImportError:
            return

    fmt = get_figure_format()

    if fmt == 'svg':
        svg_str = fig.render_to_svg_string()
        display_func(SVGWrapper(svg_str))
    else:
        # 'png' or 'retina'
        png_bytes = fig.render_to_png_bytes()
        display_func(PNGWrapper(bytes(png_bytes)))


def configure_inline_support(shell=None, backend=None):
    """Configure Jupyter to display rustplotlib figures inline."""
    pass
