"""Compatibility stub for matplotlib.collections module."""


class LineCollection:
    """Stub for matplotlib.collections.LineCollection."""

    def __init__(self, segments, **kwargs):
        self.segments = segments

    def set_array(self, a):
        pass

    def set_cmap(self, cmap):
        pass

    def set_clim(self, vmin, vmax):
        pass

    def set_linewidth(self, lw):
        pass

    def set_color(self, c):
        pass

    def set_alpha(self, a):
        pass


class PathCollection:
    """Stub for matplotlib.collections.PathCollection."""

    def __init__(self, **kwargs):
        pass

    def set_offsets(self, offsets):
        pass

    def set_array(self, a):
        pass

    def set_clim(self, vmin, vmax):
        pass

    def set_sizes(self, s):
        pass


class PatchCollection:
    """Stub for matplotlib.collections.PatchCollection."""

    def __init__(self, patches, **kwargs):
        self.patches = patches

    def set_array(self, a):
        pass

    def set_cmap(self, cmap):
        pass

    def set_clim(self, vmin, vmax):
        pass

    def set_alpha(self, a):
        pass

    def set_edgecolor(self, c):
        pass

    def set_facecolor(self, c):
        pass
