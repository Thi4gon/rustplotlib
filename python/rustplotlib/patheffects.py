"""Path effects for rustplotlib — functional implementation.

Supports the most common matplotlib path effects:
- Normal: default rendering (no effect)
- Stroke / withStroke: draws an outline behind the artist
- SimplePatchShadow / withSimplePatchShadow: shadow behind patches
- SimpleLineShadow / withSimpleLineShadow: shadow behind lines
"""


class AbstractPathEffect:
    """Base class for all path effects."""

    def __init__(self, offset=(0, 0)):
        self._offset = offset
        self.offset = offset

    def get_offset(self):
        return self._offset


class Normal(AbstractPathEffect):
    """Default rendering — no additional effect."""

    def __init__(self, **kwargs):
        super().__init__()


class Stroke(AbstractPathEffect):
    """Draw a stroke (outline) around the artist.

    Parameters
    ----------
    linewidth : float
        Width of the outline stroke.
    foreground : str
        Color of the outline stroke.
    alpha : float, optional
        Opacity of the stroke (0-1).
    """

    def __init__(self, linewidth=3, foreground='black', alpha=None, **kwargs):
        super().__init__(offset=kwargs.get('offset', (0, 0)))
        self.linewidth = linewidth
        self.foreground = foreground
        self.alpha = alpha

    def get_outline_params(self):
        """Return (color, width) tuple for Rust rendering."""
        return self.foreground, self.linewidth


class withStroke(Stroke):
    """Convenience alias for Stroke (matplotlib compatibility).

    Usage:
        ax.plot(x, y, path_effects=[withStroke(linewidth=3, foreground='black')])
    """
    pass


class SimplePatchShadow(AbstractPathEffect):
    """Draw a shadow behind patch artists.

    Parameters
    ----------
    offset : tuple
        (x, y) offset for the shadow.
    shadow_rgbFace : str
        Color of the shadow.
    alpha : float
        Opacity of the shadow.
    """

    def __init__(self, offset=(2, -2), shadow_rgbFace='black', alpha=0.3, **kwargs):
        super().__init__(offset=offset)
        self.shadow_rgbFace = shadow_rgbFace
        self.alpha = alpha


class withSimplePatchShadow(SimplePatchShadow):
    """Convenience alias for SimplePatchShadow."""
    pass


class SimpleLineShadow(AbstractPathEffect):
    """Draw a shadow behind line artists.

    Parameters
    ----------
    offset : tuple
        (x, y) offset for the shadow.
    shadow_color : str
        Color of the shadow.
    alpha : float
        Opacity of the shadow.
    linewidth : float, optional
        Width of the shadow line. If None, uses the artist's linewidth.
    """

    def __init__(self, offset=(2, -2), shadow_color='black', alpha=0.3,
                 linewidth=None, **kwargs):
        super().__init__(offset=offset)
        self.shadow_color = shadow_color
        self.alpha = alpha
        self.linewidth = linewidth


class withSimpleLineShadow(SimpleLineShadow):
    """Convenience alias for SimpleLineShadow."""
    pass
