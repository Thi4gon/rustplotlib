"""Jupyter inline display backend for rustplotlib."""

import base64


def display_figure(fig):
    """Display a figure inline in Jupyter.

    Parameters
    ----------
    fig : RustFigure
        The Rust figure object (not the FigureProxy).
    """
    try:
        from IPython.display import display, Image
        png_bytes = fig.render_to_png_bytes()
        display(Image(data=bytes(png_bytes)))
    except ImportError:
        pass


def configure_inline_support(shell=None, backend=None):
    """Configure Jupyter to display rustplotlib figures inline."""
    pass
