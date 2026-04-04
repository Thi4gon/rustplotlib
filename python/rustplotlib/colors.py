"""Color utilities and colormap stubs for rustplotlib compatibility."""


class LinearSegmentedColormap:
    """A colormap object using linear segmented data.

    This is a compatibility stub -- the actual colormap rendering uses the Rust side.
    """

    def __init__(self, name, segmentdata, N=256):
        self.name = name
        self.N = N
        self._segmentdata = segmentdata

    @staticmethod
    def from_list(name, colors, N=256):
        """Create a LinearSegmentedColormap from a list of colors.

        Parameters
        ----------
        name : str
            Name of the colormap.
        colors : list
            List of colors (strings or RGB tuples).
        N : int
            Number of entries in the colormap.
        """
        return LinearSegmentedColormap(name, colors, N)

    def __call__(self, value):
        """Return the colormap value (stub returns the input)."""
        return value


class Normalize:
    """Map data values to the [0, 1] range.

    Parameters
    ----------
    vmin : float
        Minimum data value.
    vmax : float
        Maximum data value.
    """

    def __init__(self, vmin=0, vmax=1):
        self.vmin = vmin
        self.vmax = vmax

    def __call__(self, value):
        if self.vmax == self.vmin:
            return 0.0
        return (value - self.vmin) / (self.vmax - self.vmin)


class LogNorm(Normalize):
    """Logarithmic normalization.

    Maps data values to [0, 1] using a log scale.
    """

    def __call__(self, value):
        import math
        if value <= 0 or self.vmin <= 0 or self.vmax <= 0:
            return 0.0
        if self.vmax == self.vmin:
            return 0.0
        return (math.log10(value) - math.log10(self.vmin)) / (
            math.log10(self.vmax) - math.log10(self.vmin)
        )


class BoundaryNorm:
    """Map data to discrete color levels based on boundaries.

    Compatibility stub.
    """

    def __init__(self, boundaries, ncolors, clip=False):
        self.boundaries = boundaries
        self.ncolors = ncolors
        self.clip = clip

    def __call__(self, value):
        return value


class TwoSlopeNorm:
    """Normalize data with different rates on each side of a center point.

    Useful for diverging colormaps where the center isn't at the midpoint of the data range.

    Parameters
    ----------
    vcenter : float
        The data value that maps to 0.5 in the colormap.
    vmin : float, optional
        The data value that maps to 0.0.
    vmax : float, optional
        The data value that maps to 1.0.
    """

    def __init__(self, vcenter, vmin=None, vmax=None):
        self.vcenter = vcenter
        self.vmin = vmin
        self.vmax = vmax

    def __call__(self, value):
        """Normalize value to [0, 1]."""
        import numpy as np
        value = np.asarray(value, dtype=float)
        vmin = self.vmin if self.vmin is not None else np.nanmin(value)
        vmax = self.vmax if self.vmax is not None else np.nanmax(value)

        result = np.where(
            value < self.vcenter,
            0.5 * (value - vmin) / (self.vcenter - vmin) if self.vcenter != vmin else 0.0,
            0.5 + 0.5 * (value - self.vcenter) / (vmax - self.vcenter) if vmax != self.vcenter else 1.0,
        )
        return np.clip(result, 0, 1)


class CenteredNorm:
    """Normalize data symmetrically around a center (default 0).

    Parameters
    ----------
    vcenter : float
        The center value (maps to 0.5). Default is 0.
    halfrange : float, optional
        Half the range. vmin = vcenter - halfrange, vmax = vcenter + halfrange.
    """

    def __init__(self, vcenter=0, halfrange=None):
        self.vcenter = vcenter
        self.halfrange = halfrange

    def __call__(self, value):
        """Normalize value to [0, 1]."""
        import numpy as np
        value = np.asarray(value, dtype=float)
        if self.halfrange is not None:
            hr = self.halfrange
        else:
            hr = max(abs(np.nanmax(value) - self.vcenter),
                     abs(np.nanmin(value) - self.vcenter))
        if hr == 0:
            hr = 1.0
        result = 0.5 + (value - self.vcenter) / (2 * hr)
        return np.clip(result, 0, 1)
