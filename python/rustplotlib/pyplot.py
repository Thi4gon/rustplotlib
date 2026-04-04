"""rustplotlib.pyplot — matplotlib-compatible plotting API powered by Rust."""

from rustplotlib._rustplotlib import RustFigure
import numpy as np
from rustplotlib import style

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
    # Axes prop cycle (matplotlib default 10-color cycle)
    'axes.prop_cycle': None,  # set below after cycler import attempt
    # Layout
    'figure.autolayout': False,
    # Formatter options
    'axes.formatter.useoffset': True,
    'axes.formatter.use_mathtext': False,
    # Spine visibility
    'axes.spines.left': True,
    'axes.spines.bottom': True,
    'axes.spines.top': True,
    'axes.spines.right': True,
    # Patch properties
    'patch.linewidth': 1.0,
    'patch.facecolor': '#1f77b4',
    'patch.edgecolor': 'black',
    # Histogram default bins
    'hist.bins': 10,
    # Scatter default marker
    'scatter.marker': 'o',
    # Boxplot flier marker
    'boxplot.flierprops.marker': 'o',
    # Line defaults
    'lines.linestyle': '-',
    'lines.marker': 'None',
    'lines.color': '#1f77b4',
}

# Try to set axes.prop_cycle with cycler if available
try:
    from cycler import cycler as _cycler
    rcParams['axes.prop_cycle'] = _cycler('color', [
        '#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd',
        '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf',
    ])
except ImportError:
    # cycler not installed — store a plain list as fallback
    rcParams['axes.prop_cycle'] = [
        '#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd',
        '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf',
    ]


class SpineProxy:
    """Proxy for a single spine (top/right/bottom/left)."""

    def __init__(self, figure, ax_id, which):
        self._fig = figure
        self._id = ax_id
        self._which = which
        self._position = None  # stored position (matplotlib compat)

    def set_visible(self, visible):
        self._fig.axes_set_spine_visible(self._id, self._which, visible)

    def set_position(self, position):
        """Set the spine position.

        Parameters
        ----------
        position : str or tuple
            - 'center' — spine at the center of the axes
            - 'zero'   — alias for ('data', 0)
            - ('data', val) — spine at data coordinate val
            - ('axes', val) — spine at axes fraction val (0=left/bottom, 1=right/top)
            - ('outward', val) — spine outward by val points
        """
        if position == 'zero':
            position = ('data', 0)
        self._position = position

    def get_position(self):
        """Return the stored spine position."""
        return self._position

    def set_color(self, color):
        """Set the spine color."""
        try:
            self._fig.axes_set_spine_color(self._id, color)
        except Exception:
            pass  # best-effort

    def set_edgecolor(self, color):
        """Alias for set_color."""
        self.set_color(color)

    def set_linewidth(self, lw):
        """Set the spine linewidth."""
        try:
            self._fig.axes_set_spine_linewidth(self._id, float(lw))
        except Exception:
            pass  # best-effort

    def set_lw(self, lw):
        """Alias for set_linewidth."""
        self.set_linewidth(lw)

    def set_linestyle(self, ls):
        """Set the spine linestyle (stored for matplotlib compat)."""
        pass  # visual effect not yet implemented at Rust level

    def set_capstyle(self, cs):
        """Set cap style (matplotlib compat stub)."""
        pass

    def set_joinstyle(self, js):
        """Set join style (matplotlib compat stub)."""
        pass

    def set_bounds(self, low=None, high=None):
        """Set the bounds of the spine (matplotlib compat stub)."""
        pass


class SpinesProxy:
    """Proxy for all spines of an axes."""

    _SPINE_NAMES = ('top', 'right', 'bottom', 'left')

    def __init__(self, figure, ax_id):
        self._fig = figure
        self._id = ax_id

    def __getitem__(self, key):
        return SpineProxy(self._fig, self._id, key)

    def values(self):
        """Return iterable of SpineProxy objects (matplotlib compat)."""
        return [SpineProxy(self._fig, self._id, name) for name in self._SPINE_NAMES]

    def items(self):
        """Return iterable of (name, SpineProxy) pairs (matplotlib compat)."""
        return [(name, SpineProxy(self._fig, self._id, name)) for name in self._SPINE_NAMES]

    def keys(self):
        """Return the spine names."""
        return list(self._SPINE_NAMES)

    def __iter__(self):
        return iter(self._SPINE_NAMES)

    def __contains__(self, key):
        return key in self._SPINE_NAMES


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


class _PickableMixin:
    """Mixin that adds picker/pick-event support to artist proxies."""

    def set_picker(self, picker):
        """Set picker. Can be None, bool, float (tolerance), or callable."""
        self._picker = picker

    def get_picker(self):
        return getattr(self, '_picker', None)

    def pickable(self):
        return getattr(self, '_picker', None) is not None

    def contains(self, mouseevent):
        """Test if the artist contains the mouse event point.

        Returns (bool, details_dict).
        """
        picker = self.get_picker()
        if picker is None:
            return False, {}
        if callable(picker):
            return picker(self, mouseevent)
        # Default: subclass implements _default_contains
        return self._default_contains(mouseevent, picker)

    def _default_contains(self, mouseevent, picker):
        return False, {}


class Line2DProxy(_PickableMixin):
    """Proxy for matplotlib Line2D objects returned by plot()."""

    def __init__(self):
        self._color = 'blue'
        self._label = ''
        self._linewidth = 1.5
        self._linestyle = '-'
        self._xdata = []
        self._ydata = []
        self._picker = None
        self._axes = None

    def set_data(self, x, y):
        self._xdata = list(x) if x is not None else []
        self._ydata = list(y) if y is not None else []

    def _default_contains(self, mouseevent, picker):
        """Check if mouse is within picker tolerance of any line point.

        Uses Rust hit_test_line() for performance.
        """
        if not self._xdata or not self._ydata:
            return False, {}
        tolerance = float(picker) if isinstance(picker, (int, float)) else 5.0
        mx, my = mouseevent.xdata, mouseevent.ydata
        if mx is None or my is None:
            return False, {}
        from rustplotlib._rustplotlib import hit_test_line
        indices = hit_test_line(self._xdata, self._ydata, mx, my, tolerance)
        if indices:
            return True, {"ind": list(indices)}
        return False, {}

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

    def set_path_effects(self, effects):
        """Store path effects (used for matplotlib API compatibility)."""
        self._path_effects = effects

    def get_path_effects(self):
        return getattr(self, '_path_effects', [])


class PathCollectionProxy(_PickableMixin):
    """Proxy for matplotlib PathCollection returned by scatter()."""

    def __init__(self):
        self._offsets = []
        self._picker = None
        self._axes = None

    def set_offsets(self, offsets):
        self._offsets = list(offsets)

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
        return self._offsets

    def _default_contains(self, mouseevent, picker):
        """Check if mouse is within picker tolerance of any scatter point.

        Uses Rust hit_test_points() for performance.
        """
        if not self._offsets:
            return False, {}
        tolerance = float(picker) if isinstance(picker, (int, float)) else 5.0
        mx, my = mouseevent.xdata, mouseevent.ydata
        if mx is None or my is None:
            return False, {}
        from rustplotlib._rustplotlib import hit_test_points
        xs = [pt[0] for pt in self._offsets if len(pt) >= 2]
        ys = [pt[1] for pt in self._offsets if len(pt) >= 2]
        indices = hit_test_points(xs, ys, mx, my, tolerance)
        if indices:
            return True, {"ind": list(indices)}
        return False, {}


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


class _LegendStub:
    """Minimal stub for matplotlib Legend objects."""

    def set_visible(self, b):
        pass

    def get_visible(self):
        return True

    def remove(self):
        pass

    def get_texts(self):
        return []


class _TransformStub:
    """Stub for matplotlib transform objects (transData, transAxes, etc.)."""

    def transform(self, points):
        """Return points unchanged (identity transform stub)."""
        return points

    def inverted(self):
        return self

    def __add__(self, other):
        return self

    def __radd__(self, other):
        return self


class AxesProxy:
    """Python wrapper around a Rust axes, accessed by ID."""

    def __init__(self, figure, ax_id):
        self._fig = figure
        self._id = ax_id
        self._title_cache = ''
        self._xlabel_cache = ''
        self._ylabel_cache = ''
        self._xscale_cache = 'linear'
        self._yscale_cache = 'linear'
        self._facecolor_cache = 'white'
        self._has_data_flag = False
        self._legend_obj = None
        self._artists = []  # list of artist proxies for pick events

    def plot(self, *args, zorder=None, path_effects=None, **kwargs):
        groups = _parse_plot_args_multi(*args, **kwargs)
        proxies = []
        self._has_data_flag = True
        for x, y, group_kw in groups:
            if zorder is not None:
                group_kw["zorder"] = int(zorder)
            # Process path_effects → outline params for Rust
            if path_effects:
                from rustplotlib.patheffects import Stroke
                for pe in path_effects:
                    if isinstance(pe, Stroke):
                        group_kw["outline_color"] = pe.foreground
                        group_kw["outline_width"] = float(pe.linewidth)
                        break
            # Handle categorical (string) x values
            if x and isinstance(x[0], str):
                positions, labels = _handle_categorical(x)
                x = [float(p) for p in positions]
                self._fig.axes_set_xticks(self._id, x)
                self._fig.axes_set_xticklabels(self._id, labels)
            self._fig.axes_plot(self._id, x, y, group_kw)
            proxy = Line2DProxy()
            proxy._xdata = list(x)
            proxy._ydata = list(y)
            proxy._axes = self
            if 'color' in group_kw:
                proxy._color = group_kw['color']
            if 'label' in group_kw:
                proxy._label = group_kw['label']
            if 'linewidth' in group_kw:
                proxy._linewidth = group_kw['linewidth']
            if 'linestyle' in group_kw:
                proxy._linestyle = group_kw['linestyle']
            if 'picker' in group_kw:
                proxy.set_picker(group_kw['picker'])
            self._artists.append(proxy)
            proxies.append(proxy)
        return proxies

    def scatter(self, x, y, s=None, c=None, marker="o", alpha=1.0, label=None, zorder=None, picker=None, **kwargs):
        self._has_data_flag = True
        x, y = _to_list(x), _to_list(y)
        kw = {"marker": marker, "alpha": alpha}
        if s is not None:
            kw["s"] = list(np.atleast_1d(s).astype(float))
        if c is not None:
            kw["color"] = c
        if label is not None:
            kw["label"] = label
        if zorder is not None:
            kw["zorder"] = int(zorder)
        self._fig.axes_scatter(self._id, x, y, kw)
        proxy = PathCollectionProxy()
        proxy._offsets = list(zip(x, y))
        proxy._axes = self
        if picker is not None:
            proxy.set_picker(picker)
        self._artists.append(proxy)
        return proxy

    def bar(self, x, height, width=0.8, bottom=None, color=None, label=None, alpha=1.0, hatch=None, zorder=None, **kwargs):
        self._has_data_flag = True
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
        if hatch is not None:
            kw["hatch"] = str(hatch)
        if zorder is not None:
            kw["zorder"] = int(zorder)
        if cat_labels is not None:
            self._fig.axes_set_xticks(self._id, x)
            self._fig.axes_set_xticklabels(self._id, cat_labels)
        self._fig.axes_bar(self._id, x, height, kw)
        return BarContainerProxy()

    def hist(self, x, bins=10, color=None, alpha=1.0, label=None, **kwargs):
        self._has_data_flag = True
        x = _to_list(x)
        kw = {"bins": bins, "alpha": alpha}
        if color is not None:
            kw["color"] = color
        if label is not None:
            kw["label"] = label
        self._fig.axes_hist(self._id, x, kw)
        return self

    def imshow(self, data, cmap="viridis", aspect=None, vmin=None, vmax=None,
               annotate=False, fmt=None, interpolation=None, origin=None,
               extent=None, **kwargs):
        import numpy as _np
        arr = _np.asarray(data, dtype=float)
        # Origin flip and RGB detection are handled in Rust

        if arr.ndim == 3 and arr.shape[2] in (3, 4):
            # RGB or RGBA image — pass as 3D list to Rust
            rgb_list = arr.tolist()
            kw = {}
            if interpolation is not None:
                kw["interpolation"] = str(interpolation)
            if extent is not None:
                kw["extent"] = [float(e) for e in extent]
            if origin is not None:
                kw["origin"] = str(origin)
            self._fig.axes_imshow_rgb(self._id, rgb_list, kw)
        else:
            # Scalar 2D data — use colormap
            if arr.ndim == 3:
                arr = arr.mean(axis=2)
            data_list = arr.tolist()
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
            if extent is not None:
                kw["extent"] = [float(e) for e in extent]
            if origin is not None:
                kw["origin"] = str(origin)
            self._fig.axes_imshow(self._id, data_list, kw)
        return self

    def set_title(self, title, fontsize=None, loc=None, **kwargs):
        self._title_cache = str(title)
        self._fig.axes_set_title(self._id, str(title), fontsize, loc)

    def set_xlabel(self, label, fontsize=None, color=None, **kwargs):
        self._xlabel_cache = str(label)
        color_str = None
        if color is not None:
            color_str = str(color)
        elif 'fontdict' in kwargs and isinstance(kwargs['fontdict'], dict):
            if 'color' in kwargs['fontdict']:
                color_str = str(kwargs['fontdict']['color'])
        self._fig.axes_set_xlabel(self._id, str(label), fontsize, color_str)

    def set_ylabel(self, label, fontsize=None, color=None, **kwargs):
        self._ylabel_cache = str(label)
        color_str = None
        if color is not None:
            color_str = str(color)
        elif 'fontdict' in kwargs and isinstance(kwargs['fontdict'], dict):
            if 'color' in kwargs['fontdict']:
                color_str = str(kwargs['fontdict']['color'])
        self._fig.axes_set_ylabel(self._id, str(label), fontsize, color_str)

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
        self._legend_obj = _LegendStub()

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
        self._xscale_cache = str(scale)
        self._fig.axes_set_xscale(self._id, str(scale))

    def set_yscale(self, scale, **kwargs):
        self._yscale_cache = str(scale)
        self._fig.axes_set_yscale(self._id, str(scale))

    def get_xscale(self):
        """Return the current x-axis scale ('linear' or 'log')."""
        return getattr(self, '_xscale_cache', 'linear')

    def get_yscale(self):
        """Return the current y-axis scale ('linear' or 'log')."""
        return getattr(self, '_yscale_cache', 'linear')

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
                 color=None, bbox=None, fontweight=None, fontstyle=None, **kwargs):
        """Add an annotation with optional arrow, bbox, and font styling.

        Parameters
        ----------
        text : str
            Annotation text.
        xy : (float, float)
            The point (x, y) to annotate.
        xytext : (float, float), optional
            Where to place the text (defaults to xy).
        arrowprops : dict, optional
            Arrow properties. Supports keys: 'arrowstyle', 'connectionstyle', 'color', 'width'.
        fontsize : float, optional
            Font size for the annotation text.
        color : color, optional
            Text color.
        bbox : dict, optional
            Text box properties. Supports keys: 'boxstyle', 'facecolor', 'edgecolor', 'alpha'.
        fontweight : str, optional
            Font weight ('normal', 'bold').
        fontstyle : str, optional
            Font style ('normal', 'italic').
        """
        if xytext is None:
            xytext = xy
        kw = {}
        if fontsize is not None:
            kw["fontsize"] = float(fontsize)
        if color is not None:
            kw["color"] = color
        if arrowprops is not None:
            kw["arrowprops"] = dict(arrowprops)
        if bbox is not None:
            kw["bbox"] = dict(bbox)
        if fontweight is not None:
            kw["fontweight"] = str(fontweight)
        if fontstyle is not None:
            kw["fontstyle"] = str(fontstyle)
        xy_tuple = (float(xy[0]), float(xy[1]))
        xytext_tuple = (float(xytext[0]), float(xytext[1]))
        self._fig.axes_annotate(self._id, str(text), xy_tuple, xytext_tuple, kw)
        return self

    def axis(self, arg):
        if arg == 'off':
            self._fig.axes_set_axis_off(self._id, True)
        elif arg == 'on':
            self._fig.axes_set_axis_off(self._id, False)

    def set_xticks(self, ticks, labels=None, minor=False, **kwargs):
        if minor:
            self._fig.axes_set_xticks_minor(self._id, [float(t) for t in ticks])
        else:
            self._fig.axes_set_xticks(self._id, [float(t) for t in ticks])
            if labels is not None:
                self._fig.axes_set_xticklabels(self._id, [str(l) for l in labels])

    def set_yticks(self, ticks, labels=None, minor=False, **kwargs):
        if minor:
            self._fig.axes_set_yticks_minor(self._id, [float(t) for t in ticks])
        else:
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

    def add_collection(self, collection):
        """Add a collection (LineCollection, PatchCollection) to the axes via Rust."""
        from rustplotlib.collections import LineCollection, PatchCollection
        if isinstance(collection, LineCollection):
            # Convert segments to list of list of (f64, f64)
            segments = []
            for seg in collection.segments:
                pts = [(float(p[0]), float(p[1])) for p in seg]
                segments.append(pts)
            kw = {}
            if collection._color is not None:
                kw['color'] = collection._color
            if collection._linewidth is not None:
                kw['linewidth'] = float(collection._linewidth)
            if collection._alpha is not None:
                kw['alpha'] = float(collection._alpha)
            if collection._label is not None:
                kw['label'] = str(collection._label)
            self._fig.axes_add_line_collection(self._id, segments, kw)
        elif isinstance(collection, PatchCollection):
            # Render each patch individually
            for patch in collection.patches:
                self.add_patch(patch)
        return collection

    def add_patch(self, patch):
        """Add a patch object (Rectangle, Circle, Polygon, FancyArrowPatch) to the axes."""
        from rustplotlib.patches import Rectangle, Circle, Polygon, FancyArrowPatch
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

        if isinstance(patch, FancyArrowPatch):
            if patch.posA is not None and patch.posB is not None:
                fa_kw = {}
                if hasattr(patch, 'facecolor') and patch.facecolor is not None:
                    fa_kw['color'] = patch.facecolor
                elif hasattr(patch, 'edgecolor') and patch.edgecolor is not None:
                    fa_kw['color'] = patch.edgecolor
                if hasattr(patch, 'linewidth'):
                    fa_kw['linewidth'] = float(patch.linewidth)
                if hasattr(patch, 'arrowstyle') and patch.arrowstyle:
                    fa_kw['arrowstyle'] = str(patch.arrowstyle)
                if hasattr(patch, 'connectionstyle') and patch.connectionstyle:
                    fa_kw['connectionstyle'] = str(patch.connectionstyle)
                if hasattr(patch, 'mutation_scale'):
                    fa_kw['mutation_scale'] = float(patch.mutation_scale)
                if hasattr(patch, 'shrinkA'):
                    fa_kw['shrinkA'] = float(patch.shrinkA)
                if hasattr(patch, 'shrinkB'):
                    fa_kw['shrinkB'] = float(patch.shrinkB)
                if hasattr(patch, 'alpha') and patch.alpha is not None:
                    fa_kw['alpha'] = float(patch.alpha)
                pos_a = (float(patch.posA[0]), float(patch.posA[1]))
                pos_b = (float(patch.posB[0]), float(patch.posB[1]))
                self._fig.axes_fancy_arrow(self._id, pos_a, pos_b, fa_kw)
        elif isinstance(patch, Rectangle):
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

    def colorbar(self, mappable=None, cmap="viridis", vmin=0.0, vmax=1.0,
                 label=None, orientation='vertical', shrink=1.0, pad=0.05, **kwargs):
        """Add a colorbar as a separate Axes adjacent to this axes.

        Creates a new Axes with a ColorbarArtist rendered in Rust.
        """
        if mappable is not None:
            if hasattr(mappable, 'cmap'):
                cmap = mappable.cmap
            if hasattr(mappable, 'vmin') and mappable.vmin is not None:
                vmin = mappable.vmin
            if hasattr(mappable, 'vmax') and mappable.vmax is not None:
                vmax = mappable.vmax

        # Create a new axes for the colorbar with position adjacent to this axes
        cb_idx = self._fig.add_axes()

        horizontal = (orientation == 'horizontal')
        if horizontal:
            # Colorbar below: same width, small height
            # We don't know exact position, so use a relative approach
            # Set as a custom-positioned axes
            # This is approximate — works well with add_axes-based layouts
            pass
        else:
            # Colorbar to the right: small width, same height
            pass

        # Add the colorbar artist to the new axes
        self._fig.axes_add_colorbar_artist(
            cb_idx,
            str(cmap),
            float(vmin),
            float(vmax),
            str(orientation),
            str(label) if label is not None else None,
        )

        # Also keep the old inline colorbar for backward compat with grid layouts
        kw = {
            "cmap": str(cmap),
            "vmin": float(vmin),
            "vmax": float(vmax),
            "orientation": str(orientation),
            "shrink": float(shrink),
            "pad": float(pad),
        }
        if label is not None:
            kw["label"] = str(label)
        self._fig.axes_colorbar(self._id, kw)

        return AxesProxy(self._fig, cb_idx)

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

        Supported kwargs: title, xlabel, ylabel, xlim, ylim, xscale, yscale,
        aspect, facecolor (and any other key that maps to a set_<key> method).
        """
        # Normalise keys that map directly to named methods
        _aliases = {
            'title': 'set_title',
            'xlabel': 'set_xlabel',
            'ylabel': 'set_ylabel',
            'xscale': 'set_xscale',
            'yscale': 'set_yscale',
            'aspect': 'set_aspect',
            'facecolor': 'set_facecolor',
        }
        # Keys that accept tuple/list unpacked as positional args
        _unpack = {'xlim', 'ylim'}

        for key, val in kwargs.items():
            if key in _aliases:
                getattr(self, _aliases[key])(val)
            else:
                method = getattr(self, f'set_{key}', None)
                if method:
                    if isinstance(val, (list, tuple)) and key in _unpack:
                        method(*val)
                    else:
                        method(val)
        return self

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
        self._facecolor_cache = color
        self._fig.axes_set_facecolor(self._id, color)

    def get_facecolor(self):
        """Return the current axes facecolor."""
        return getattr(self, '_facecolor_cache', 'white')

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
        return self._fig.axes_get_xlim(self._id)

    def get_ylim(self):
        return self._fig.axes_get_ylim(self._id)

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

    @property
    def name(self):
        """Return the axes projection name (matplotlib compat)."""
        return 'rectilinear'

    @property
    def transData(self):
        """Return a compatibility stub for the data coordinate transform."""
        return _TransformStub()

    @property
    def transAxes(self):
        """Return a compatibility stub for the axes coordinate transform."""
        return _TransformStub()

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
        """Return True if any artists have been plotted to this axes."""
        return getattr(self, '_has_data_flag', False)

    def clear(self):
        """Clear all artists from this axes."""
        self._has_data_flag = False
        self._fig.axes_clear(self._id)

    def cla(self):
        """Clear current axes (alias for clear)."""
        self.clear()

    def get_lines(self):
        """Return list of Line2D artists in this axes."""
        return []

    def get_legend(self):
        """Return the legend for this axes, or None if no legend was added."""
        return getattr(self, '_legend_obj', None)

    def get_title(self):
        """Return the axes title."""
        return self._title_cache

    def get_xlabel(self):
        """Return the x-axis label."""
        return self._xlabel_cache

    def get_ylabel(self):
        """Return the y-axis label."""
        return self._ylabel_cache

    @property
    def patches(self):
        """Return list of Patch artists."""
        return []

    @property
    def lines(self):
        """Return list of Line2D artists."""
        return []

    @property
    def texts(self):
        """Return list of Text artists."""
        return []

    @property
    def images(self):
        """Return list of image artists."""
        return []

    @property
    def collections(self):
        """Return list of Collection artists."""
        return []

    @property
    def containers(self):
        """Return list of container artists."""
        return []

    def remove(self):
        """Remove this axes from the figure."""
        pass

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
        """Set picker on the axes itself."""
        self._picker = picker

    def get_picker(self):
        return getattr(self, '_picker', None)

    def pick(self, mouseevent):
        """Test all child artists for pick, fire pick_event via canvas callbacks."""
        from .events import PickEvent
        for artist in self._artists:
            if artist.pickable():
                inside, props = artist.contains(mouseevent)
                if inside:
                    pick_event = PickEvent('pick_event', mouseevent.canvas,
                                           mouseevent=mouseevent, artist=artist)
                    pick_event.ind = props.get('ind', [])
                    # Fire via canvas callback registry
                    if hasattr(mouseevent, 'canvas') and mouseevent.canvas is not None:
                        canvas = mouseevent.canvas
                        if hasattr(canvas, '_callbacks'):
                            canvas._callbacks.process('pick_event', pick_event)

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

    def fill(self, *args, color=None, alpha=None, label=None, **kwargs):
        """Draw a filled polygon. Usage: fill(x, y) or fill(x, y, color)."""
        if len(args) >= 2:
            x = _to_list(args[0])
            y = _to_list(args[1])
            if len(args) >= 3 and isinstance(args[2], str) and color is None:
                color = args[2]
        else:
            return self
        kw = {}
        if color is not None:
            kw["color"] = color
        if alpha is not None:
            kw["alpha"] = float(alpha)
        if label is not None:
            kw["label"] = label
        self._fig.axes_fill(self._id, x, y, kw)
        return self

    def pcolormesh(self, *args, cmap=None, alpha=None, edgecolors=None, **kwargs):
        """Draw a pseudocolor mesh plot."""
        # pcolormesh(C) or pcolormesh(X, Y, C)
        if len(args) == 1:
            c = [_to_list(row) for row in args[0]]
            kw = {}
        elif len(args) == 3:
            x = [_to_list(row) for row in args[0]]
            y = [_to_list(row) for row in args[1]]
            c = [_to_list(row) for row in args[2]]
            kw = {"x": x, "y": y}
        else:
            return self
        if cmap is not None:
            kw["cmap"] = str(cmap)
        if alpha is not None:
            kw["alpha"] = float(alpha)
        if edgecolors is not None:
            kw["edgecolors"] = edgecolors
        self._fig.axes_pcolormesh(self._id, c, kw)
        return self

    def pcolor(self, *args, cmap=None, alpha=None, **kwargs):
        """Draw a pseudocolor plot with cell outlines."""
        if len(args) == 1:
            c = [_to_list(row) for row in args[0]]
            kw = {}
        elif len(args) == 3:
            x = [_to_list(row) for row in args[0]]
            y = [_to_list(row) for row in args[1]]
            c = [_to_list(row) for row in args[2]]
            kw = {"x": x, "y": y}
        else:
            return self
        if cmap is not None:
            kw["cmap"] = str(cmap)
        if alpha is not None:
            kw["alpha"] = float(alpha)
        self._fig.axes_pcolor(self._id, c, kw)
        return self

    def spy(self, Z, precision=0, marker=None, markersize=None, **kwargs):
        """Plot the sparsity pattern of a 2D array."""
        import numpy as np
        Z = np.asarray(Z)
        mask = np.abs(Z) > precision
        self.imshow(mask.astype(float), cmap='gray_r', interpolation='nearest', **kwargs)
        self.set_aspect('equal')

    def stairs(self, values, edges=None, **kwargs):
        """Draw a step-wise constant function (like matplotlib.pyplot.stairs)."""
        if edges is None:
            edges = list(range(len(values) + 1))
        x = []
        y = []
        for i, v in enumerate(values):
            x.extend([edges[i], edges[i+1]])
            y.extend([v, v])
        self.plot(x, y, **kwargs)

    def ecdf(self, x, **kwargs):
        """Plot the empirical cumulative distribution function."""
        import numpy as np
        x_sorted = np.sort(x)
        y = np.arange(1, len(x_sorted) + 1) / len(x_sorted)
        self.step(x_sorted.tolist(), y.tolist(), where='post', **kwargs)
        self.set_ylabel('Proportion')

    def triplot(self, x, y, triangles=None, **kwargs):
        """Plot triangulation edges."""
        if triangles is not None:
            for tri in triangles:
                i, j, k = tri
                tx = [x[i], x[j], x[k], x[i]]
                ty = [y[i], y[j], y[k], y[i]]
                self.plot(tx, ty, **kwargs)
        else:
            self.plot(x, y, **kwargs)

    def tricontour(self, *args, **kwargs):
        """Contour plot sobre uma triangulação.

        Uso: tricontour(x, y, triangles, z) ou tricontour(x, y, z)
        Interpola os dados numa grade regular e chama contour().
        """
        self._tri_contour_impl(False, *args, **kwargs)

    def tricontourf(self, *args, **kwargs):
        """Filled contour plot sobre uma triangulação.

        Uso: tricontourf(x, y, triangles, z) ou tricontourf(x, y, z)
        Interpola os dados numa grade regular e chama contourf().
        """
        self._tri_contour_impl(True, *args, **kwargs)

    def _tri_contour_impl(self, filled, *args, **kwargs):
        """Implementação compartilhada para tricontour/tricontourf."""
        if len(args) >= 4:
            x, y, triangles, z = args[0], args[1], args[2], args[3]
        elif len(args) >= 3:
            x, y, z = args[0], args[1], args[2]
            triangles = None
        else:
            return

        x = np.asarray(x, dtype=float).ravel()
        y = np.asarray(y, dtype=float).ravel()
        z = np.asarray(z, dtype=float).ravel()

        if triangles is None:
            n = len(x)
            triangles = np.array([[0, i, i + 1] for i in range(1, n - 1)])
        else:
            triangles = np.asarray(triangles, dtype=int)

        levels = kwargs.pop('levels', 10)
        nx, ny = 50, 50
        xi = np.linspace(float(np.min(x)), float(np.max(x)), nx)
        yi = np.linspace(float(np.min(y)), float(np.max(y)), ny)
        Xi, Yi = np.meshgrid(xi, yi)

        Zi = _interp_triangles(x, y, z, triangles, Xi.ravel(), Yi.ravel())
        Zi = Zi.reshape(ny, nx)

        # contour/contourf esperam lista de valores, não int
        if isinstance(levels, int):
            z_valid = Zi[~np.isnan(Zi)]
            if len(z_valid) > 0:
                levels = np.linspace(float(z_valid.min()), float(z_valid.max()), levels).tolist()
            else:
                levels = None

        if filled:
            self.contourf(Xi, Yi, Zi, levels=levels, **kwargs)
        else:
            self.contour(Xi, Yi, Zi, levels=levels, **kwargs)

    def tripcolor(self, *args, **kwargs):
        """Pseudocolor plot sobre uma triangulação.

        Uso: tripcolor(x, y, triangles, C) ou tripcolor(x, y, C)

        Parâmetros
        ----------
        x, y : array-like
            Coordenadas dos vértices.
        triangles : array-like de shape (ntri, 3), opcional
            Índices dos vértices de cada triângulo. Se None, usa triangulação fan.
        C : array-like
            Valores de cor. Se len(C) == len(x), valores são por vértice (média por triângulo).
            Se len(C) == len(triangles), valores são por triângulo.
        cmap : str
            Nome do colormap (apenas 'viridis' com aproximação nativa; outros usam viridis).
        """
        if len(args) >= 4:
            x, y, triangles, C = args[0], args[1], args[2], args[3]
        elif len(args) == 3:
            x, y, C = args[0], args[1], args[2]
            triangles = None
        else:
            return

        x = np.asarray(x, dtype=float).ravel()
        y = np.asarray(y, dtype=float).ravel()
        C = np.asarray(C, dtype=float).ravel()

        if triangles is None:
            n = len(x)
            triangles = np.array([[0, i, i + 1] for i in range(1, n - 1)])
        else:
            triangles = np.asarray(triangles, dtype=int)

        # Cor por triângulo: média dos vértices se C for por vértice
        if len(C) == len(x):
            tri_vals = np.array([np.mean(C[tri]) for tri in triangles])
        else:
            tri_vals = C[:len(triangles)]

        vmin = kwargs.get('vmin', float(np.nanmin(tri_vals)))
        vmax = kwargs.get('vmax', float(np.nanmax(tri_vals)))
        alpha = kwargs.get('alpha', 1.0)

        if vmax <= vmin:
            vmax = vmin + 1.0
        norm = np.clip((tri_vals - vmin) / (vmax - vmin), 0, 1)

        for i, tri in enumerate(triangles):
            t = float(norm[i])
            r, g, b = _viridis_approx(t)
            color = f'#{r:02x}{g:02x}{b:02x}'
            tx = [float(x[tri[0]]), float(x[tri[1]]), float(x[tri[2]])]
            ty = [float(y[tri[0]]), float(y[tri[1]]), float(y[tri[2]])]
            self.fill(tx, ty, color=color, alpha=alpha)

    def matshow(self, data, cmap=None, **kwargs):
        """Display a matrix as an image with integer ticks."""
        data_list = [_to_list(row) for row in data]
        kw = {}
        if cmap is not None:
            kw["cmap"] = str(cmap)
        self._fig.axes_matshow(self._id, data_list, kw)
        return self

    def sankey(self, flows, labels=None, orientations=None, alpha=None, **kwargs):
        """Draw a basic Sankey diagram."""
        flows = _to_list(flows)
        kw = {}
        if labels is not None:
            kw["labels"] = list(labels)
        if orientations is not None:
            kw["orientations"] = list(orientations)
        if alpha is not None:
            kw["alpha"] = float(alpha)
        self._fig.axes_sankey(self._id, flows, kw)
        return self

    def arrow(self, x, y, dx, dy, head_width=None, head_length=None,
              width=None, color=None, alpha=None, label=None, zorder=None, **kwargs):
        """Draw an arrow from (x, y) to (x+dx, y+dy)."""
        kw = {}
        if color is not None:
            kw["color"] = color
        if width is not None:
            kw["width"] = float(width)
        if head_width is not None:
            kw["head_width"] = float(head_width)
        if head_length is not None:
            kw["head_length"] = float(head_length)
        if alpha is not None:
            kw["alpha"] = float(alpha)
        if label is not None:
            kw["label"] = label
        if zorder is not None:
            kw["zorder"] = int(zorder)
        self._fig.axes_arrow(self._id, float(x), float(y), float(dx), float(dy), kw)
        return self

    def axline(self, xy1, xy2=None, slope=None, color=None, linestyle=None,
               linewidth=None, alpha=None, **kwargs):
        """Draw an infinite line through xy1 with given slope or through xy1 and xy2."""
        kw = {}
        if xy2 is not None:
            kw["xy2"] = (float(xy2[0]), float(xy2[1]))
        if slope is not None:
            kw["slope"] = float(slope)
        if color is not None:
            kw["color"] = color
        if linestyle is not None:
            kw["linestyle"] = linestyle
        if linewidth is not None:
            kw["linewidth"] = float(linewidth)
        if alpha is not None:
            kw["alpha"] = float(alpha)
        self._fig.axes_axline(self._id, (float(xy1[0]), float(xy1[1])), kw)
        return self

    def indicate_inset(self, bounds, **kwargs):
        """Draw a rectangle and connecting lines to show an inset region."""
        return None  # stub

    def indicate_inset_zoom(self, inset_ax, **kwargs):
        """Draw connecting lines between this axes and an inset axes."""
        return None  # stub

    def bar_label(self, container, labels=None, fmt='%g', label_type='edge',
                  fontsize=None, **kwargs):
        """Add labels on bar chart bars."""
        pass  # stub

    def margins(self, *args, x=None, y=None, tight=True):
        """Set margins around data."""
        pass  # stub

    def specgram(self, x, NFFT=256, Fs=2, noverlap=128, cmap='viridis', **kwargs):
        """Plot a spectrogram."""
        x = np.asarray(x)

        # Compute STFT
        noverlap = min(noverlap, NFFT - 1)  # ensure step >= 1
        step = NFFT - noverlap
        num_segments = (len(x) - NFFT) // step + 1

        spec = np.zeros((NFFT // 2 + 1, num_segments))
        window = np.hanning(NFFT)

        for i in range(num_segments):
            start = i * step
            segment = x[start:start + NFFT] * window
            fft_result = np.fft.rfft(segment)
            spec[:, i] = np.abs(fft_result) ** 2

        # Convert to dB
        spec = 10 * np.log10(spec + 1e-10)

        # Plot as imshow
        freqs = np.fft.rfftfreq(NFFT, d=1.0 / Fs)
        times = np.arange(num_segments) * step / Fs

        self.imshow(spec.tolist(), cmap=cmap, origin='lower',
                    extent=[times[0], times[-1], freqs[0], freqs[-1]])
        self.set_xlabel('Time')
        self.set_ylabel('Frequency')

        return spec, freqs, times

    def acorr(self, x, maxlags=None, **kwargs):
        """Plot autocorrelation."""
        x = np.asarray(x)
        x = x - x.mean()

        if maxlags is None:
            maxlags = len(x) - 1
        maxlags = min(maxlags, len(x) - 1)

        # Full autocorrelation
        corr = np.correlate(x, x, mode='full')
        corr = corr / corr[len(x) - 1]  # normalize

        # Extract lags
        lags = np.arange(-maxlags, maxlags + 1)
        mid = len(x) - 1
        acorr_vals = corr[mid - maxlags:mid + maxlags + 1]

        self.stem(lags.tolist(), acorr_vals.tolist(), **kwargs)
        self.set_xlabel('Lag')
        self.set_ylabel('Autocorrelation')

        return lags, acorr_vals

    def xcorr(self, x, y, maxlags=None, **kwargs):
        """Plot cross-correlation."""
        x = np.asarray(x) - np.mean(x)
        y = np.asarray(y) - np.mean(y)

        if maxlags is None:
            maxlags = max(len(x), len(y)) - 1

        corr = np.correlate(x, y, mode='full')
        norm = np.sqrt(np.sum(x**2) * np.sum(y**2))
        if norm > 0:
            corr = corr / norm

        mid = len(x) - 1
        lags = np.arange(-maxlags, maxlags + 1)
        xcorr_vals = corr[mid - maxlags:mid + maxlags + 1]

        self.stem(lags.tolist(), xcorr_vals.tolist(), **kwargs)
        self.set_xlabel('Lag')
        self.set_ylabel('Cross-correlation')

        return lags, xcorr_vals

    def psd(self, x, NFFT=256, Fs=2, **kwargs):
        """Plot power spectral density."""
        x = np.asarray(x)

        freqs = np.fft.rfftfreq(NFFT, d=1.0 / Fs)

        # Welch's method
        step = NFFT // 2
        num_segments = (len(x) - NFFT) // step + 1
        window = np.hanning(NFFT)

        psd_vals = np.zeros(NFFT // 2 + 1)
        for i in range(num_segments):
            start = i * step
            segment = x[start:start + NFFT] * window
            fft_result = np.fft.rfft(segment)
            psd_vals += np.abs(fft_result) ** 2

        psd_vals /= num_segments
        psd_db = 10 * np.log10(psd_vals + 1e-10)

        self.plot(freqs.tolist(), psd_db.tolist(), **kwargs)
        self.set_xlabel('Frequency')
        self.set_ylabel('Power/Frequency (dB/Hz)')
        self.grid(True)

        return psd_vals, freqs

    def magnitude_spectrum(self, x, Fs=2, **kwargs):
        """Plot magnitude spectrum."""
        x = np.asarray(x)
        freqs = np.fft.rfftfreq(len(x), d=1.0 / Fs)
        spectrum = np.abs(np.fft.rfft(x))
        self.plot(freqs.tolist(), spectrum.tolist(), **kwargs)
        self.set_xlabel('Frequency')
        self.set_ylabel('Magnitude')
        return spectrum, freqs

    def angle_spectrum(self, x, Fs=2, **kwargs):
        """Plot angle spectrum."""
        x = np.asarray(x)
        freqs = np.fft.rfftfreq(len(x), d=1.0 / Fs)
        spectrum = np.angle(np.fft.rfft(x))
        self.plot(freqs.tolist(), spectrum.tolist(), **kwargs)
        self.set_xlabel('Frequency')
        self.set_ylabel('Angle (radians)')
        return spectrum, freqs

    def phase_spectrum(self, x, Fs=2, **kwargs):
        """Plot phase spectrum (alias for angle_spectrum)."""
        return self.angle_spectrum(x, Fs=Fs, **kwargs)

    def cohere(self, x, y, NFFT=256, Fs=2, **kwargs):
        """Plot coherence between two signals."""
        x = np.asarray(x)
        y = np.asarray(y)
        freqs = np.fft.rfftfreq(NFFT, d=1.0 / Fs)

        # Simplified coherence
        Pxx = np.abs(np.fft.rfft(x[:NFFT])) ** 2
        Pyy = np.abs(np.fft.rfft(y[:NFFT])) ** 2
        Pxy = np.abs(np.fft.rfft(x[:NFFT]) * np.conj(np.fft.rfft(y[:NFFT]))) ** 2

        coh = Pxy / (Pxx * Pyy + 1e-10)

        self.plot(freqs.tolist(), coh.tolist(), **kwargs)
        self.set_xlabel('Frequency')
        self.set_ylabel('Coherence')
        self.set_ylim(0, 1)

        return coh, freqs

    def csd(self, x, y, NFFT=256, Fs=2, **kwargs):
        """Plot cross spectral density."""
        x = np.asarray(x)
        y = np.asarray(y)
        freqs = np.fft.rfftfreq(NFFT, d=1.0 / Fs)
        Pxy = np.fft.rfft(x[:NFFT]) * np.conj(np.fft.rfft(y[:NFFT]))
        self.plot(freqs.tolist(), (10 * np.log10(np.abs(Pxy) + 1e-10)).tolist(), **kwargs)
        self.set_xlabel('Frequency')
        self.set_ylabel('Cross Spectral Density (dB)')
        return Pxy, freqs

    def hist2d(self, x, y, bins=10, cmap='viridis', **kwargs):
        """Plot a 2D histogram."""
        x = np.asarray(x)
        y = np.asarray(y)

        if isinstance(bins, int):
            bins_x = bins_y = bins
        else:
            bins_x, bins_y = bins

        H, xedges, yedges = np.histogram2d(x, y, bins=[bins_x, bins_y])

        self.imshow(H.T.tolist(), cmap=cmap, origin='lower',
                    extent=[xedges[0], xedges[-1], yedges[0], yedges[-1]])

        return H, xedges, yedges

    def semilogx(self, *args, **kwargs):
        """Plot with log scaling on the x axis."""
        self._fig.axes_set_xscale(self._id, 'log')
        return self.plot(*args, **kwargs)

    def semilogy(self, *args, **kwargs):
        """Plot with log scaling on the y axis."""
        self._fig.axes_set_yscale(self._id, 'log')
        return self.plot(*args, **kwargs)

    def loglog(self, *args, **kwargs):
        """Plot with log scaling on both axes."""
        self._fig.axes_set_xscale(self._id, 'log')
        self._fig.axes_set_yscale(self._id, 'log')
        return self.plot(*args, **kwargs)

    def get_figure(self):
        """Return the FigureProxy that owns this axes."""
        return FigureProxy(self._fig, [self])

    @property
    def figure(self):
        """Return the FigureProxy that owns this axes."""
        return FigureProxy(self._fig, [self])

    # ------------------------------------------------------------------
    # Polar axes methods
    # ------------------------------------------------------------------

    def set_theta_zero_location(self, loc, offset=0.0):
        """Set where theta=0 appears on the polar plot.

        Parameters
        ----------
        loc : str or float
            Cardinal direction ('N', 'S', 'E', 'W') or angle in degrees.
        offset : float, optional
            Additional offset in degrees (default 0).
        """
        _loc_map = {'N': 90.0, 'NW': 135.0, 'W': 180.0, 'SW': 225.0,
                    'S': 270.0, 'SE': 315.0, 'E': 0.0, 'NE': 45.0}
        if isinstance(loc, str):
            angle = _loc_map.get(loc.upper(), 0.0)
        else:
            angle = float(loc)
        self._polar_theta_zero = angle + float(offset)

    def set_theta_direction(self, direction):
        """Set direction of increasing theta.

        Parameters
        ----------
        direction : int or str
            1 or 'counterclockwise' for CCW (default),
            -1 or 'clockwise' for CW.
        """
        if direction in (-1, 'clockwise'):
            self._polar_theta_direction = -1
        else:
            self._polar_theta_direction = 1

    def set_rlabel_position(self, angle):
        """Set the angular position of radial axis labels.

        Parameters
        ----------
        angle : float
            Angle in degrees where radial tick labels appear.
        """
        self._polar_rlabel_position = float(angle)

    def set_rmax(self, rmax):
        """Set the maximum radius.

        Equivalent to set_ylim(0, rmax) on a polar axes.

        Parameters
        ----------
        rmax : float
            Maximum radial value.
        """
        self._fig.axes_set_ylim(self._id, 0.0, float(rmax))

    def set_rmin(self, rmin):
        """Set the minimum radius.

        Parameters
        ----------
        rmin : float
            Minimum radial value.
        """
        try:
            current_ylim = self._fig.axes_get_ylim(self._id)
            rmax = current_ylim[1]
        except Exception:
            rmax = 1.0
        self._fig.axes_set_ylim(self._id, float(rmin), rmax)

    def set_rticks(self, ticks, labels=None):
        """Set radial tick positions.

        Alias for set_yticks on polar axes.

        Parameters
        ----------
        ticks : list of float
            Radial tick positions.
        labels : list of str, optional
            Custom tick labels.
        """
        self._fig.axes_set_yticks(self._id, [float(t) for t in ticks])
        if labels is not None:
            self._fig.axes_set_yticklabels(self._id, [str(l) for l in labels])

    def set_thetagrids(self, angles, labels=None, fmt=None, **kwargs):
        """Set angular grid lines at given angles (in degrees).

        Parameters
        ----------
        angles : list of float
            Angular positions in degrees for grid lines.
        labels : list of str, optional
            Custom labels for each angle; defaults to '<angle>°'.
        """
        self._polar_thetagrids = [float(a) for a in angles]
        if labels is None:
            labels = [f'{a:g}°' for a in angles]
        self._polar_thetagrids_labels = list(labels)
        # Store as x-ticks so the renderer can use them if needed
        ticks_rad = [float(a) * 3.141592653589793 / 180.0 for a in angles]
        self._fig.axes_set_xticks(self._id, ticks_rad)
        self._fig.axes_set_xticklabels(self._id, [str(l) for l in labels])

    def set_rgrids(self, radii, labels=None, **kwargs):
        """Set radial grid lines at given radii.

        Parameters
        ----------
        radii : list of float
            Radial positions for grid circles.
        labels : list of str, optional
            Custom tick labels.
        """
        self._polar_rgrids = [float(r) for r in radii]
        self.set_rticks(radii, labels=labels)


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
        self._elev = 30.0
        self._azim = -60.0

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
        self._elev = float(elev)
        self._azim = float(azim)
        self._fig.axes3d_view_init(self._id, self._elev, self._azim)

    def rotate(self, d_azim, d_elev):
        """Rotate the 3D view by incremental angles."""
        self._azim += d_azim
        self._elev = max(-90, min(90, self._elev + d_elev))
        self._fig.axes3d_view_init(self._id, self._elev, self._azim)

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
    """Canvas with event callback support (matplotlib-compatible)."""

    def __init__(self):
        from rustplotlib.callback_registry import CallbackRegistry
        self._callbacks = CallbackRegistry()
        self.callbacks = self._callbacks
        self.figure = None  # set by FigureProxy
        # Auto-connect pick event processing on button press
        self._callbacks.connect("button_press_event", self._process_pick)

    def mpl_connect(self, event_name, callback):
        """Connect a callback to an event. Returns a connection id."""
        return self._callbacks.connect(event_name, callback)

    def mpl_disconnect(self, cid):
        """Disconnect a callback by its connection id."""
        self._callbacks.disconnect(cid)

    def draw(self):
        """Request a canvas redraw."""
        self._callbacks.process("draw_event")

    def draw_idle(self):
        """Request a canvas redraw at idle time."""
        self._callbacks.process("draw_event")

    def _iter_axes(self):
        """Iterate over all AxesProxy objects regardless of storage format."""
        if self.figure is None:
            return
        axes = self.figure._axes
        if isinstance(axes, AxesProxy):
            yield axes
        elif isinstance(axes, dict):
            for ax in axes.values():
                if isinstance(ax, AxesProxy):
                    yield ax
        elif isinstance(axes, (list, tuple)):
            for item in axes:
                if isinstance(item, AxesProxy):
                    yield item
                elif isinstance(item, (list, tuple)):
                    for sub in item:
                        if isinstance(sub, AxesProxy):
                            yield sub

    def _process_pick(self, mouseevent):
        """Auto-process pick events on button press for all axes."""
        for ax in self._iter_axes():
            ax.pick(mouseevent)

    def pick(self, mouseevent):
        """Process pick events for all axes in the figure."""
        self._process_pick(mouseevent)


class FigureProxy:
    """Python wrapper around RustFigure."""

    def __init__(self, rust_fig, axes_proxies):
        self._fig = rust_fig
        self._axes = axes_proxies
        self._canvas = CanvasProxy()
        self._canvas.figure = self
        self._figwidth = 6.4
        self._figheight = 4.8

    @property
    def canvas(self):
        return self._canvas

    def savefig(self, fname, dpi=None, transparent=False, format=None, bbox_inches=None, **kwargs):
        tight = bbox_inches == 'tight' if bbox_inches else False
        self._fig.savefig(str(fname), dpi, transparent, tight)

    def set_size_inches(self, w, h=None):
        if h is None and hasattr(w, "__iter__"):
            w, h = w
        self._figwidth = float(w)
        self._figheight = float(h)
        self._fig.set_size_inches(float(w), float(h))

    def suptitle(self, text, fontsize=None, **kwargs):
        self._fig.suptitle(str(text), fontsize)

    def subplots_adjust(self, hspace=None, wspace=None, **kwargs):
        self._fig.subplots_adjust(hspace, wspace)

    def tight_layout(self, pad=1.08, w_pad=None, h_pad=None, rect=None, **kwargs):
        """Automatically adjust subplot parameters for a tight layout.

        Parameters
        ----------
        pad : float
            Padding between the figure edge and the edges of subplots,
            as a fraction of the font size.
        w_pad : float, optional
            Padding (width/horizontal) between edges of adjacent subplots.
        h_pad : float, optional
            Padding (height/vertical) between edges of adjacent subplots.
        rect : tuple of 4 floats, optional
            (left, bottom, right, top) in normalized figure coordinates.
        """
        hspace = 0.3 if h_pad is None else h_pad / 72.0
        wspace = 0.3 if w_pad is None else w_pad / 72.0
        self._fig.subplots_adjust(hspace, wspace)
        self._tight = True

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
        from rustplotlib.backends import show_figure
        show_figure(self)

    def _repr_png_(self):
        """Jupyter rich display: render as PNG bytes."""
        return bytes(self._fig.render_to_png_bytes())

    def _repr_svg_(self):
        """Jupyter rich display: render as SVG string."""
        return self._fig.render_to_svg_string()

    def _repr_html_(self):
        """Jupyter rich display: render as HTML img tag with base64 PNG."""
        import base64
        png = self._repr_png_()
        b64 = base64.b64encode(png).decode('ascii')
        return f'<img src="data:image/png;base64,{b64}" />'

    def add_subplot(self, *args, projection=None, **kwargs):
        """Add a subplot to the figure. Supports projection='3d' and SubplotSpec."""
        from rustplotlib.gridspec import SubplotSpec

        # Handle SubplotSpec (GridSpec spanning)
        if len(args) == 1 and isinstance(args[0], SubplotSpec):
            spec = args[0]
            gs = spec.get_gridspec()
            nrows, ncols = gs.nrows, gs.ncols
            self._fig.setup_subplots(nrows, ncols)
            idx = self._fig.add_axes()
            ax = AxesProxy(self._fig, idx)
            # Set grid span in Rust
            self._fig.axes_set_grid_span(idx, spec.row_start, spec.row_end,
                                          spec.col_start, spec.col_end)
            return ax

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
        n = self._fig.num_axes()
        return [AxesProxy(self._fig, i) for i in range(n)]

    def get_size_inches(self):
        """Return figure size in inches as (width, height)."""
        return (self._figwidth, self._figheight)

    def get_figwidth(self):
        """Return figure width in inches."""
        return self._figwidth

    def get_figheight(self):
        """Return figure height in inches."""
        return self._figheight

    def set_figwidth(self, w):
        """Set figure width in inches."""
        self._figwidth = float(w)
        self._fig.set_size_inches(float(w), self._figheight)

    def set_figheight(self, h):
        """Set figure height in inches."""
        self._figheight = float(h)
        self._fig.set_size_inches(self._figwidth, float(h))

    def text(self, x, y, s, **kwargs):
        """Add text to the figure at position (x, y) in figure coordinates (0-1)."""
        kw = {}
        if 'fontsize' in kwargs:
            kw['fontsize'] = float(kwargs['fontsize'])
        if 'color' in kwargs:
            kw['color'] = kwargs['color']
        if 'ha' in kwargs:
            kw['ha'] = kwargs['ha']
        if 'va' in kwargs:
            kw['va'] = kwargs['va']
        if 'transform' in kwargs:
            pass  # ignored — figure coords assumed
        # Add text to first axes as a rough approximation
        n = self._fig.num_axes()
        if n > 0:
            # Convert figure coords (0-1) to a text on axes 0 via axes_text
            # This is a best-effort; figure-level text is not natively separate
            try:
                self._fig.axes_text(0, float(x), float(y), str(s), kw)
            except Exception:
                pass

    def get_dpi(self):
        return 100

    def set_dpi(self, dpi):
        pass

    def clf(self):
        pass

    def clear(self):
        pass

    def add_axes(self, rect, **kwargs):
        """Add axes at a specified position.

        Parameters
        ----------
        rect : sequence of float
            [left, bottom, width, height] in figure coordinates (0 to 1).

        Returns
        -------
        AxesProxy
        """
        projection = kwargs.get('projection', None)
        idx = self._fig.add_axes()

        if projection == '3d':
            ax3d_id = self._fig.add_subplot_3d(idx)
            ax = Axes3DProxy(self._fig, ax3d_id)
        else:
            ax = AxesProxy(self._fig, idx)

        # Set custom position via Rust
        if rect is not None and len(rect) == 4:
            self._fig.axes_set_position(idx, float(rect[0]), float(rect[1]),
                                        float(rect[2]), float(rect[3]))
        return ax

    def get_tight_layout(self):
        return getattr(self, '_tight', False)

    def set_tight_layout(self, tight):
        self._tight = bool(tight)
        if tight:
            self.tight_layout()

    @property
    def axes(self):
        n = self._fig.num_axes()
        return [AxesProxy(self._fig, i) for i in range(n)]

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

    def legend(self, *args, **kwargs):
        """Add a figure-level legend (applies to last axes)."""
        n = self._fig.num_axes()
        if n > 0:
            kw = {}
            if 'loc' in kwargs:
                kw['loc'] = kwargs['loc']
            if 'ncol' in kwargs:
                kw['ncol'] = int(kwargs['ncol'])
            self._fig.axes_legend(n - 1, kw)


def _viridis_approx(t):
    """Aproximação do colormap viridis. t em [0, 1], retorna (r, g, b) como ints 0-255."""
    t = float(np.clip(t, 0, 1))
    # Aproximação linear por partes do viridis
    if t < 0.25:
        r = int(255 * (0.267 + 0.008 * t))
        g = int(255 * (0.005 + 1.0 * t))
        b = int(255 * (0.33 + 0.46 * t))
    elif t < 0.5:
        r = int(255 * (0.27 + 0.02 * (t - 0.25)))
        g = int(255 * np.clip(0.25 + 0.8 * (t - 0.25), 0, 1))
        b = int(255 * np.clip(0.44 + 0.2 * (t - 0.25), 0, 1))
    elif t < 0.75:
        r = int(255 * np.clip(0.27 + 1.4 * (t - 0.5), 0, 1))
        g = int(255 * np.clip(0.45 + 0.9 * (t - 0.5), 0, 1))
        b = int(255 * np.clip(0.49 - 0.8 * (t - 0.5), 0, 1))
    else:
        r = int(255 * np.clip(0.69 + 0.71 * (t - 0.75), 0, 1))
        g = int(255 * np.clip(0.68 + 0.64 * (t - 0.75), 0, 1))
        b = int(255 * np.clip(0.29 - 0.86 * (t - 0.75), 0, 1))
    return (r, g, b)


def _interp_triangles(x, y, z, triangles, xi, yi):
    """Interpola dados dispersos usando coordenadas baricêntricas nos triângulos."""
    zi = np.full(len(xi), np.nan)

    for tri in triangles:
        x0, y0 = x[tri[0]], y[tri[0]]
        x1, y1 = x[tri[1]], y[tri[1]]
        x2, y2 = x[tri[2]], y[tri[2]]
        z0, z1, z2 = z[tri[0]], z[tri[1]], z[tri[2]]

        # Bounding box para rejeição rápida
        bx_min = min(x0, x1, x2)
        bx_max = max(x0, x1, x2)
        by_min = min(y0, y1, y2)
        by_max = max(y0, y1, y2)

        mask = (xi >= bx_min) & (xi <= bx_max) & (yi >= by_min) & (yi <= by_max)
        if not np.any(mask):
            continue

        denom = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2)
        if abs(denom) < 1e-12:
            continue

        px = xi[mask]
        py = yi[mask]

        l1 = ((y1 - y2) * (px - x2) + (x2 - x1) * (py - y2)) / denom
        l2 = ((y2 - y0) * (px - x2) + (x0 - x2) * (py - y2)) / denom
        l3 = 1.0 - l1 - l2

        inside = (l1 >= -1e-10) & (l2 >= -1e-10) & (l3 >= -1e-10)
        idx = np.where(mask)[0][inside]
        zi[idx] = l1[inside] * z0 + l2[inside] * z1 + l3[inside] * z2

    # Preenche NaN com vizinho mais próximo
    nan_mask = np.isnan(zi)
    if np.any(nan_mask) and not np.all(nan_mask):
        valid = ~nan_mask
        dist = (np.abs(xi[nan_mask, None] - xi[None, valid]) +
                np.abs(yi[nan_mask, None] - yi[None, valid]))
        from_valid = np.argmin(dist, axis=1)
        zi[nan_mask] = zi[valid][from_valid]

    return zi


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


def _is_array_like(obj):
    """Check if obj looks like an array (not a string)."""
    if isinstance(obj, str):
        return False
    return hasattr(obj, '__len__') or hasattr(obj, '__iter__')


def _parse_plot_args_multi(*args, **kwargs):
    """Parse matplotlib-style plot arguments, handling multiple data+format groups.

    Returns a list of (x, y, group_kwargs) tuples.
    Examples:
        plot(y)                     -> [(range(n), y, {})]
        plot(x, y)                  -> [(x, y, {})]
        plot(x, y, 'r-')           -> [(x, y, {color:'r', linestyle:'-'})]
        plot(x1, y1, 'r-', x2, y2, 'b--') -> two groups
    """
    plain_args = list(args)
    groups = []
    i = 0

    while i < len(plain_args):
        # Try to detect: x, y, fmt
        if (i + 2 < len(plain_args)
                and _is_array_like(plain_args[i])
                and _is_array_like(plain_args[i + 1])
                and isinstance(plain_args[i + 2], str)):
            # x, y, fmt group
            if _is_string_sequence(plain_args[i]):
                x = list(plain_args[i])
            else:
                x = _to_list(plain_args[i])
            y = _to_list(plain_args[i + 1])
            fmt = plain_args[i + 2]
            group_kw = dict(kwargs)
            _parse_fmt(fmt, group_kw)
            if not (x and isinstance(x[0], str)):
                x = [float(v) for v in x]
            groups.append((x, y, group_kw))
            i += 3
        elif (i + 1 < len(plain_args)
              and _is_array_like(plain_args[i])
              and _is_array_like(plain_args[i + 1])
              and not isinstance(plain_args[i + 1], str)):
            # x, y group (no fmt)
            if _is_string_sequence(plain_args[i]):
                x = list(plain_args[i])
            else:
                x = _to_list(plain_args[i])
            y = _to_list(plain_args[i + 1])
            if not (x and isinstance(x[0], str)):
                x = [float(v) for v in x]
            groups.append((x, y, dict(kwargs)))
            i += 2
        elif i < len(plain_args) and _is_array_like(plain_args[i]):
            # single y (or y, fmt)
            if (i + 1 < len(plain_args) and isinstance(plain_args[i + 1], str)):
                y = _to_list(plain_args[i])
                x = list(range(len(y)))
                fmt = plain_args[i + 1]
                group_kw = dict(kwargs)
                _parse_fmt(fmt, group_kw)
                x = [float(v) for v in x]
                groups.append((x, y, group_kw))
                i += 2
            else:
                y = _to_list(plain_args[i])
                x = list(range(len(y)))
                x = [float(v) for v in x]
                groups.append((x, y, dict(kwargs)))
                i += 1
        else:
            # Unknown arg — skip
            i += 1

    return groups


def _parse_plot_args(*args, **kwargs):
    """Parse matplotlib-style plot arguments: plot(y), plot(x, y), plot(x, y, fmt).

    Returns (x, y, kwargs) for the first group (backward compatible).
    """
    groups = _parse_plot_args_multi(*args, **kwargs)
    if groups:
        return groups[0]
    # Fallback
    return [], [], kwargs


def _parse_fmt(fmt, kwargs):
    """Parse matplotlib format string like 'r--o' into color, linestyle, marker.

    Uses Rust parse_fmt() for the actual parsing.
    """
    from rustplotlib._rustplotlib import parse_fmt as _rust_parse_fmt
    color, linestyle, marker = _rust_parse_fmt(fmt)
    if color is not None and "color" not in kwargs:
        kwargs["color"] = color
    if linestyle is not None and "linestyle" not in kwargs:
        kwargs["linestyle"] = linestyle
    if marker is not None and "marker" not in kwargs:
        kwargs["marker"] = marker


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


def figure(figsize=None, dpi=100, constrained_layout=False, tight_layout=False, **kwargs):
    """Create a new figure.

    Parameters
    ----------
    figsize : (float, float), optional
        Width and height in inches.
    dpi : float, optional
        Dots per inch.
    constrained_layout : bool, optional
        If True, use tight layout to automatically adjust subplot parameters.
    tight_layout : bool, optional
        If True, use tight layout adjustment (alias for constrained_layout).
    """
    global _current_figure, _current_axes_id
    if figsize:
        w, h = figsize
        _current_figure = RustFigure(int(w * dpi), int(h * dpi), dpi)
    else:
        _current_figure = RustFigure(640, 480, dpi)
    _current_axes_id = _current_figure.add_axes()
    _apply_current_style(_current_figure)
    fig_proxy = FigureProxy(_current_figure, [_gca()])
    if constrained_layout or tight_layout:
        fig_proxy._tight = True
    return fig_proxy


def subplots(nrows=1, ncols=1, figsize=None, dpi=100, subplot_kw=None,
             constrained_layout=False, tight_layout=False, **kwargs):
    """Create a figure and a set of subplots.

    Parameters
    ----------
    nrows, ncols : int
        Number of rows/columns of the subplot grid.
    figsize : (float, float), optional
        Width and height in inches.
    dpi : float, optional
        Dots per inch.
    subplot_kw : dict, optional
        Dict with keywords passed to add_subplot.
    constrained_layout : bool, optional
        If True, use tight layout automatically.
    tight_layout : bool, optional
        If True, use tight layout (alias for constrained_layout).
    """
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
    is_polar = subplot_kw.get('projection') == 'polar'

    def _make_ax(idx):
        if is_3d:
            ax3d_id = fig.add_subplot_3d(idx)
            return Axes3DProxy(fig, ax3d_id)
        else:
            ax = AxesProxy(fig, idx)
            if is_polar:
                fig.axes_set_polar(idx, True)
            return ax

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

    fig_proxy = FigureProxy(fig, axes)
    if constrained_layout or tight_layout:
        fig_proxy._tight = True
        fig_proxy.tight_layout()
    return fig_proxy, axes


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


def fill(*args, **kwargs):
    _gca().fill(*args, **kwargs)


def pcolormesh(*args, **kwargs):
    _gca().pcolormesh(*args, **kwargs)


def pcolor(*args, **kwargs):
    _gca().pcolor(*args, **kwargs)


def spy(Z, **kwargs):
    _gca().spy(Z, **kwargs)


def stairs(values, edges=None, **kwargs):
    _gca().stairs(values, edges=edges, **kwargs)


def ecdf(x, **kwargs):
    _gca().ecdf(x, **kwargs)


def triplot(x, y, triangles=None, **kwargs):
    _gca().triplot(x, y, triangles=triangles, **kwargs)


def tricontour(*args, **kwargs):
    _gca().tricontour(*args, **kwargs)


def tricontourf(*args, **kwargs):
    _gca().tricontourf(*args, **kwargs)


def tripcolor(*args, **kwargs):
    _gca().tripcolor(*args, **kwargs)


def matshow(data, **kwargs):
    _gca().matshow(data, **kwargs)


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


def tight_layout(pad=1.08, w_pad=None, h_pad=None, rect=None, **kwargs):
    """Adjust the padding between and around subplots.

    Parameters
    ----------
    pad : float
        Padding between figure edge and subplots, as a fraction of font size.
    w_pad : float, optional
        Horizontal padding between subplots.
    h_pad : float, optional
        Vertical padding between subplots.
    rect : tuple of 4 floats, optional
        (left, bottom, right, top) in normalized figure coordinates.
    """
    _ensure_figure()
    hspace = 0.3 if h_pad is None else h_pad / 72.0
    wspace = 0.3 if w_pad is None else w_pad / 72.0
    _current_figure.subplots_adjust(hspace, wspace)


def savefig(fname, dpi=None, transparent=False, bbox_inches=None, **kwargs):
    tight = bbox_inches == 'tight' if bbox_inches else False
    _gcf().savefig(str(fname), dpi, transparent, tight)


def show():
    from rustplotlib.backends import show_figure
    fig = gcf()
    show_figure(fig)


def close(*args):
    global _current_figure, _current_axes_id
    _current_figure = None
    _current_axes_id = None


def switch_backend(backend):
    """Switch rendering backend (compatibility stub)."""
    from rustplotlib.backends import set_backend
    set_backend(backend)


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
    if _current_figure is not None and _current_axes_id is not None:
        _current_figure.axes_clear(_current_axes_id)


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
    """Connect a callback to the current figure's canvas. Returns cid."""
    return gcf().canvas.mpl_connect(event, func)


def disconnect(cid):
    """Disconnect a callback from the current figure's canvas."""
    gcf().canvas.mpl_disconnect(cid)


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
    base = [
        # Perceptually uniform
        'viridis', 'plasma', 'inferno', 'magma', 'cividis',
        # Cyclic
        'twilight', 'twilight_shifted', 'hsv',
        # Misc sequential
        'turbo', 'hot', 'cool', 'gray', 'jet',
        'spring', 'summer', 'autumn', 'winter',
        'copper', 'bone', 'pink', 'binary', 'gist_heat',
        'ocean', 'terrain', 'afmhot', 'Wistia',
        # ColorBrewer sequential (single-hue)
        'Blues', 'Reds', 'Greens', 'Oranges', 'Purples',
        # ColorBrewer sequential (multi-hue)
        'YlOrRd', 'YlOrBr', 'YlGnBu', 'YlGn',
        'GnBu', 'PuBu', 'PuRd', 'OrRd', 'BuGn', 'BuPu',
        # ColorBrewer diverging
        'RdYlBu', 'RdBu', 'PiYG', 'PRGn', 'BrBG', 'Spectral',
        # Qualitative
        'Set1', 'Set2', 'Set3',
        'Pastel1', 'Pastel2',
        'Accent', 'Dark2', 'Paired',
        'tab10', 'tab20', 'tab20b', 'tab20c',
        # Misc (rainbow, gnuplot, gist_*)
        'rainbow', 'gist_rainbow', 'gnuplot', 'gnuplot2',
        'CMRmap', 'cubehelix', 'brg',
        'gist_earth', 'gist_stern', 'gist_ncar',
    ]
    # Add reversed variants
    reversed_cmaps = [name + '_r' for name in base]
    return base + reversed_cmaps


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


# --- Arrow ---

def arrow(x, y, dx, dy, **kwargs):
    """Draw an arrow from (x, y) to (x+dx, y+dy) on the current axes."""
    _gca().arrow(x, y, dx, dy, **kwargs)


# --- axline ---

def axline(xy1, xy2=None, slope=None, **kwargs):
    """Draw an infinite line through xy1 with given slope or through xy1 and xy2."""
    _gca().axline(xy1, xy2=xy2, slope=slope, **kwargs)


# --- subplot2grid ---

def subplot2grid(shape, loc, rowspan=1, colspan=1, fig=None, **kwargs):
    """Create subplot at specific grid location.

    Parameters:
        shape: (nrows, ncols) - grid shape
        loc: (row, col) - starting position
        rowspan: number of rows to span
        colspan: number of columns to span
    """
    nrows, ncols = shape
    row, col = loc
    return subplot(nrows, ncols, row * ncols + col + 1, **kwargs)


# --- imread / imsave ---

def imread(fname, format=None):
    """Read an image from file (requires Pillow).

    Returns a numpy array with values in [0, 1].
    """
    try:
        from PIL import Image
        img = Image.open(fname)
        return np.array(img) / 255.0
    except ImportError:
        raise ImportError("imread requires Pillow: pip install Pillow")


def imsave(fname, arr, **kwargs):
    """Save an array as an image (requires Pillow).

    Parameters:
        fname: output file path
        arr: numpy array (values in [0, 1] or [0, 255])
    """
    try:
        from PIL import Image
        if arr.max() <= 1.0:
            arr = (arr * 255).astype(np.uint8)
        img = Image.fromarray(arr.astype(np.uint8))
        img.save(fname)
    except ImportError:
        raise ImportError("imsave requires Pillow: pip install Pillow")


# --- bar_label ---

def bar_label(container, labels=None, fmt='%g', label_type='edge', fontsize=None, **kwargs):
    """Add labels on bar chart bars (module-level)."""
    _gca().bar_label(container, labels=labels, fmt=fmt, label_type=label_type,
                     fontsize=fontsize, **kwargs)


# --- Signal processing / spectral plots ---

def specgram(x, NFFT=256, Fs=2, noverlap=128, cmap='viridis', **kwargs):
    """Plot a spectrogram."""
    return _gca().specgram(x, NFFT=NFFT, Fs=Fs, noverlap=noverlap, cmap=cmap, **kwargs)


def acorr(x, maxlags=None, **kwargs):
    """Plot autocorrelation."""
    return _gca().acorr(x, maxlags=maxlags, **kwargs)


def xcorr(x, y, maxlags=None, **kwargs):
    """Plot cross-correlation."""
    return _gca().xcorr(x, y, maxlags=maxlags, **kwargs)


def psd(x, NFFT=256, Fs=2, **kwargs):
    """Plot power spectral density."""
    return _gca().psd(x, NFFT=NFFT, Fs=Fs, **kwargs)


def magnitude_spectrum(x, Fs=2, **kwargs):
    """Plot magnitude spectrum."""
    return _gca().magnitude_spectrum(x, Fs=Fs, **kwargs)


def angle_spectrum(x, Fs=2, **kwargs):
    """Plot angle spectrum."""
    return _gca().angle_spectrum(x, Fs=Fs, **kwargs)


def phase_spectrum(x, Fs=2, **kwargs):
    """Plot phase spectrum (alias for angle_spectrum)."""
    return _gca().phase_spectrum(x, Fs=Fs, **kwargs)


def cohere(x, y, NFFT=256, Fs=2, **kwargs):
    """Plot coherence between two signals."""
    return _gca().cohere(x, y, NFFT=NFFT, Fs=Fs, **kwargs)


def csd(x, y, NFFT=256, Fs=2, **kwargs):
    """Plot cross spectral density."""
    return _gca().csd(x, y, NFFT=NFFT, Fs=Fs, **kwargs)


def hist2d(x, y, bins=10, cmap='viridis', **kwargs):
    """Plot a 2D histogram."""
    return _gca().hist2d(x, y, bins=bins, cmap=cmap, **kwargs)


def semilogx(*args, **kwargs):
    """Plot with log scaling on the x axis."""
    return _gca().semilogx(*args, **kwargs)


def semilogy(*args, **kwargs):
    """Plot with log scaling on the y axis."""
    return _gca().semilogy(*args, **kwargs)


def loglog(*args, **kwargs):
    """Plot with log scaling on both axes."""
    return _gca().loglog(*args, **kwargs)
