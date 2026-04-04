"""GridSpec for rustplotlib — supports rowspan/colspan via Rust grid_span."""


class GridSpec:
    """A grid layout specification for subplots with spanning support.

    Usage:
        gs = GridSpec(3, 3)
        ax1 = fig.add_subplot(gs[0, :])      # top row, all columns
        ax2 = fig.add_subplot(gs[1:, 0])      # left column, rows 1-2
        ax3 = fig.add_subplot(gs[1:, 1:])     # bottom-right block
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
    """A spec for the location of a subplot in a GridSpec, with row/col spanning."""

    def __init__(self, gridspec, key):
        self.gridspec = gridspec
        self._parse_key(key)

    def _parse_key(self, key):
        """Parse the indexing key into (row_start, row_end, col_start, col_end)."""
        nrows = self.gridspec.nrows
        ncols = self.gridspec.ncols

        if isinstance(key, tuple) and len(key) == 2:
            row_key, col_key = key
        elif isinstance(key, (int, slice)):
            # Single index: interpret as flat index or row slice
            row_key = key
            col_key = slice(None)
        else:
            row_key = slice(None)
            col_key = slice(None)

        self.row_start, self.row_end = self._resolve_slice(row_key, nrows)
        self.col_start, self.col_end = self._resolve_slice(col_key, ncols)

    def _resolve_slice(self, key, length):
        """Convert int or slice to (start, end) range."""
        if isinstance(key, int):
            if key < 0:
                key += length
            return key, key + 1
        elif isinstance(key, slice):
            start = key.start if key.start is not None else 0
            stop = key.stop if key.stop is not None else length
            if start < 0:
                start += length
            if stop < 0:
                stop += length
            return start, stop
        return 0, length

    def get_gridspec(self):
        return self.gridspec

    @property
    def rowspan(self):
        return range(self.row_start, self.row_end)

    @property
    def colspan(self):
        return range(self.col_start, self.col_end)
