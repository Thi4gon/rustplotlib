"""Style sheets for rustplotlib."""

from contextlib import contextmanager

_styles = {
    "default": {
        "figure.facecolor": "white",
        "axes.facecolor": "white",
        "text.color": "black",
        "axes.edgecolor": "black",
        "xtick.color": "black",
        "ytick.color": "black",
        "grid.alpha": 0.3,
    },
    "dark_background": {
        "figure.facecolor": "#1C1C1C",
        "axes.facecolor": "#1C1C1C",
        "text.color": "white",
        "axes.edgecolor": "white",
        "xtick.color": "white",
        "ytick.color": "white",
        "grid.alpha": 0.2,
    },
    "ggplot": {
        "figure.facecolor": "white",
        "axes.facecolor": "#E5E5E5",
        "text.color": "#333333",
        "axes.edgecolor": "white",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.alpha": 0.8,
    },
    "seaborn": {
        "figure.facecolor": "white",
        "axes.facecolor": "#EAEAF2",
        "text.color": "#333333",
        "axes.edgecolor": "white",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.alpha": 0.8,
    },
    "bmh": {
        "figure.facecolor": "white",
        "axes.facecolor": "#eeeeee",
        "text.color": "#333333",
        "axes.edgecolor": "black",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.alpha": 0.5,
    },
    "fivethirtyeight": {
        "figure.facecolor": "#F0F0F0",
        "axes.facecolor": "#F0F0F0",
        "text.color": "#3C3C3C",
        "axes.edgecolor": "#F0F0F0",
        "xtick.color": "#3C3C3C",
        "ytick.color": "#3C3C3C",
        "grid.alpha": 0.4,
    },
    # --- New styles ---
    "Solarize_Light2": {
        "figure.facecolor": "#FDF6E3",
        "axes.facecolor": "#EEE8D5",
        "text.color": "#657B83",
        "axes.edgecolor": "#93A1A1",
        "xtick.color": "#657B83",
        "ytick.color": "#657B83",
        "grid.color": "#93A1A1",
        "grid.alpha": 0.5,
    },
    "grayscale": {
        "figure.facecolor": "white",
        "axes.facecolor": "white",
        "text.color": "#222222",
        "axes.edgecolor": "#222222",
        "xtick.color": "#222222",
        "ytick.color": "#222222",
        "grid.color": "#999999",
        "grid.alpha": 0.4,
        "lines.color": "#444444",
    },
    "tableau-colorblind10": {
        "figure.facecolor": "white",
        "axes.facecolor": "white",
        "text.color": "#333333",
        "axes.edgecolor": "#333333",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.color": "#CCCCCC",
        "grid.alpha": 0.5,
    },
    "seaborn-v0_8-whitegrid": {
        "figure.facecolor": "white",
        "axes.facecolor": "white",
        "text.color": "#333333",
        "axes.edgecolor": "white",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.color": "#CCCCCC",
        "grid.alpha": 0.8,
    },
    "seaborn-v0_8-darkgrid": {
        "figure.facecolor": "white",
        "axes.facecolor": "#EAEAF2",
        "text.color": "#333333",
        "axes.edgecolor": "white",
        "xtick.color": "#333333",
        "ytick.color": "#333333",
        "grid.color": "white",
        "grid.alpha": 0.9,
    },
    "seaborn-v0_8-dark": {
        "figure.facecolor": "#2D2D2D",
        "axes.facecolor": "#2D2D2D",
        "text.color": "#CCCCCC",
        "axes.edgecolor": "#555555",
        "xtick.color": "#CCCCCC",
        "ytick.color": "#CCCCCC",
        "grid.color": "#444444",
        "grid.alpha": 0.5,
    },
    "fast": {
        "figure.facecolor": "white",
        "axes.facecolor": "white",
        "text.color": "black",
        "axes.edgecolor": "black",
        "xtick.color": "black",
        "ytick.color": "black",
        "grid.alpha": 0.0,
    },
}

# Aliases for seaborn styles (matplotlib compatibility)
_aliases = {
    "seaborn-whitegrid": "seaborn-v0_8-whitegrid",
    "seaborn-darkgrid": "seaborn-v0_8-darkgrid",
    "seaborn-dark": "seaborn-v0_8-dark",
}

available = list(_styles.keys()) + list(_aliases.keys())


def use(style_name):
    """Apply a named style to future plots."""
    from rustplotlib import pyplot
    # Resolve alias first
    resolved = _aliases.get(style_name, style_name)
    if resolved not in _styles:
        raise ValueError(f"Unknown style: {style_name}. Available: {available}")
    style_dict = _styles[resolved]
    pyplot.rcParams.update(style_dict)
    pyplot._current_style = style_name


@contextmanager
def context(style_name):
    """Context manager to temporarily apply a style.

    Usage::

        with style.context('dark_background'):
            ax.plot(...)
    """
    from rustplotlib import pyplot
    # Save current rcParams snapshot
    saved = dict(pyplot.rcParams)
    saved_style = getattr(pyplot, '_current_style', 'default')
    try:
        use(style_name)
        yield
    finally:
        pyplot.rcParams.clear()
        pyplot.rcParams.update(saved)
        pyplot._current_style = saved_style
