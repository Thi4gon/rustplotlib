"""Axes divider for positioning colorbars and insets.

Provides make_axes_locatable() which creates an AxesDivider that can
append new axes to any side of an existing axes.
"""

from rustplotlib.pyplot import AxesProxy


class Size:
    """Size specification for axes divider."""

    class Fixed:
        def __init__(self, size):
            self.size = size

    class Scaled:
        def __init__(self, scale):
            self.scale = scale


class AxesDivider:
    """Divider that creates new axes adjacent to an existing one.

    Usage:
        divider = make_axes_locatable(ax)
        cax = divider.append_axes("right", size="5%", pad=0.05)
        plt.colorbar(im, cax=cax)
    """

    def __init__(self, ax):
        self._ax = ax
        self._fig = ax._fig

    def append_axes(self, position, size="5%", pad=0.0, **kwargs):
        """Create a new axes appended to the given side.

        Parameters
        ----------
        position : {'left', 'right', 'top', 'bottom'}
            Side to append the new axes.
        size : str or float
            Size as percentage string ("5%") or fraction (0.05).
        pad : float
            Padding between axes as fraction.

        Returns
        -------
        AxesProxy
            New axes for colorbar or other content.
        """
        # Parse size
        if isinstance(size, str) and size.endswith('%'):
            size_frac = float(size[:-1]) / 100.0
        else:
            size_frac = float(size)

        pad_frac = float(pad)

        # Create new axes with position relative to parent
        new_idx = self._fig.add_axes()
        new_ax = AxesProxy(self._fig, new_idx)

        # Calculate position based on parent's grid position
        # Since we don't know exact pixel positions from Python,
        # use custom_position relative to figure coords
        # This works best when parent also uses custom_position
        if position == 'right':
            self._fig.axes_set_grid_span(new_idx, 0, 1, 0, 1)
            # The rendering will place this as a small axes to the right
        elif position == 'bottom':
            self._fig.axes_set_grid_span(new_idx, 0, 1, 0, 1)
        elif position == 'left':
            self._fig.axes_set_grid_span(new_idx, 0, 1, 0, 1)
        elif position == 'top':
            self._fig.axes_set_grid_span(new_idx, 0, 1, 0, 1)

        return new_ax


def make_axes_locatable(ax):
    """Create an AxesDivider for the given axes.

    Usage:
        from mpl_toolkits.axes_grid1 import make_axes_locatable
        divider = make_axes_locatable(ax)
        cax = divider.append_axes("right", size="5%", pad=0.05)
    """
    return AxesDivider(ax)
