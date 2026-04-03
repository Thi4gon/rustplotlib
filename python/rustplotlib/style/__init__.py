"""Style sheets for rustplotlib."""

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
}

available = list(_styles.keys())


def use(style_name):
    """Apply a named style to future plots."""
    from rustplotlib import pyplot
    if style_name not in _styles:
        raise ValueError(f"Unknown style: {style_name}. Available: {available}")
    style = _styles[style_name]
    pyplot.rcParams.update(style)
    pyplot._current_style = style_name
