"""rustplotlib.font_manager — stub for matplotlib.font_manager compatibility."""


class FontProperties:
    """Stub FontProperties that stores kwargs but does not change fonts yet."""

    def __init__(self, family=None, weight=None, style=None, size=None, **kwargs):
        self.family = family
        self.weight = weight
        self.style = style
        self.size = size
        # Store any extra kwargs
        for k, v in kwargs.items():
            setattr(self, k, v)

    def get_family(self):
        return self.family

    def get_weight(self):
        return self.weight

    def get_style(self):
        return self.style

    def get_size(self):
        return self.size
