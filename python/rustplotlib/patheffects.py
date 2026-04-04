"""Path effects for rustplotlib (compatibility module)."""


class AbstractPathEffect:
    pass


class Normal(AbstractPathEffect):
    def __init__(self, **kwargs):
        pass


class Stroke(AbstractPathEffect):
    def __init__(self, linewidth=3, foreground='black', **kwargs):
        self.linewidth = linewidth
        self.foreground = foreground


class withStroke(Stroke):
    pass


class SimplePatchShadow(AbstractPathEffect):
    def __init__(self, offset=(2, -2), shadow_rgbFace='black', alpha=0.3, **kwargs):
        self.offset = offset
        self.shadow_rgbFace = shadow_rgbFace
        self.alpha = alpha


class withSimplePatchShadow(SimplePatchShadow):
    pass


class SimpleLineShadow(AbstractPathEffect):
    def __init__(self, offset=(2, -2), shadow_color='black', alpha=0.3, **kwargs):
        pass
