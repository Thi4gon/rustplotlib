"""rustplotlib.pyplot — matplotlib-compatible plotting API powered by Rust."""

from rustplotlib._rustplotlib import RustFigure
import numpy as np

_current_figure = None
_current_axes_id = None
_current_style = "default"

# Global rcParams dict (matplotlib compatibility — accepts any key/value)
rcParams = {
    'figure.figsize': [6.4, 4.8],
    'figure.dpi': 100,
    'figure.facecolor': 'white',
    'figure.edgecolor': 'white',
    'axes.facecolor': 'white',
    'axes.edgecolor': 'black',
    'axes.linewidth': 0.8,
    'axes.grid': False,
    'axes.titlesize': 14,
    'axes.labelsize': 12,
    'text.color': 'black',
    'font.family': ['sans-serif'],
    'font.size': 10.0,
    'xtick.color': 'black',
    'ytick.color': 'black',
    'xtick.labelsize': 10,
    'ytick.labelsize': 10,
    'grid.color': '#b0b0b0',
    'grid.linestyle': '-',
    'grid.linewidth': 0.8,
    'grid.alpha': 1.0,
    'legend.fontsize': 10,
    'legend.frameon': True,
    'legend.loc': 'best',
    'lines.linewidth': 1.5,
    'lines.markersize': 6.0,
    'savefig.dpi': 'figure',
    'savefig.transparent': False,
    'mathtext.fontset': 'dejavusans',
    'image.cmap': 'viridis',
}


class SpineProxy:
    """Proxy for a single spine (top/right/bottom/left)."""

    def __init__(self, figure, ax_id, which):
        self._fig = figure
        self._id = ax_id
        self._which = which

    def set_visible(self, visible):
        self._fig.axes_set_spine_visible(self._id, self._which, visible)

    def set_color(self, color):
        pass

    def set_linewidth(self, lw):
        pass

    def set_lw(self, lw):
        pass


class SpinesProxy:
    """Proxy for all spines of an axes."""

    def __init__(self, figure, ax_id):
        self._fig = figure
        self._id = ax_id

    def __getitem__(self, key):
        return SpineProxy(self._fig, self._id, key)


class AxisProxy:
    """Stub proxy for a single axis (x or y) — matplotlib.axis.Axis compatibility."""

    def set_major_formatter(self, formatter):
        pass

    def set_minor_formatter(self, formatter):
        pass

    def set_major_locator(self, locator):
        pass

    def set_minor_locator(self, locator):
        pass

    def set_visible(self, b):
        pass

    def set_ticks_position(self, position):
        pass

    def set_label_position(self, position):
        pass

    def get_major_ticks(self):
        return []

    def get_minor_ticks(self):
        return []

    def set_tick_params(self, **kwargs):
        pass


class Line2DProxy:
    """Proxy for matplotlib Line2D objects returned by plot()."""

    def __init__(self):
        self._color = 'blue'
        self._label = ''
        self._linewidth = 1.5
        self._linestyle = '-'

    def set_data(self, x, y):
        pass  # stub for animation compat

    def set_color(self, c):
        self._color = c

    def set_linewidth(self, lw):
        self._linewidth = lw

    def set_linestyle(self, ls):
        self._linestyle = ls

    def set_label(self, label):
        self._label = label

    def get_color(self):
        return self._color

    def get_label(self):
        return self._label

    def get_linewidth(self):
        return self._linewidth

    def get_linestyle(self):
        return self._linestyle

    def remove(self):
        pass

    def set_visible(self, b):
        pass

    def set_xdata(self, x):
        pass

    def set_ydata(self, y):
        pass

    def get_xdata(self):
        return []

    def get_ydata(self):
        return []

    def set_marker(self, m):
        pass

    def set_markersize(self, s):
        pass

    def set_alpha(self, a):
        pass


class PathCollectionProxy:
    """Proxy for matplotlib PathCollection returned by scatter()."""

    def __init__(self):
        pass

    def set_offsets(self, offsets):
        pass

    def set_array(self, a):
        pass

    def set_clim(self, vmin, vmax):
        pass

    def set_cmap(self, cmap):
        pass

    def set_sizes(self, s):
        pass

    def set_color(self, c):
        pass

    def set_alpha(self, a):
        pass

    def set_visible(self, b):
        pass

    def remove(self):
        pass

    def get_offsets(self):
        return []


class BarContainerProxy:
    """Proxy for matplotlib BarContainer returned by bar()."""

    def __init__(self, patches=None):
        self.patches = patches or []

    def __iter__(self):
        return iter(self.patches)

    def __len__(self):
        return len(self.patches)

    def remove(self):
        pass

    def set_visible(self, b):
        pass


class AxesProxy:
    """Python wrapper around a Rust axes, accessed by ID."""

    def __init__(self, figure, ax_id):
        self._fig = figure
        self._id = ax_id

    def plot(self, *args, **kwargs):
        x, y, kwargs = _parse_plot_args(*args, **kwargs)
        # Handle categorical (string) x values
        if x and isinstance(x[0], str):
            positions, labels = _handle_categorical(x)
            x = [float(p) for p in positions]
            self._fig.axes_set_xticks(self._id, x)
            self._fig.axes_set_xticklabels(self._id, labels)
        self._fig.axes_plot(self._id, x, y, kwargs)
        proxy = Line2DProxy()
        if 'color' in kwargs:
            proxy._color = kwargs['color']
        if 'label' in kwargs:
            proxy._label = kwargs['label']
        if 'linewidth' in kwargs:
            proxy._linewidth = kwargs['linewidth']
        if 'linestyle' in kwargs:
            proxy._linestyle = kwargs['linestyle']
        return [proxy]

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
        return PathCollectionProxy()

    def bar(self, x, height, width=0.8, bottom=None, color=None, label=None, alpha=1.0, **kwargs):
        # Handle categorical (string) x values
        cat_labels = None
        if x and isinstance(x[0], str):
            positions, cat_labels = _handle_categorical(x)
            x = [float(p) for p in positions]
        else:
            x = _to_list(x)
        height = _to_list(height)
        kw = {"width": width, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        if bottom is not None:
            kw["bottom"] = float(bottom)
        if cat_labels is not None:
            self._fig.axes_set_xticks(self._id, x)
            self._fig.axes_set_xticklabels(self._id, cat_labels)
        self._fig.axes_bar(self._id, x, height, kw)
        return BarContainerProxy()

    def hist(self, x, bins=10, color=None, alpha=1.0, label=None, **kwargs):
        x = _to_list(x)
        kw = {"bins": bins, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_hist(self._id, x, kw)
        return self

    def imshow(self, data, cmap="viridis", aspect=None, vmin=None, vmax=None,
               annotate=False, fmt=None, interpolation=None, **kwargs):
        data_list = _to_2d_list(data)
        kw = {"cmap": cmap}
        if vmin is not None:
            kw["vmin"] = vmin
        if vmax is not None:
            kw["vmax"] = vmax
        if annotate:
            kw["annotate"] = True
        if fmt is not None:
            kw["fmt"] = str(fmt)
        if interpolation is not None:
            kw["interpolation"] = str(interpolation)
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
        if 'ncol' in kwargs:
            kw['ncol'] = int(kwargs['ncol'])
        if 'prop' in kwargs:
            kw['prop'] = str(kwargs['prop'])  # pass as string, ignored on Rust side
        self._fig.axes_legend(self._id, kw)

    def grid(self, visible=True, which='major', **kwargs):
        kw = {"which": which}
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

    def annotate(self, text, xy, xytext=None, arrowprops=None, fontsize=None,
                 color=None, **kwargs):
        if xytext is None:
            xytext = xy
        kw = {}
        if fontsize is not None:
            kw["fontsize"] = float(fontsize)
        if color is not None:
            kw["color"] = color
        if arrowprops is not None:
            kw["arrowprops"] = dict(arrowprops)
        xy_tuple = (float(xy[0]), float(xy[1]))
        xytext_tuple = (float(xytext[0]), float(xytext[1]))
        self._fig.axes_annotate(self._id, str(text), xy_tuple, xytext_tuple, kw)
        return self

    def axis(self, arg):
        if arg == 'off':
            self._fig.axes_set_axis_off(self._id, True)
        elif arg == 'on':
            self._fig.axes_set_axis_off(self._id, False)

    def set_xticks(self, ticks, labels=None, **kwargs):
        self._fig.axes_set_xticks(self._id, [float(t) for t in ticks])
        if labels is not None:
            self._fig.axes_set_xticklabels(self._id, [str(l) for l in labels])

    def set_yticks(self, ticks, labels=None, **kwargs):
        self._fig.axes_set_yticks(self._id, [float(t) for t in ticks])
        if labels is not None:
            self._fig.axes_set_yticklabels(self._id, [str(l) for l in labels])

    def set_xticklabels(self, labels, **kwargs):
        self._fig.axes_set_xticklabels(self._id, [str(l) for l in labels])

    def set_yticklabels(self, labels, **kwargs):
        self._fig.axes_set_yticklabels(self._id, [str(l) for l in labels])

    def set_aspect(self, aspect, **kwargs):
        self._fig.axes_set_aspect(self._id, str(aspect))

    def invert_xaxis(self):
        self._fig.axes_invert_xaxis(self._id)

    def invert_yaxis(self):
        self._fig.axes_invert_yaxis(self._id)

    def add_patch(self, patch):
        """Add a patch object (Rectangle, Circle, Polygon) to the axes."""
        from rustplotlib.patches import Rectangle, Circle, Polygon
        kw = {}
        if hasattr(patch, 'facecolor') and patch.facecolor is not None:
            kw['facecolor'] = patch.facecolor
        if hasattr(patch, 'edgecolor') and patch.edgecolor is not None:
            kw['edgecolor'] = patch.edgecolor
        if hasattr(patch, 'linewidth'):
            kw['linewidth'] = float(getattr(patch, 'linewidth', 1.0))
        if hasattr(patch, 'alpha') and patch.alpha is not None:
            kw['alpha'] = float(patch.alpha)
        if hasattr(patch, 'label') and patch.label is not None:
            kw['label'] = str(patch.label)

        if isinstance(patch, Rectangle):
            kw['x'] = float(patch.xy[0])
            kw['y'] = float(patch.xy[1])
            kw['width'] = float(patch.width)
            kw['height'] = float(patch.height)
            self._fig.axes_add_patch(self._id, "rectangle", kw)
        elif isinstance(patch, Circle):
            kw['cx'] = float(patch.xy[0])
            kw['cy'] = float(patch.xy[1])
            kw['radius'] = float(patch.radius)
            self._fig.axes_add_patch(self._id, "circle", kw)
        elif isinstance(patch, Polygon):
            kw['points'] = [(float(p[0]), float(p[1])) for p in patch.xy]
            self._fig.axes_add_patch(self._id, "polygon", kw)
        # else: silently ignore unknown patch types
        return self

    def axhspan(self, ymin, ymax, color=None, alpha=0.3, **kwargs):
        kw = {"alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        self._fig.axes_axhspan(self._id, float(ymin), float(ymax), kw)
        return self

    def axvspan(self, xmin, xmax, color=None, alpha=0.3, **kwargs):
        kw = {"alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        self._fig.axes_axvspan(self._id, float(xmin), float(xmax), kw)
        return self

    def contour(self, *args, levels=None, linewidth=1.0, **kwargs):
        x, y, z = _parse_contour_args(*args)
        kw = {"linewidth": float(linewidth)}
        if levels is not None:
            kw["levels"] = [float(l) for l in levels]
        self._fig.axes_contour(self._id, x, y, z, kw)
        return self

    def contourf(self, *args, levels=None, **kwargs):
        x, y, z = _parse_contour_args(*args)
        kw = {}
        if levels is not None:
            kw["levels"] = [float(l) for l in levels]
        self._fig.axes_contourf(self._id, x, y, z, kw)
        return self

    def hexbin(self, x, y, gridsize=20, cmap="viridis", mincnt=1, **kwargs):
        x, y = _to_list(x), _to_list(y)
        kw = {"gridsize": int(gridsize), "cmap": str(cmap), "mincnt": int(mincnt)}
        self._fig.axes_hexbin(self._id, x, y, kw)
        return self

    def colorbar(self, mappable=None, cmap="viridis", vmin=0.0, vmax=1.0, **kwargs):
        """Add a colorbar to this axes."""
        # Try to extract cmap/vmin/vmax from the mappable if provided
        if mappable is not None:
            if hasattr(mappable, 'cmap'):
                cmap = mappable.cmap
            if hasattr(mappable, 'vmin') and mappable.vmin is not None:
                vmin = mappable.vmin
            if hasattr(mappable, 'vmax') and mappable.vmax is not None:
                vmax = mappable.vmax
        kw = {"cmap": str(cmap), "vmin": float(vmin), "vmax": float(vmax)}
        self._fig.axes_colorbar(self._id, kw)
        return self

    def quiver(self, *args, color=None, scale=None, width=None, **kwargs):
        """Draw arrows at (x, y) with direction (u, v)."""
        if len(args) == 4:
            x, y, u, v = args
        elif len(args) == 2:
            u, v = args
            nr = len(u) if hasattr(u, '__len__') else 1
            nc = len(u[0]) if hasattr(u, '__len__') and hasattr(u[0], '__len__') else len(u)
            x = list(range(nc))
            y = list(range(nr))
            # flatten for 1D
        else:
            raise ValueError("quiver requires 2 or 4 positional arguments")
        x, y = _to_list(x), _to_list(y)
        u, v = _to_list(u), _to_list(v)
        kw = {}
        if color is not None:
            kw["color"] = color
        if scale is not None:
            kw["scale"] = float(scale)
        if width is not None:
            kw["width"] = float(width)
        self._fig.axes_quiver(self._id, x, y, u, v, kw)
        return self

    def streamplot(self, x, y, u, v, color=None, density=1.0, linewidth=1.0, **kwargs):
        """Draw streamlines from vector field (u, v) on grid (x, y)."""
        x_2d = _to_2d_list(x) if hasattr(x[0], '__len__') else [_to_list(x)]
        y_2d = _to_2d_list(y) if hasattr(y[0], '__len__') else [_to_list(y)]
        u_2d = _to_2d_list(u)
        v_2d = _to_2d_list(v)
        # If x and y are 1D, expand them into a meshgrid-like 2D format
        if len(x_2d) == 1 and len(u_2d) > 1:
            x_row = x_2d[0]
            y_col = y_2d[0] if len(y_2d) == 1 else [row[0] for row in y_2d]
            x_2d = [list(x_row) for _ in range(len(u_2d))]
            y_2d = [[yv] * len(x_row) for yv in y_col]
        kw = {"density": float(density), "linewidth": float(linewidth)}
        if color is not None:
            kw["color"] = color
        self._fig.axes_streamplot(self._id, x_2d, y_2d, u_2d, v_2d, kw)
        return self

    def set(self, **kwargs):
        """Set multiple axes properties at once.

        Example: ax.set(xlabel='X', ylabel='Y', title='My Plot', xlim=(0, 10))
        """
        for key, val in kwargs.items():
            method = getattr(self, f'set_{key}', None)
            if method:
                if isinstance(val, (list, tuple)) and key in ('xlim', 'ylim'):
                    method(*val)
                else:
                    method(val)

    def twinx(self):
        twin_id = self._fig.axes_twinx(self._id)
        return TwinAxesProxy(self._fig, twin_id)

    def twiny(self):
        twin_id = self._fig.axes_twiny(self._id)
        return TwinXAxesProxy(self._fig, twin_id)

    @property
    def spines(self):
        return SpinesProxy(self._fig, self._id)

    def tick_params(self, axis='both', direction='out', length=5.0, width=1.0,
                    labelsize=None, **kwargs):
        kw = {}
        if direction:
            kw['direction'] = direction
        if length:
            kw['length'] = float(length)
        if width:
            kw['width'] = float(width)
        if labelsize:
            kw['labelsize'] = float(labelsize)
        if 'color' in kwargs:
            kw['color'] = kwargs['color']
        self._fig.axes_tick_params(self._id, kw)

    def set_facecolor(self, color):
        self._fig.axes_set_facecolor(self._id, color)

    def hlines(self, y, xmin, xmax, colors=None, linestyles='-', linewidth=1.0, alpha=1.0, **kwargs):
        """Draw horizontal lines at each y from xmin to xmax."""
        if isinstance(y, (int, float)):
            y = [float(y)]
        else:
            y = _to_list(y)
        kw = {"linestyle": linestyles if isinstance(linestyles, str) else "-",
              "linewidth": float(linewidth), "alpha": float(alpha)}
        if colors is not None:
            kw["color"] = colors
        self._fig.axes_hlines(self._id, y, float(xmin), float(xmax), kw)
        return self

    def vlines(self, x, ymin, ymax, colors=None, linestyles='-', linewidth=1.0, alpha=1.0, **kwargs):
        """Draw vertical lines at each x from ymin to ymax."""
        if isinstance(x, (int, float)):
            x = [float(x)]
        else:
            x = _to_list(x)
        kw = {"linestyle": linestyles if isinstance(linestyles, str) else "-",
              "linewidth": float(linewidth), "alpha": float(alpha)}
        if colors is not None:
            kw["color"] = colors
        self._fig.axes_vlines(self._id, x, float(ymin), float(ymax), kw)
        return self

    def violinplot(self, dataset, positions=None, widths=None, showmeans=False,
                   showmedians=True, color=None, alpha=None, label=None, **kwargs):
        """Draw a violin plot."""
        # dataset can be a list of arrays or a single array (single violin)
        if dataset and not isinstance(dataset[0], (list, tuple)):
            if hasattr(dataset[0], '__iter__'):
                data_list = [_to_list(d) for d in dataset]
            else:
                data_list = [_to_list(dataset)]
        else:
            data_list = [_to_list(d) for d in dataset]
        kw = {"showmeans": showmeans, "showmedians": showmedians}
        if positions is not None:
            kw["positions"] = [float(p) for p in positions]
        if widths is not None:
            kw["widths"] = float(widths)
        if color is not None:
            kw["color"] = color
        if alpha is not None:
            kw["alpha"] = float(alpha)
        if label is not None:
            kw["label"] = label
        self._fig.axes_violinplot(self._id, data_list, kw)
        return self

    def fill_betweenx(self, y, x1, x2=0, alpha=0.3, color=None, label=None, **kwargs):
        """Fill between two x-curves sharing y values (horizontal bands)."""
        y = _to_list(y)
        x1 = _to_list(x1)
        if isinstance(x2, (int, float)):
            x2 = [float(x2)] * len(y)
        else:
            x2 = _to_list(x2)
        kw = {"alpha": float(alpha)}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_fill_betweenx(self._id, y, x1, x2, kw)
        return self

    def table(self, cellText=None, colLabels=None, rowLabels=None, loc='bottom', **kwargs):
        """Draw a table inside this axes."""
        kw = {"loc": loc}
        if cellText is not None:
            kw["cellText"] = [[str(cell) for cell in row] for row in cellText]
        if colLabels is not None:
            kw["colLabels"] = [str(l) for l in colLabels]
        if rowLabels is not None:
            kw["rowLabels"] = [str(l) for l in rowLabels]
        self._fig.axes_table(self._id, kw)
        return self

    def secondary_xaxis(self, location='top', functions=None, **kwargs):
        """Return a SecondaryAxisProxy for the secondary x-axis."""
        return SecondaryAxisProxy(self)

    def secondary_yaxis(self, location='right', functions=None, **kwargs):
        """Return a SecondaryAxisProxy for the secondary y-axis."""
        return SecondaryAxisProxy(self)

    def set_frame_on(self, b):
        pass

    def get_xlim(self):
        return (0, 1)  # stub

    def get_ylim(self):
        return (0, 1)  # stub

    def get_xaxis(self):
        return AxisProxy()

    def get_yaxis(self):
        return AxisProxy()

    @property
    def xaxis(self):
        return AxisProxy()

    @property
    def yaxis(self):
        return AxisProxy()

    def set_position(self, pos):
        pass

    def get_position(self):
        return [0, 0, 1, 1]

    def contains(self, event):
        return False, {}

    def format_coord(self, x, y):
        return f'x={x:.4g}, y={y:.4g}'

    def relim(self):
        pass

    def autoscale_view(self, **kwargs):
        pass

    def set_navigate(self, b):
        pass

    def get_children(self):
        return []

    def has_data(self):
        return True

    def can_pan(self):
        return False

    def can_zoom(self):
        return False

    def get_label(self):
        return ''

    def set_label(self, s):
        pass

    def set_zorder(self, level):
        pass

    def get_zorder(self):
        return 0

    def get_patch(self):
        return None

    def get_transData(self):
        return None

    def get_transAxes(self):
        return None

    def set_clip_on(self, b):
        pass

    def set_picker(self, picker):
        pass

    def radar(self, categories, values, colors=None, labels=None, alpha=0.8,
              fill=True, **kwargs):
        """Draw a radar / spider chart."""
        kw = {"alpha": float(alpha), "fill": bool(fill)}
        if colors is not None:
            kw["colors"] = list(colors)
        if labels is not None:
            kw["labels"] = list(labels)
        # values should be a list of lists (multiple series)
        if values and not isinstance(values[0], (list, tuple)):
            if hasattr(values[0], '__iter__'):
                vals = [_to_list(v) for v in values]
            else:
                vals = [_to_list(values)]
        else:
            vals = [_to_list(v) for v in values]
        self._fig.axes_radar(self._id, list(categories), vals, kw)
        return self

    def broken_barh(self, xranges, yrange, facecolors=None, alpha=1.0,
                    label=None, **kwargs):
        """Draw broken horizontal bars.

        Parameters:
            xranges: list of (x_start, width) tuples for each segment
            yrange: (y_start, height) for this row
            facecolors: color or list of colors
        """
        # Matplotlib API: broken_barh(xranges, yrange) for a single row
        y_ranges = [(float(yrange[0]), float(yrange[1]))]
        x_ranges = [[(float(xr[0]), float(xr[1])) for xr in xranges]]
        kw = {"alpha": float(alpha)}
        if facecolors is not None:
            if isinstance(facecolors, (list, tuple)) and len(facecolors) > 0:
                if isinstance(facecolors[0], str):
                    kw["colors"] = facecolors
                else:
                    kw["colors"] = list(facecolors)
            else:
                kw["colors"] = [facecolors]
        if label is not None:
            kw["label"] = label
        self._fig.axes_broken_barh(self._id, y_ranges, x_ranges, kw)
        return self

    def eventplot(self, positions, orientation='horizontal', linewidths=1.5,
                  colors=None, linelength=0.8, **kwargs):
        """Draw an event / raster plot."""
        # positions can be a single list (one row) or list of lists
        if positions and not isinstance(positions[0], (list, tuple)):
            if hasattr(positions[0], '__iter__'):
                pos = [_to_list(p) for p in positions]
            else:
                pos = [_to_list(positions)]
        else:
            pos = [_to_list(p) for p in positions]
        kw = {"orientation": orientation, "linewidths": float(linewidths),
              "linelength": float(linelength)}
        if colors is not None:
            kw["colors"] = list(colors)
        self._fig.axes_eventplot(self._id, pos, kw)
        return self

    def stackplot(self, x, *args, colors=None, labels=None, alpha=0.5, **kwargs):
        """Draw a stacked area chart."""
        x = _to_list(x)
        ys = [_to_list(y) for y in args]
        kw = {"alpha": float(alpha)}
        if colors is not None:
            kw["colors"] = list(colors)
        if labels is not None:
            kw["labels"] = list(labels)
        self._fig.axes_stackplot(self._id, x, ys, kw)
        return self

    def get_figure(self):
        """Return the FigureProxy that owns this axes."""
        return FigureProxy(self._fig, [self])

    @property
    def figure(self):
        """Return the FigureProxy that owns this axes."""
        return FigureProxy(self._fig, [self])


class TwinAxesProxy:
    """Python wrapper for a twin (right-side y-axis) axes."""

    def __init__(self, figure, twin_id):
        self._fig = figure
        self._id = twin_id

    def plot(self, *args, **kwargs):
        x, y, kwargs = _parse_plot_args(*args, **kwargs)
        self._fig.twin_axes_plot(self._id, x, y, kwargs)
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
        self._fig.twin_axes_scatter(self._id, x, y, kw)
        return self

    def bar(self, x, height, width=0.8, color=None, label=None, alpha=1.0, **kwargs):
        x, height = _to_list(x), _to_list(height)
        kw = {"width": width, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.twin_axes_bar(self._id, x, height, kw)
        return self

    def set_ylabel(self, label, fontsize=None, **kwargs):
        self._fig.twin_axes_set_ylabel(self._id, str(label), fontsize)

    def set_ylim(self, bottom=None, top=None, **kwargs):
        if bottom is not None and top is not None:
            self._fig.twin_axes_set_ylim(self._id, float(bottom), float(top))

    def legend(self, *args, **kwargs):
        kw = {}
        if 'loc' in kwargs:
            kw['loc'] = kwargs['loc']
        self._fig.twin_axes_legend(self._id, kw)


class TwinXAxesProxy:
    """Python wrapper for a twin (top-side x-axis) axes (twiny)."""

    def __init__(self, figure, twin_id):
        self._fig = figure
        self._id = twin_id

    def plot(self, *args, **kwargs):
        x, y, kwargs = _parse_plot_args(*args, **kwargs)
        self._fig.twiny_axes_plot(self._id, x, y, kwargs)
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
        self._fig.twiny_axes_scatter(self._id, x, y, kw)
        return self

    def set_xlabel(self, label, fontsize=None, **kwargs):
        self._fig.twiny_axes_set_xlabel(self._id, str(label), fontsize)

    def set_xlim(self, left=None, right=None, **kwargs):
        if left is not None and right is not None:
            self._fig.twiny_axes_set_xlim(self._id, float(left), float(right))

    def legend(self, *args, **kwargs):
        kw = {}
        if 'loc' in kwargs:
            kw['loc'] = kwargs['loc']
        self._fig.twiny_axes_legend(self._id, kw)


class SecondaryAxisProxy:
    """Proxy for secondary axes returned by secondary_xaxis()/secondary_yaxis().

    Provides no-op methods for matplotlib compatibility.
    """

    def __init__(self, parent_ax):
        self._parent = parent_ax

    def set_xlabel(self, label, **kwargs):
        pass

    def set_ylabel(self, label, **kwargs):
        pass

    def set_ticks(self, ticks, **kwargs):
        pass

    def set_ticklabels(self, labels, **kwargs):
        pass

    def set_label(self, label, **kwargs):
        pass

    def set_color(self, color):
        pass

    def set_visible(self, visible):
        pass

    def tick_params(self, **kwargs):
        pass

    def set_functions(self, functions):
        pass


class Axes3DProxy:
    """Python wrapper around a Rust 3D axes, accessed by ID."""

    def __init__(self, figure, ax3d_id):
        self._fig = figure
        self._id = ax3d_id

    def plot(self, xs, ys, zs, **kwargs):
        xs, ys, zs = _to_list(xs), _to_list(ys), _to_list(zs)
        kw = {}
        if 'color' in kwargs:
            kw['color'] = kwargs['color']
        if 'linewidth' in kwargs:
            kw['linewidth'] = float(kwargs['linewidth'])
        if 'label' in kwargs:
            kw['label'] = kwargs['label']
        self._fig.axes3d_plot(self._id, xs, ys, zs, kw)
        return self

    def scatter(self, xs, ys, zs, s=None, c=None, marker='o', alpha=1.0, label=None, **kwargs):
        xs, ys, zs = _to_list(xs), _to_list(ys), _to_list(zs)
        kw = {'marker': marker, 'alpha': float(alpha)}
        if s is not None:
            kw['s'] = list(np.atleast_1d(s).astype(float))
        if c is not None:
            kw['color'] = c
        if label is not None:
            kw['label'] = label
        self._fig.axes3d_scatter(self._id, xs, ys, zs, kw)
        return self

    def plot_surface(self, X, Y, Z, cmap='viridis', alpha=0.9, **kwargs):
        x_2d = _to_2d_list(X)
        y_2d = _to_2d_list(Y)
        z_2d = _to_2d_list(Z)
        kw = {'cmap': str(cmap), 'alpha': float(alpha)}
        self._fig.axes3d_plot_surface(self._id, x_2d, y_2d, z_2d, kw)
        return self

    def plot_wireframe(self, X, Y, Z, color=None, linewidth=0.5, **kwargs):
        x_2d = _to_2d_list(X)
        y_2d = _to_2d_list(Y)
        z_2d = _to_2d_list(Z)
        kw = {'linewidth': float(linewidth)}
        if color is not None:
            kw['color'] = color
        self._fig.axes3d_plot_wireframe(self._id, x_2d, y_2d, z_2d, kw)
        return self

    def bar3d(self, x, y, z, dx, dy, dz, color=None, alpha=0.9, **kwargs):
        x, y, z = _to_list(x), _to_list(y), _to_list(z)
        dx_l, dy_l, dz_l = _to_list(dx), _to_list(dy), _to_list(dz)
        kw = {'alpha': float(alpha)}
        if color is not None:
            kw['color'] = color
        self._fig.axes3d_bar3d(self._id, x, y, z, dx_l, dy_l, dz_l, kw)
        return self

    def set_title(self, title, fontsize=None, **kwargs):
        self._fig.axes3d_set_title(self._id, str(title), fontsize)

    def set_xlabel(self, label, fontsize=None, **kwargs):
        self._fig.axes3d_set_xlabel(self._id, str(label), fontsize)

    def set_ylabel(self, label, fontsize=None, **kwargs):
        self._fig.axes3d_set_ylabel(self._id, str(label), fontsize)

    def set_zlabel(self, label, fontsize=None, **kwargs):
        self._fig.axes3d_set_zlabel(self._id, str(label), fontsize)

    def view_init(self, elev=30, azim=-60, **kwargs):
        self._fig.axes3d_view_init(self._id, float(elev), float(azim))

    def set_xlim(self, left=None, right=None, **kwargs):
        if left is not None and right is not None:
            self._fig.axes3d_set_xlim(self._id, float(left), float(right))

    def set_ylim(self, bottom=None, top=None, **kwargs):
        if bottom is not None and top is not None:
            self._fig.axes3d_set_ylim(self._id, float(bottom), float(top))

    def set_zlim(self, bottom=None, top=None, **kwargs):
        if bottom is not None and top is not None:
            self._fig.axes3d_set_zlim(self._id, float(bottom), float(top))

    def legend(self, *args, **kwargs):
        self._fig.axes3d_legend(self._id)

    def contour3D(self, X, Y, Z, levels=None, cmap='viridis', linewidth=1.0,
                  alpha=1.0, offset=None, **kwargs):
        """Draw 3D contour lines projected at a fixed Z level."""
        x_2d = _to_2d_list(X)
        y_2d = _to_2d_list(Y)
        z_2d = _to_2d_list(Z)
        kw = {'cmap': str(cmap), 'alpha': float(alpha), 'linewidth': float(linewidth)}
        if levels is not None:
            kw['levels'] = [float(l) for l in levels]
        if offset is not None:
            kw['offset'] = float(offset)
        self._fig.axes3d_contour3d(self._id, x_2d, y_2d, z_2d, kw)
        return self

    # Alias: contour3d (lowercase) for convenience
    contour3d = contour3D

    def contourf3D(self, X, Y, Z, levels=None, cmap='viridis', linewidth=0.5,
                   alpha=0.7, offset=None, **kwargs):
        """Draw 3D filled contour lines projected at a fixed Z level."""
        x_2d = _to_2d_list(X)
        y_2d = _to_2d_list(Y)
        z_2d = _to_2d_list(Z)
        kw = {'cmap': str(cmap), 'alpha': float(alpha), 'linewidth': float(linewidth)}
        if levels is not None:
            kw['levels'] = [float(l) for l in levels]
        if offset is not None:
            kw['offset'] = float(offset)
        self._fig.axes3d_contourf3d(self._id, x_2d, y_2d, z_2d, kw)
        return self

    # Alias: contourf3d (lowercase)
    contourf3d = contourf3D

    def plot_trisurf(self, x, y, z, cmap='viridis', alpha=0.9, triangles=None, **kwargs):
        """Draw a 3D triangulated surface plot."""
        x, y, z = _to_list(x), _to_list(y), _to_list(z)
        kw = {'cmap': str(cmap), 'alpha': float(alpha)}
        if triangles is not None:
            # triangles is a list of [i, j, k] triples
            kw['triangles'] = [[int(v) for v in tri] for tri in triangles]
        self._fig.axes3d_plot_trisurf(self._id, x, y, z, kw)
        return self

    # No-op stubs for matplotlib compat
    def grid(self, visible=True, **kwargs):
        pass

    def set_xticks(self, *args, **kwargs):
        pass

    def set_yticks(self, *args, **kwargs):
        pass

    def set_zticks(self, *args, **kwargs):
        pass


class CanvasProxy:
    """Stub canvas for matplotlib event-connection compatibility."""

    def mpl_connect(self, event_name, callback):
        """Stub: accept event connection without crashing."""
        pass

    def mpl_disconnect(self, cid):
        pass

    def draw(self):
        pass

    def draw_idle(self):
        pass


class FigureProxy:
    """Python wrapper around RustFigure."""

    def __init__(self, rust_fig, axes_proxies):
        self._fig = rust_fig
        self._axes = axes_proxies
        self._canvas = CanvasProxy()

    @property
    def canvas(self):
        return self._canvas

    def savefig(self, fname, dpi=None, transparent=False, format=None, bbox_inches=None, **kwargs):
        tight = bbox_inches == 'tight' if bbox_inches else False
        self._fig.savefig(str(fname), dpi, transparent, tight)

    def set_size_inches(self, w, h=None):
        if h is None and hasattr(w, "__iter__"):
            w, h = w
        self._fig.set_size_inches(float(w), float(h))

    def suptitle(self, text, fontsize=None, **kwargs):
        self._fig.suptitle(str(text), fontsize)

    def subplots_adjust(self, hspace=None, wspace=None, **kwargs):
        self._fig.subplots_adjust(hspace, wspace)

    def tight_layout(self, **kwargs):
        pass

    def colorbar(self, mappable=None, ax=None, **kwargs):
        """Add a colorbar to the figure."""
        if ax is not None and hasattr(ax, 'colorbar'):
            ax.colorbar(mappable, **kwargs)
        elif self._axes:
            target = self._axes[0] if isinstance(self._axes, list) else self._axes
            if hasattr(target, 'colorbar'):
                target.colorbar(mappable, **kwargs)

    def set_facecolor(self, color):
        self._fig.set_facecolor(color)

    def show(self):
        try:
            get_ipython()  # noqa: F821 — exists in Jupyter/IPython
            from rustplotlib.backends.backend_inline import display_figure
            display_figure(self._fig)
        except NameError:
            self._fig.show()

    def add_subplot(self, *args, projection=None, **kwargs):
        """Add a subplot to the figure. Supports projection='3d'."""
        # Parse subplot spec: add_subplot(111) or add_subplot(1, 1, 1)
        if len(args) == 1:
            spec = int(args[0])
            nrows = spec // 100
            ncols = (spec // 10) % 10
            idx = (spec % 10) - 1
        elif len(args) == 3:
            nrows, ncols, idx = int(args[0]), int(args[1]), int(args[2]) - 1
        else:
            nrows, ncols, idx = 1, 1, 0

        # Ensure subplots layout is set up
        if nrows > 1 or ncols > 1:
            self._fig.setup_subplots(nrows, ncols)

        if projection == '3d':
            ax3d_id = self._fig.add_subplot_3d(idx)
            return Axes3DProxy(self._fig, ax3d_id)
        else:
            return AxesProxy(self._fig, idx)

    def get_axes(self):
        return self._axes

    def get_size_inches(self):
        return (6.4, 4.8)  # default

    def get_dpi(self):
        return 100

    def set_dpi(self, dpi):
        pass

    def clf(self):
        pass

    def clear(self):
        pass

    def add_axes(self, rect, **kwargs):
        return _gca()

    def get_tight_layout(self):
        return False

    def set_tight_layout(self, tight):
        pass

    @property
    def axes(self):
        return self._axes

    @property
    def number(self):
        return 1

    def align_labels(self, axs=None):
        pass

    def add_gridspec(self, nrows, ncols, **kwargs):
        """Create a GridSpec for this figure."""
        from rustplotlib.gridspec import GridSpec
        return GridSpec(nrows, ncols, figure=self, **kwargs)

    def align_xlabels(self, axs=None):
        pass

    def align_ylabels(self, axs=None):
        pass


def _to_list(data):
    # Handle pandas Series/Index
    try:
        import pandas as pd
        if isinstance(data, pd.Series):
            return data.astype(float).tolist()
        if isinstance(data, pd.Index):
            return data.astype(float).tolist()
    except ImportError:
        pass

    if isinstance(data, np.ndarray):
        return data.astype(float).flatten().tolist()
    if isinstance(data, (list, tuple)):
        return [float(v) for v in data]
    return [float(data)]


def _to_2d_list(data):
    try:
        import pandas as pd
        if isinstance(data, pd.DataFrame):
            return data.values.astype(float).tolist()
    except ImportError:
        pass
    if isinstance(data, np.ndarray):
        return data.astype(float).tolist()
    return [[float(v) for v in row] for row in data]


def _parse_contour_args(*args):
    """Parse contour arguments: contour(Z) or contour(X, Y, Z)."""
    if len(args) == 1:
        z = _to_2d_list(args[0])
        nrows = len(z)
        ncols = len(z[0]) if nrows > 0 else 0
        x = [[float(c) for c in range(ncols)] for _ in range(nrows)]
        y = [[float(r)] * ncols for r in range(nrows)]
        return x, y, z
    elif len(args) >= 3:
        x = _to_2d_list(args[0])
        y = _to_2d_list(args[1])
        z = _to_2d_list(args[2])
        return x, y, z
    else:
        raise ValueError("contour requires 1 or 3 positional arguments: contour(Z) or contour(X, Y, Z)")


def _is_string_sequence(data):
    """Check if data is a sequence of strings (categorical data)."""
    if isinstance(data, (list, tuple)) and data and isinstance(data[0], str):
        return True
    return False


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
        if isinstance(plain_args[1], str) and not _is_string_sequence(plain_args):
            y = _to_list(plain_args[0])
            x = list(range(len(y)))
            fmt = plain_args[1]
        else:
            # Keep string x values as-is for categorical handling
            if _is_string_sequence(plain_args[0]):
                x = list(plain_args[0])
            else:
                x = _to_list(plain_args[0])
            y = _to_list(plain_args[1])
    elif len(plain_args) >= 3:
        # Keep string x values as-is for categorical handling
        if _is_string_sequence(plain_args[0]):
            x = list(plain_args[0])
        else:
            x = _to_list(plain_args[0])
        y = _to_list(plain_args[1])
        if isinstance(plain_args[2], str):
            fmt = plain_args[2]

    if fmt:
        _parse_fmt(fmt, kwargs)

    # Ensure x values are float for Rust (skip if categorical strings)
    if not (x and isinstance(x[0], str)):
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


def _apply_font_from_rcparams():
    """Try to load a system font matching rcParams['font.family']."""
    import os
    family = rcParams.get('font.family', ['sans-serif'])
    if isinstance(family, list):
        family = family[0]

    font_dirs = [
        '/System/Library/Fonts/',  # macOS
        '/Library/Fonts/',  # macOS
        os.path.expanduser('~/Library/Fonts/'),  # macOS user
        '/usr/share/fonts/',  # Linux
        '/usr/share/fonts/truetype/',  # Linux (Ubuntu/Debian)
        '/usr/share/fonts/truetype/dejavu/',  # Linux DejaVu
        'C:/Windows/Fonts/',  # Windows
    ]
    font_names = {
        'serif': ['Times New Roman.ttf', 'TimesNewRoman.ttf', 'DejaVuSerif.ttf',
                   'LiberationSerif-Regular.ttf', 'Georgia.ttf'],
        'sans-serif': ['Helvetica.ttc', 'Arial.ttf', 'DejaVuSans.ttf',
                        'LiberationSans-Regular.ttf'],
        'monospace': ['Courier New.ttf', 'DejaVuSansMono.ttf',
                       'LiberationMono-Regular.ttf'],
    }

    candidates = font_names.get(family, [])
    for font_dir in font_dirs:
        for font_name in candidates:
            path = os.path.join(font_dir, font_name)
            if os.path.exists(path):
                try:
                    from rustplotlib._rustplotlib import set_font
                    set_font(path)
                    return
                except Exception:
                    pass

    # If family is an exact file path, try to load it directly
    if os.path.exists(family):
        try:
            from rustplotlib._rustplotlib import set_font
            set_font(family)
        except Exception:
            pass


def _apply_current_style(fig, ax_ids=None):
    """Apply the current rcParams style colors to a Rust figure and its axes."""
    # Try to resolve font from rcParams
    _apply_font_from_rcparams()

    # Figure background
    fig_fc = rcParams.get("figure.facecolor")
    if fig_fc:
        try:
            fig.set_facecolor(fig_fc)
        except Exception:
            pass

    # Apply to each axes
    if ax_ids is None:
        ax_ids = list(range(fig.num_axes()))
    for ax_id in ax_ids:
        # Axes background
        axes_fc = rcParams.get("axes.facecolor")
        if axes_fc:
            try:
                fig.axes_set_facecolor(ax_id, axes_fc)
            except Exception:
                pass
        # Text color
        text_c = rcParams.get("text.color")
        if text_c:
            try:
                fig.axes_set_text_color(ax_id, text_c)
            except Exception:
                pass
        # Spine / edge color
        edge_c = rcParams.get("axes.edgecolor")
        if edge_c:
            try:
                fig.axes_set_spine_color(ax_id, edge_c)
            except Exception:
                pass
        # Tick color (x and y)
        xtick_c = rcParams.get("xtick.color")
        if xtick_c:
            try:
                fig.axes_set_tick_color(ax_id, xtick_c)
            except Exception:
                pass
        # Grid settings
        if rcParams.get("axes.grid"):
            try:
                kw = {"visible": True}
                grid_color = rcParams.get("grid.color")
                if grid_color:
                    kw["color"] = grid_color
                grid_alpha = rcParams.get("grid.alpha")
                if grid_alpha is not None:
                    kw["alpha"] = float(grid_alpha)
                grid_lw = rcParams.get("grid.linewidth")
                if grid_lw is not None:
                    kw["linewidth"] = float(grid_lw)
                fig.axes_grid(ax_id, kw)
            except Exception:
                pass


def _ensure_figure():
    global _current_figure, _current_axes_id
    if _current_figure is None:
        _current_figure = RustFigure(640, 480, 100)
        _current_axes_id = _current_figure.add_axes()
        _apply_current_style(_current_figure)


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
    _apply_current_style(_current_figure)
    return FigureProxy(_current_figure, [_gca()])


def subplots(nrows=1, ncols=1, figsize=None, dpi=100, subplot_kw=None, **kwargs):
    global _current_figure, _current_axes_id
    if figsize:
        w, h = figsize
        fig = RustFigure(int(w * dpi), int(h * dpi), dpi)
    else:
        fig = RustFigure(640, 480, dpi)
    fig.setup_subplots(nrows, ncols)
    _current_figure = fig
    _current_axes_id = 0
    _apply_current_style(fig)

    subplot_kw = subplot_kw or {}
    is_3d = subplot_kw.get('projection') == '3d'

    def _make_ax(idx):
        if is_3d:
            ax3d_id = fig.add_subplot_3d(idx)
            return Axes3DProxy(fig, ax3d_id)
        else:
            return AxesProxy(fig, idx)

    if nrows == 1 and ncols == 1:
        axes = _make_ax(0)
    elif nrows == 1 or ncols == 1:
        n = max(nrows, ncols)
        axes = [_make_ax(i) for i in range(n)]
    else:
        axes = []
        for r in range(nrows):
            row = [_make_ax(r * ncols + c) for c in range(ncols)]
            axes.append(row)

    return FigureProxy(fig, axes), axes


def subplot_mosaic(mosaic, figsize=None, dpi=100, **kwargs):
    """Create subplots from an ASCII art or nested list layout.

    Example:
        fig, axes = plt.subplot_mosaic([['A', 'B'], ['C', 'C']])
    """
    global _current_figure, _current_axes_id

    if isinstance(mosaic, str):
        # Parse string mosaic: "AB\nCC"
        rows = [list(row.strip()) for row in mosaic.strip().split('\n')]
    else:
        rows = mosaic

    nrows = len(rows)
    ncols = max(len(row) for row in rows)

    # Find unique labels (preserving order)
    labels = []
    for row in rows:
        for label in row:
            if label not in labels and label != '.':
                labels.append(label)

    if figsize:
        w, h = figsize
        fig = RustFigure(int(w * dpi), int(h * dpi), dpi)
    else:
        fig = RustFigure(640, 480, dpi)

    fig.setup_subplots(nrows, ncols)
    _current_figure = fig
    _current_axes_id = 0
    _apply_current_style(fig)

    # Map labels to axes
    axes_dict = {}
    label_to_id = {}
    for label in labels:
        # Find the first cell with this label
        for r, row in enumerate(rows):
            for c, cell in enumerate(row):
                if cell == label and label not in label_to_id:
                    ax_id = r * ncols + c
                    label_to_id[label] = ax_id
                    axes_dict[label] = AxesProxy(fig, ax_id)

    fig_proxy = FigureProxy(fig, axes_dict)
    return fig_proxy, axes_dict


def plot(*args, **kwargs):
    return _gca().plot(*args, **kwargs)


def scatter(x, y, **kwargs):
    return _gca().scatter(x, y, **kwargs)


def bar(x, height, **kwargs):
    return _gca().bar(x, height, **kwargs)


def hist(x, **kwargs):
    return _gca().hist(x, **kwargs)


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


def axhspan(ymin, ymax, **kwargs):
    _gca().axhspan(ymin, ymax, **kwargs)


def axvspan(xmin, xmax, **kwargs):
    _gca().axvspan(xmin, xmax, **kwargs)


def hlines(y, xmin, xmax, **kwargs):
    _gca().hlines(y, xmin, xmax, **kwargs)


def vlines(x, ymin, ymax, **kwargs):
    _gca().vlines(x, ymin, ymax, **kwargs)


def violinplot(dataset, **kwargs):
    _gca().violinplot(dataset, **kwargs)


def fill_betweenx(y, x1, x2=0, **kwargs):
    _gca().fill_betweenx(y, x1, x2, **kwargs)


def table(**kwargs):
    _gca().table(**kwargs)


def contour(*args, **kwargs):
    _gca().contour(*args, **kwargs)


def contourf(*args, **kwargs):
    _gca().contourf(*args, **kwargs)


def hexbin(x, y, **kwargs):
    _gca().hexbin(x, y, **kwargs)


def colorbar(mappable=None, **kwargs):
    _gca().colorbar(mappable, **kwargs)


def quiver(*args, **kwargs):
    _gca().quiver(*args, **kwargs)


def streamplot(x, y, u, v, **kwargs):
    _gca().streamplot(x, y, u, v, **kwargs)


def broken_barh(xranges, yrange, **kwargs):
    _gca().broken_barh(xranges, yrange, **kwargs)


def eventplot(positions, **kwargs):
    _gca().eventplot(positions, **kwargs)


def stackplot(x, *args, **kwargs):
    _gca().stackplot(x, *args, **kwargs)


def subplot_polar(**kwargs):
    """Create a polar subplot."""
    global _current_figure, _current_axes_id
    _ensure_figure()
    ax_id = _current_axes_id
    _current_figure.axes_set_polar(ax_id, True)
    return _gca()


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


def xticks(ticks=None, labels=None, **kwargs):
    if ticks is not None:
        _gca().set_xticks(ticks, labels=labels)


def yticks(ticks=None, labels=None, **kwargs):
    if ticks is not None:
        _gca().set_yticks(ticks, labels=labels)


def annotate(text, xy, xytext=None, arrowprops=None, **kwargs):
    _gca().annotate(text, xy, xytext=xytext, arrowprops=arrowprops, **kwargs)


def suptitle(text, fontsize=None, **kwargs):
    _ensure_figure()
    _current_figure.suptitle(str(text), fontsize)


def subplots_adjust(hspace=None, wspace=None, **kwargs):
    _ensure_figure()
    _current_figure.subplots_adjust(hspace, wspace)


def tight_layout(**kwargs):
    pass


def savefig(fname, dpi=None, transparent=False, bbox_inches=None, **kwargs):
    tight = bbox_inches == 'tight' if bbox_inches else False
    _gcf().savefig(str(fname), dpi, transparent, tight)


def show():
    try:
        get_ipython()  # noqa: F821 — exists in Jupyter/IPython
        from rustplotlib.backends.backend_inline import display_figure
        display_figure(_gcf())
    except NameError:
        _gcf().show()


def close(*args):
    global _current_figure, _current_axes_id
    _current_figure = None
    _current_axes_id = None


def switch_backend(backend):
    """Switch rendering backend (compatibility stub)."""
    from rustplotlib import backends
    backends._current_backend = backend.lower()


# Also support matplotlib.use() pattern
def use(backend):
    switch_backend(backend)


def _handle_categorical(data):
    """Convert string data to numeric positions + tick labels."""
    if data and isinstance(data[0], str):
        labels = list(data)
        positions = list(range(len(labels)))
        return positions, labels
    return _to_list(data), None


# --- Phase 9: Additional pyplot compatibility functions ---


def clf():
    """Clear current figure."""
    global _current_figure, _current_axes_id
    _current_figure = None
    _current_axes_id = None


def cla():
    """Clear current axes."""
    close()


def gcf():
    """Get current figure."""
    _ensure_figure()
    return FigureProxy(_current_figure, [_gca()])


def gca(**kwargs):
    """Get current axes."""
    return _gca()


def subplot(*args, **kwargs):
    """Add a subplot to the current figure. Supports subplot(nrows, ncols, index) and subplot(NRC)."""
    global _current_figure, _current_axes_id
    if len(args) == 1 and isinstance(args[0], int) and args[0] >= 100:
        # subplot(211) format
        n = args[0]
        nrows = n // 100
        ncols = (n % 100) // 10
        index = n % 10
    elif len(args) == 3:
        nrows, ncols, index = int(args[0]), int(args[1]), int(args[2])
    else:
        nrows, ncols, index = 1, 1, 1

    if _current_figure is None:
        fig = RustFigure(640, 480, 100)
        fig.setup_subplots(nrows, ncols)
        _current_figure = fig

    _current_axes_id = index - 1  # matplotlib uses 1-based indexing
    return _gca()


def axes(arg=None, **kwargs):
    """Add axes to current figure."""
    if arg is None:
        return _gca()
    # arg is [left, bottom, width, height] — custom positioning, return stub
    return _gca()


def figtext(x, y, s, **kwargs):
    """Add text to figure (not axes)."""
    # Approximate: add text to current axes using figure coords
    pass


def figimage(*args, **kwargs):
    """Add image to figure."""
    pass


def figlegend(*args, **kwargs):
    """Add legend to figure."""
    legend(*args, **kwargs)


def minorticks_on():
    pass


def minorticks_off():
    pass


def tick_params(**kwargs):
    _gca().tick_params(**kwargs)


def margins(*args, **kwargs):
    pass


def autoscale(enable=True, axis='both', tight=None):
    pass


def ioff():
    """Turn interactive mode off."""
    pass


def ion():
    """Turn interactive mode on."""
    pass


def isinteractive():
    return False


def draw():
    pass


def pause(interval):
    """Pause for interval seconds."""
    import time
    time.sleep(interval)


def connect(event, func):
    pass


def disconnect(cid):
    pass


def get_fignums():
    if _current_figure is not None:
        return [1]
    return []


def figure_exists(num):
    return _current_figure is not None


def get_current_fig_manager():
    return None


def colormaps():
    """Return list of available colormaps."""
    return ['viridis', 'plasma', 'inferno', 'magma', 'cividis', 'twilight', 'turbo',
            'hot', 'cool', 'gray', 'jet', 'spring', 'summer', 'autumn', 'winter',
            'copper', 'bone', 'pink', 'binary', 'gist_heat', 'ocean', 'terrain',
            'Blues', 'Reds', 'Greens', 'YlOrRd', 'YlGnBu', 'RdYlBu', 'RdBu',
            'PiYG', 'PRGn', 'BrBG', 'Spectral', 'Set1', 'Set2', 'Set3',
            'Pastel1', 'Pastel2', 'tab20']


def get_cmap(name='viridis'):
    """Get a colormap by name."""
    return name  # stub


# Log-scale convenience aliases
def semilogy(*args, **kwargs):
    yscale('log')
    plot(*args, **kwargs)


def semilogx(*args, **kwargs):
    xscale('log')
    plot(*args, **kwargs)


def loglog(*args, **kwargs):
    xscale('log')
    yscale('log')
    plot(*args, **kwargs)


def rc(group, **kwargs):
    """Set rcParams for a group.

    Example: plt.rc('font', size=14, family='serif')
    """
    for key, val in kwargs.items():
        rcParams[f"{group}.{key}"] = val


def rc_context(rc=None):
    """Context manager to temporarily change rcParams.

    Example:
        with plt.rc_context({'font.size': 14}):
            plt.plot(x, y)
    """
    from contextlib import contextmanager

    @contextmanager
    def _ctx():
        old = rcParams.copy()
        if rc:
            rcParams.update(rc)
        try:
            yield
        finally:
            rcParams.clear()
            rcParams.update(old)
    return _ctx()
