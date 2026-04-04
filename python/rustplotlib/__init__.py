from rustplotlib._rustplotlib import __version__
from rustplotlib import pyplot
from rustplotlib import style
from rustplotlib import animation
from rustplotlib import widgets
from rustplotlib import dates
from rustplotlib import backends
from rustplotlib import font_manager
from rustplotlib import ticker
from rustplotlib import patches
from rustplotlib import colors
from rustplotlib import gridspec
from rustplotlib import cycler
from rustplotlib import axes
from rustplotlib import figure
from rustplotlib import collections
from rustplotlib import cm
from rustplotlib import patheffects
from rustplotlib import transforms
from rustplotlib import lines
from rustplotlib import text
from rustplotlib import spines
from rustplotlib import events
from rustplotlib import callback_registry

__all__ = ["pyplot", "style", "animation", "widgets", "dates", "backends",
           "font_manager", "ticker", "patches", "colors", "gridspec", "cycler",
           "axes", "figure", "collections", "cm", "patheffects", "transforms",
           "lines", "text", "spines", "events", "callback_registry",
           "__version__", "use"]


def use(backend):
    """Set the rendering backend (compatibility stub)."""
    from rustplotlib.backends import set_backend
    set_backend(backend)
