"""Cycler compatibility stub for rustplotlib."""


class cycler:
    """A simple property cycle specification.

    This is a compatibility stub for matplotlib's cycler.

    Parameters
    ----------
    **kwargs : dict
        Property name to list-of-values mapping.
        Example: cycler(color=['r', 'g', 'b'])
    """

    def __init__(self, **kwargs):
        self.props = kwargs

    def __add__(self, other):
        """Combine two cyclers (compatibility stub)."""
        merged = dict(self.props)
        merged.update(other.props)
        return cycler(**merged)

    def __mul__(self, other):
        """Product of two cyclers (compatibility stub)."""
        merged = dict(self.props)
        merged.update(other.props)
        return cycler(**merged)

    def __iter__(self):
        """Iterate over the property cycle."""
        if not self.props:
            return iter([])
        keys = list(self.props.keys())
        n = max(len(v) for v in self.props.values())
        for i in range(n):
            d = {}
            for k in keys:
                vals = self.props[k]
                d[k] = vals[i % len(vals)]
            yield d

    def __len__(self):
        if not self.props:
            return 0
        return max(len(v) for v in self.props.values())
