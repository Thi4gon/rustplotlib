"""Backend system for rustplotlib.

Manages backend selection and auto-detection:
- 'inline' — Jupyter/IPython (auto-detected)
- 'tk' — Tkinter interactive window (default for scripts)
- 'agg' — Non-interactive, save-only (fallback)
"""

_current_backend = None  # None = auto-detect

_BACKEND_REGISTRY = {
    'agg': 'rustplotlib.backends.backend_base',
    'inline': 'rustplotlib.backends.backend_inline',
    'tk': 'rustplotlib.backends.backend_tk',
}

from rustplotlib.backends.backend_pdf import PdfPages


def _auto_detect():
    """Auto-detect the best backend for the current environment."""
    try:
        get_ipython()  # noqa: F821
        return 'inline'
    except NameError:
        pass
    return 'agg'  # Default to non-interactive


def get_backend():
    """Get the current backend name."""
    global _current_backend
    if _current_backend is None:
        _current_backend = _auto_detect()
    return _current_backend


def set_backend(name):
    """Set the backend by name."""
    global _current_backend
    name = name.lower()
    if name == 'tkagg':
        name = 'tk'
    _current_backend = name


def list_backends():
    """Return list of registered backend names."""
    return list(_BACKEND_REGISTRY.keys())


def show_figure(figure_proxy):
    """Show a figure using the current backend.

    Parameters
    ----------
    figure_proxy : FigureProxy
        The figure to show.
    """
    backend = get_backend()

    if backend == 'inline':
        from rustplotlib.backends.backend_inline import display_figure
        display_figure(figure_proxy._fig)
    elif backend == 'tk':
        from rustplotlib.backends.backend_tk import FigureCanvasTk, FigureManagerTk
        canvas = FigureCanvasTk(figure_proxy)
        manager = FigureManagerTk(canvas, 1)
        manager.show()
    else:
        # agg — non-interactive, use system viewer via Rust
        figure_proxy._fig.show()
