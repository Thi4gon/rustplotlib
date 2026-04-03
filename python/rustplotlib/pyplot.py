"""rustplotlib.pyplot — matplotlib-compatible plotting API powered by Rust."""

from rustplotlib._rustplotlib import RustFigure
import numpy as np

_current_figure = None
_current_axes_id = None


class AxesProxy:
    """Python wrapper around a Rust axes, accessed by ID."""

    def __init__(self, figure, ax_id):
        self._fig = figure
        self._id = ax_id

    def plot(self, *args, **kwargs):
        x, y, kwargs = _parse_plot_args(*args, **kwargs)
        self._fig.axes_plot(self._id, x, y, kwargs)
        return self

    def scatter(self, x, y, s=None, c=None, marker="o", alpha=1.0, label=None, **kwargs):
        x, y = _to_list(x), _to_list(y)
        kw = {"marker": marker, "alpha": alpha}
        if s is not None:
            kw["s"] = list(np.atleast_1d(s).astype(float))
        if c is not None:
            kw["color"] = c
        if label is not None:
            kw["label"] = label
        self._fig.axes_scatter(self._id, x, y, kw)
        return self

    def bar(self, x, height, width=0.8, color=None, label=None, alpha=1.0, **kwargs):
        x, height = _to_list(x), _to_list(height)
        kw = {"width": width, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_bar(self._id, x, height, kw)
        return self

    def hist(self, x, bins=10, color=None, alpha=1.0, label=None, **kwargs):
        x = _to_list(x)
        kw = {"bins": bins, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_hist(self._id, x, kw)
        return self

    def imshow(self, data, cmap="viridis", aspect=None, vmin=None, vmax=None, **kwargs):
        data_list = _to_2d_list(data)
        kw = {"cmap": cmap}
        if vmin is not None:
            kw["vmin"] = vmin
        if vmax is not None:
            kw["vmax"] = vmax
        self._fig.axes_imshow(self._id, data_list, kw)
        return self

    def set_title(self, title, fontsize=None, **kwargs):
        self._fig.axes_set_title(self._id, str(title), fontsize)

    def set_xlabel(self, label, fontsize=None, **kwargs):
        self._fig.axes_set_xlabel(self._id, str(label), fontsize)

    def set_ylabel(self, label, fontsize=None, **kwargs):
        self._fig.axes_set_ylabel(self._id, str(label), fontsize)

    def set_xlim(self, left=None, right=None, **kwargs):
        if left is not None and right is not None:
            self._fig.axes_set_xlim(self._id, float(left), float(right))

    def set_ylim(self, bottom=None, top=None, **kwargs):
        if bottom is not None and top is not None:
            self._fig.axes_set_ylim(self._id, float(bottom), float(top))

    def legend(self, *args, **kwargs):
        self._fig.axes_legend(self._id)

    def grid(self, visible=True, **kwargs):
        self._fig.axes_grid(self._id, visible)


class FigureProxy:
    """Python wrapper around RustFigure."""

    def __init__(self, rust_fig, axes_proxies):
        self._fig = rust_fig
        self._axes = axes_proxies

    def savefig(self, fname, dpi=None, format=None, bbox_inches=None, **kwargs):
        self._fig.savefig(str(fname))

    def set_size_inches(self, w, h=None):
        if h is None and hasattr(w, "__iter__"):
            w, h = w
        self._fig.set_size_inches(float(w), float(h))

    def tight_layout(self, **kwargs):
        pass

    def show(self):
        self._fig.show()


def _to_list(data):
    if isinstance(data, np.ndarray):
        return data.astype(float).flatten().tolist()
    if isinstance(data, (list, tuple)):
        return [float(v) for v in data]
    return [float(data)]


def _to_2d_list(data):
    if isinstance(data, np.ndarray):
        return data.astype(float).tolist()
    return [[float(v) for v in row] for row in data]


def _parse_plot_args(*args, **kwargs):
    """Parse matplotlib-style plot arguments: plot(y), plot(x, y), plot(x, y, fmt)."""
    x = None
    y = None
    fmt = None

    plain_args = list(args)

    if len(plain_args) == 1:
        y = _to_list(plain_args[0])
        x = list(range(len(y)))
    elif len(plain_args) == 2:
        if isinstance(plain_args[1], str):
            y = _to_list(plain_args[0])
            x = list(range(len(y)))
            fmt = plain_args[1]
        else:
            x = _to_list(plain_args[0])
            y = _to_list(plain_args[1])
    elif len(plain_args) >= 3:
        x = _to_list(plain_args[0])
        y = _to_list(plain_args[1])
        if isinstance(plain_args[2], str):
            fmt = plain_args[2]

    if fmt:
        _parse_fmt(fmt, kwargs)

    # Ensure x values are float for Rust
    x = [float(v) for v in x]

    return x, y, kwargs


def _parse_fmt(fmt, kwargs):
    """Parse matplotlib format string like 'r--o' into color, linestyle, marker."""
    color_chars = {"r", "g", "b", "c", "m", "y", "k", "w"}
    marker_chars = {".", "o", "s", "^", "v", "+", "x", "D", "*"}
    remaining = fmt

    if remaining and remaining[0] in color_chars:
        if "color" not in kwargs:
            kwargs["color"] = remaining[0]
        remaining = remaining[1:]

    for ls in ["--", "-.", ":", "-"]:
        if ls in remaining:
            if "linestyle" not in kwargs:
                kwargs["linestyle"] = ls
            remaining = remaining.replace(ls, "", 1)
            break

    for ch in remaining:
        if ch in marker_chars:
            if "marker" not in kwargs:
                kwargs["marker"] = ch
            break


def _ensure_figure():
    global _current_figure, _current_axes_id
    if _current_figure is None:
        _current_figure = RustFigure(640, 480, 100)
        _current_axes_id = _current_figure.add_axes()


def _gcf():
    _ensure_figure()
    return _current_figure


def _gca_id():
    _ensure_figure()
    return _current_axes_id


def _gca():
    return AxesProxy(_gcf(), _gca_id())


# --- Public API ---


def figure(figsize=None, dpi=100, **kwargs):
    global _current_figure, _current_axes_id
    if figsize:
        w, h = figsize
        _current_figure = RustFigure(int(w * dpi), int(h * dpi), dpi)
    else:
        _current_figure = RustFigure(640, 480, dpi)
    _current_axes_id = _current_figure.add_axes()
    return FigureProxy(_current_figure, [_gca()])


def subplots(nrows=1, ncols=1, figsize=None, dpi=100, **kwargs):
    global _current_figure, _current_axes_id
    if figsize:
        w, h = figsize
        fig = RustFigure(int(w * dpi), int(h * dpi), dpi)
    else:
        fig = RustFigure(640, 480, dpi)
    fig.setup_subplots(nrows, ncols)
    _current_figure = fig
    _current_axes_id = 0

    if nrows == 1 and ncols == 1:
        axes = AxesProxy(fig, 0)
    elif nrows == 1 or ncols == 1:
        n = max(nrows, ncols)
        axes = [AxesProxy(fig, i) for i in range(n)]
    else:
        axes = []
        for r in range(nrows):
            row = [AxesProxy(fig, r * ncols + c) for c in range(ncols)]
            axes.append(row)

    return FigureProxy(fig, axes), axes


def plot(*args, **kwargs):
    _gca().plot(*args, **kwargs)


def scatter(x, y, **kwargs):
    _gca().scatter(x, y, **kwargs)


def bar(x, height, **kwargs):
    _gca().bar(x, height, **kwargs)


def hist(x, **kwargs):
    _gca().hist(x, **kwargs)


def imshow(data, **kwargs):
    _gca().imshow(data, **kwargs)


def title(text, **kwargs):
    _gca().set_title(text, **kwargs)


def xlabel(text, **kwargs):
    _gca().set_xlabel(text, **kwargs)


def ylabel(text, **kwargs):
    _gca().set_ylabel(text, **kwargs)


def xlim(*args, **kwargs):
    if len(args) == 2:
        _gca().set_xlim(args[0], args[1])


def ylim(*args, **kwargs):
    if len(args) == 2:
        _gca().set_ylim(args[0], args[1])


def legend(*args, **kwargs):
    _gca().legend(*args, **kwargs)


def grid(visible=True, **kwargs):
    _gca().grid(visible, **kwargs)


def tight_layout(**kwargs):
    pass


def savefig(fname, **kwargs):
    _gcf().savefig(str(fname))


def show():
    _gcf().show()


def close(*args):
    global _current_figure, _current_axes_id
    _current_figure = None
    _current_axes_id = None
