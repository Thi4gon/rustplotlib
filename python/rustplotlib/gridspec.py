"""GridSpec compatibility stub for rustplotlib."""


class GridSpec:
    """A grid layout specification for subplots.

    This is a compatibility stub. Subplots already work via subplots(nrows, ncols).
    """

    def __init__(self, nrows, ncols, figure=None, **kwargs):
        self.nrows = nrows
        self.ncols = ncols
        self.hspace = kwargs.get('hspace', 0.2)
        self.wspace = kwargs.get('wspace', 0.2)
        self.figure = figure

    def __getitem__(self, key):
        return SubplotSpec(self, key)


class SubplotSpec:
    """A spec for the location of a subplot in a GridSpec."""

    def __init__(self, gridspec, key):
        self.gridspec = gridspec
        self.key = key

    def get_position(self, figure=None):
        """Return a dummy position (compatibility)."""
        return (0, 0, 1, 1)
