"""rustplotlib.ticker — stub for matplotlib.ticker compatibility."""


class FormatStrFormatter:
    """Stub FormatStrFormatter that does nothing."""

    def __init__(self, fmt=None):
        self.fmt = fmt

    def __call__(self, x, pos=None):
        if self.fmt:
            return self.fmt % x
        return str(x)


class FuncFormatter:
    """Stub FuncFormatter."""

    def __init__(self, func):
        self.func = func

    def __call__(self, x, pos=None):
        return self.func(x, pos)
