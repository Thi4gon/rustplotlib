"""rustplotlib.pyplot — matplotlib-compatible plotting API powered by Rust."""

from rustplotlib._rustplotlib import RustFigure
import numpy as np

_current_figure = None
_current_axes_id = None

# Global rcParams dict (matplotlib compatibility — accepts any key/value)
rcParams = {
    'font.family': ['sans-serif'],
    'mathtext.fontset': 'dejavusans',
    'font.size': 10,
    'axes.labelsize': 12,
    'axes.titlesize': 14,
    'xtick.labelsize': 10,
    'ytick.labelsize': 10,
    'legend.fontsize': 10,
    'figure.figsize': [6.4, 4.8],
    'figure.dpi': 100,
}


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

    def set_xlim(self, left=None, right=None, xmin=None, xmax=None, **kwargs):
        left = left if left is not None else xmin
        right = right if right is not None else xmax
        if left is not None and right is not None:
            self._fig.axes_set_xlim(self._id, float(left), float(right))

    def set_ylim(self, bottom=None, top=None, ymin=None, ymax=None, **kwargs):
        bottom = bottom if bottom is not None else ymin
        top = top if top is not None else ymax
        if bottom is not None and top is not None:
            self._fig.axes_set_ylim(self._id, float(bottom), float(top))

    def legend(self, *args, **kwargs):
        kw = {}
        if 'loc' in kwargs:
            kw['loc'] = kwargs['loc']
        if 'prop' in kwargs:
            kw['prop'] = str(kwargs['prop'])  # pass as string, ignored on Rust side
        self._fig.axes_legend(self._id, kw)

    def grid(self, visible=True, **kwargs):
        kw = {}
        if 'color' in kwargs:
            kw['color'] = kwargs['color']
        if 'linewidth' in kwargs:
            kw['linewidth'] = float(kwargs['linewidth'])
        if 'alpha' in kwargs:
            kw['alpha'] = float(kwargs['alpha'])
        if 'linestyle' in kwargs:
            kw['linestyle'] = kwargs['linestyle']
        self._fig.axes_grid(self._id, bool(visible), kw)

    def text(self, x, y, s, **kwargs):
        kw = {}
        if 'fontsize' in kwargs:
            kw['fontsize'] = float(kwargs['fontsize'])
        if 'color' in kwargs:
            kw['color'] = kwargs['color']
        self._fig.axes_text(self._id, float(x), float(y), str(s), kw)

    def fill_between(self, x, y1, y2=0, alpha=0.3, color=None, label=None, **kwargs):
        x = _to_list(x)
        y1 = _to_list(y1)
        if isinstance(y2, (int, float)):
            y2 = [float(y2)] * len(x)
        else:
            y2 = _to_list(y2)
        kw = {"alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_fill_between(self._id, x, y1, y2, kw)
        return self

    def step(self, x, y, color=None, linewidth=None, linestyle=None,
             label=None, alpha=None, **kwargs):
        x, y = _to_list(x), _to_list(y)
        where_style = kwargs.pop("where", "pre")
        kw = {"where": where_style}
        if color is not None:
            kw["color"] = color
        if linewidth is not None:
            kw["linewidth"] = float(linewidth)
        if linestyle is not None:
            kw["linestyle"] = linestyle
        if label is not None:
            kw["label"] = label
        if alpha is not None:
            kw["alpha"] = float(alpha)
        self._fig.axes_step(self._id, x, y, kw)
        return self

    def pie(self, sizes, labels=None, colors=None, startangle=90, **kwargs):
        sizes = _to_list(sizes)
        kw = {"startangle": float(startangle)}
        if labels is not None:
            kw["labels"] = list(labels)
        if colors is not None:
            kw["colors"] = list(colors)
        self._fig.axes_pie(self._id, sizes, kw)
        return self

    def set_xscale(self, scale, **kwargs):
        self._fig.axes_set_xscale(self._id, str(scale))

    def set_yscale(self, scale, **kwargs):
        self._fig.axes_set_yscale(self._id, str(scale))

    def errorbar(self, x, y, yerr=None, xerr=None, color=None, linewidth=None,
                 capsize=3.0, marker=None, markersize=None, label=None,
                 alpha=None, linestyle=None, fmt=None, **kwargs):
        x, y = _to_list(x), _to_list(y)
        kw = {}
        if yerr is not None:
            kw["yerr"] = _to_list(yerr)
        if xerr is not None:
            kw["xerr"] = _to_list(xerr)
        if color is not None:
            kw["color"] = color
        if linewidth is not None:
            kw["linewidth"] = float(linewidth)
        if capsize is not None:
            kw["capsize"] = float(capsize)
        if marker is not None:
            kw["marker"] = marker
        if markersize is not None:
            kw["markersize"] = float(markersize)
        if label is not None:
            kw["label"] = label
        if alpha is not None:
            kw["alpha"] = float(alpha)
        if linestyle is not None:
            kw["linestyle"] = linestyle
        self._fig.axes_errorbar(self._id, x, y, kw)
        return self

    def barh(self, y, width, height=0.8, color=None, label=None, alpha=1.0, **kwargs):
        y, width = _to_list(y), _to_list(width)
        kw = {"height": float(height), "alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_barh(self._id, y, width, kw)
        return self

    def boxplot(self, data, positions=None, widths=None, color=None,
                median_color=None, **kwargs):
        # data can be a list of lists or a single list (single box)
        if data and not isinstance(data[0], (list, tuple)):
            if hasattr(data[0], '__iter__'):
                data_list = [_to_list(d) for d in data]
            else:
                data_list = [_to_list(data)]
        else:
            data_list = [_to_list(d) for d in data]
        kw = {}
        if positions is not None:
            kw["positions"] = [float(p) for p in positions]
        if widths is not None:
            kw["widths"] = float(widths)
        if color is not None:
            kw["color"] = color
        if median_color is not None:
            kw["median_color"] = median_color
        self._fig.axes_boxplot(self._id, data_list, kw)
        return self

    def stem(self, *args, color=None, linewidth=None, marker=None,
             markersize=None, label=None, baseline=0.0, **kwargs):
        if len(args) == 1:
            y = _to_list(args[0])
            x = list(range(len(y)))
        elif len(args) >= 2:
            x = _to_list(args[0])
            y = _to_list(args[1])
        else:
            raise ValueError("stem requires at least one positional argument")
        kw = {"baseline": float(baseline)}
        if color is not None:
            kw["color"] = color
        if linewidth is not None:
            kw["linewidth"] = float(linewidth)
        if marker is not None:
            kw["marker"] = marker
        if markersize is not None:
            kw["markersize"] = float(markersize)
        if label is not None:
            kw["label"] = label
        self._fig.axes_stem(self._id, x, y, kw)
        return self

    def axhline(self, y=0, color=None, linestyle="--", linewidth=1.0, alpha=1.0, **kwargs):
        kw = {"linestyle": linestyle, "linewidth": float(linewidth), "alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        self._fig.axes_axhline(self._id, float(y), kw)
        return self

    def axvline(self, x=0, color=None, linestyle="--", linewidth=1.0, alpha=1.0, **kwargs):
        kw = {"linestyle": linestyle, "linewidth": float(linewidth), "alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        self._fig.axes_axvline(self._id, float(x), kw)
        return self

    def add_patch(self, patch):
        pass  # Stub for patches.Rectangle etc.


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


def fill_between(x, y1, y2=0, **kwargs):
    _gca().fill_between(x, y1, y2, **kwargs)


def step(x, y, **kwargs):
    _gca().step(x, y, **kwargs)


def pie(sizes, **kwargs):
    _gca().pie(sizes, **kwargs)


def xscale(scale, **kwargs):
    _gca().set_xscale(scale, **kwargs)


def yscale(scale, **kwargs):
    _gca().set_yscale(scale, **kwargs)


def errorbar(x, y, **kwargs):
    _gca().errorbar(x, y, **kwargs)


def barh(y, width, **kwargs):
    _gca().barh(y, width, **kwargs)


def boxplot(data, **kwargs):
    _gca().boxplot(data, **kwargs)


def stem(*args, **kwargs):
    _gca().stem(*args, **kwargs)


def axhline(y=0, **kwargs):
    _gca().axhline(y, **kwargs)


def axvline(x=0, **kwargs):
    _gca().axvline(x, **kwargs)


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


def text(x, y, s, **kwargs):
    _gca().text(x, y, s, **kwargs)


def xticks(*args, **kwargs):
    pass  # Accept fontsize, ticks, labels, etc.


def yticks(*args, **kwargs):
    pass  # Accept fontsize, ticks, labels, etc.


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
