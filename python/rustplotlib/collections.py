"""Collections for rustplotlib — backed by Rust rendering.

LineCollection renders multiple line segments as a single Rust artist.
PatchCollection renders multiple patches as a batch.
"""


class LineCollection:
    """Collection of line segments rendered by Rust.

    Each segment is a list of (x, y) points.
    Compatible with matplotlib.collections.LineCollection.
    """

    def __init__(self, segments, colors=None, linewidths=None,
                 color=None, linewidth=None, alpha=None, label=None,
                 cmap=None, norm=None, **kwargs):
        self.segments = list(segments)
        self._color = color
        self._colors = colors
        self._linewidth = linewidth or 1.0
        self._linewidths = linewidths
        self._alpha = alpha or 1.0
        self._label = label
        self._cmap = cmap
        self._array = None

    def set_array(self, a):
        """Set array for colormap mapping."""
        self._array = a

    def set_cmap(self, cmap):
        self._cmap = cmap

    def set_clim(self, vmin, vmax):
        self._clim = (vmin, vmax)

    def set_linewidth(self, lw):
        self._linewidth = lw

    def set_linewidths(self, lws):
        self._linewidths = lws

    def set_color(self, c):
        self._color = c

    def set_colors(self, cs):
        self._colors = cs

    def set_alpha(self, a):
        self._alpha = a

    def set_label(self, label):
        self._label = label


class PathCollection:
    """Collection of paths (scatter points) — backed by Rust scatter artist."""

    def __init__(self, **kwargs):
        self._offsets = []
        self._sizes = None
        self._array = None

    def set_offsets(self, offsets):
        self._offsets = list(offsets)

    def set_array(self, a):
        self._array = a

    def set_clim(self, vmin, vmax):
        self._clim = (vmin, vmax)

    def set_sizes(self, s):
        self._sizes = s


class PatchCollection:
    """Collection of patches — renders multiple patches as batch.

    Compatible with matplotlib.collections.PatchCollection.
    """

    def __init__(self, patches, match_original=False, **kwargs):
        self.patches = list(patches)
        self.match_original = match_original
        self._facecolor = kwargs.get('facecolor', None)
        self._edgecolor = kwargs.get('edgecolor', None)
        self._alpha = kwargs.get('alpha', 1.0)
        self._cmap = kwargs.get('cmap', None)
        self._array = None

    def set_array(self, a):
        self._array = a

    def set_cmap(self, cmap):
        self._cmap = cmap

    def set_clim(self, vmin, vmax):
        self._clim = (vmin, vmax)

    def set_alpha(self, a):
        self._alpha = a

    def set_edgecolor(self, c):
        self._edgecolor = c

    def set_facecolor(self, c):
        self._facecolor = c

    def set_linewidth(self, lw):
        self._linewidth = lw
