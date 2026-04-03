"""rustplotlib.patches — stub for matplotlib.patches compatibility."""


class Rectangle:
    """Stub Rectangle patch."""

    def __init__(self, xy, width, height, linewidth=1, edgecolor=None,
                 facecolor=None, **kwargs):
        self.xy = xy
        self.width = width
        self.height = height
        self.linewidth = linewidth
        self.edgecolor = edgecolor
        self.facecolor = facecolor
        for k, v in kwargs.items():
            setattr(self, k, v)


class Circle:
    """Stub Circle patch."""

    def __init__(self, xy, radius, **kwargs):
        self.xy = xy
        self.radius = radius
        for k, v in kwargs.items():
            setattr(self, k, v)


class Patch:
    """Stub base Patch."""

    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)
