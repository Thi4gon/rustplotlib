"""rustplotlib.patches — matplotlib.patches compatibility with Rust backend."""


class Patch:
    """Base Patch class."""

    def __init__(self, **kwargs):
        self.facecolor = kwargs.get('facecolor', None)
        self.edgecolor = kwargs.get('edgecolor', None)
        self.linewidth = kwargs.get('linewidth', 1.0)
        self.alpha = kwargs.get('alpha', 1.0)
        self.label = kwargs.get('label', None)
        for k, v in kwargs.items():
            if not hasattr(self, k):
                setattr(self, k, v)


class Rectangle(Patch):
    """Rectangle patch defined by lower-left corner (xy), width, and height."""

    def __init__(self, xy, width, height, linewidth=1, edgecolor=None,
                 facecolor=None, alpha=1.0, label=None, **kwargs):
        super().__init__(
            facecolor=facecolor, edgecolor=edgecolor,
            linewidth=linewidth, alpha=alpha, label=label, **kwargs
        )
        self.xy = xy
        self.width = width
        self.height = height


class Circle(Patch):
    """Circle patch defined by center (xy) and radius."""

    def __init__(self, xy, radius, edgecolor=None, facecolor=None,
                 linewidth=1, alpha=1.0, label=None, **kwargs):
        super().__init__(
            facecolor=facecolor, edgecolor=edgecolor,
            linewidth=linewidth, alpha=alpha, label=label, **kwargs
        )
        self.xy = xy
        self.radius = radius


class Polygon(Patch):
    """Polygon patch defined by a list of (x, y) vertices."""

    def __init__(self, xy, closed=True, edgecolor=None, facecolor=None,
                 linewidth=1, alpha=1.0, label=None, **kwargs):
        super().__init__(
            facecolor=facecolor, edgecolor=edgecolor,
            linewidth=linewidth, alpha=alpha, label=label, **kwargs
        )
        self.xy = xy
        self.closed = closed


class FancyBboxPatch(Rectangle):
    """Stub for matplotlib.patches.FancyBboxPatch (treated as Rectangle)."""

    def __init__(self, xy, width, height, boxstyle="round", **kwargs):
        super().__init__(xy, width, height, **kwargs)
        self.boxstyle = boxstyle


class Wedge(Patch):
    """Stub for matplotlib.patches.Wedge."""

    def __init__(self, center, r, theta1, theta2, **kwargs):
        super().__init__(**kwargs)
        self.center = center
        self.r = r
        self.theta1 = theta1
        self.theta2 = theta2


class FancyArrowPatch(Patch):
    """Basic FancyArrowPatch stub for matplotlib compatibility.

    Supports posA/posB point-to-point arrows and basic arrow styles.
    """

    def __init__(self, posA=None, posB=None, path=None, arrowstyle='->',
                 connectionstyle='arc3', mutation_scale=1, shrinkA=2,
                 shrinkB=2, **kwargs):
        super().__init__(**kwargs)
        self.posA = posA
        self.posB = posB
        self.path = path
        self.arrowstyle = arrowstyle
        self.connectionstyle = connectionstyle
        self.mutation_scale = mutation_scale
        self.shrinkA = shrinkA
        self.shrinkB = shrinkB


class ConnectionPatch(FancyArrowPatch):
    """Connection patch that draws an arrow between two points, optionally on different axes."""

    def __init__(self, xyA, xyB, coordsA='data', coordsB='data',
                 axesA=None, axesB=None, **kwargs):
        super().__init__(posA=xyA, posB=xyB, **kwargs)
        self.coordsA = coordsA
        self.coordsB = coordsB
        self.axesA = axesA
        self.axesB = axesB
