from rustplotlib._rustplotlib import __version__
from rustplotlib import pyplot
from rustplotlib import style
from rustplotlib import colors
from rustplotlib import gridspec
from rustplotlib import cycler
from rustplotlib import animation
from rustplotlib import widgets
from rustplotlib import dates
from rustplotlib import backends

__all__ = ["pyplot", "style", "colors", "gridspec", "cycler", "animation", "widgets",
           "dates", "backends", "__version__"]


def use(backend):
    """Set the rendering backend (compatibility stub)."""
    from rustplotlib import backends
    backends._current_backend = backend.lower()
