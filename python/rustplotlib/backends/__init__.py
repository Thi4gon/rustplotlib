"""Backend system for rustplotlib."""

_current_backend = "agg"

def get_backend():
    return _current_backend

from rustplotlib.backends.backend_pdf import PdfPages
