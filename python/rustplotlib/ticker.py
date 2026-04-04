"""Tick formatters and locators for rustplotlib."""

import math

# ============================================================
# FORMATTERS
# ============================================================

class Formatter:
    """Base class for tick formatters."""
    def __call__(self, x, pos=None):
        return self.format_data(x)
    def format_data(self, value):
        return str(value)

class ScalarFormatter(Formatter):
    def __init__(self, useOffset=True, useMathText=False, useLocale=False):
        self.useOffset = useOffset
        self.useMathText = useMathText
    def __call__(self, x, pos=None):
        from rustplotlib._rustplotlib import format_tick_scalar
        return format_tick_scalar(float(x))

class FormatStrFormatter(Formatter):
    def __init__(self, fmt):
        self.fmt = fmt
    def __call__(self, x, pos=None):
        return self.fmt % x

class FuncFormatter(Formatter):
    def __init__(self, func):
        self.func = func
    def __call__(self, x, pos=None):
        return self.func(x, pos)

class StrMethodFormatter(Formatter):
    def __init__(self, fmt):
        self.fmt = fmt
    def __call__(self, x, pos=None):
        return self.fmt.format(x=x, pos=pos)

class PercentFormatter(Formatter):
    def __init__(self, xmax=100, decimals=None, symbol='%'):
        self.xmax = xmax
        self.decimals = decimals
        self.symbol = symbol
    def __call__(self, x, pos=None):
        from rustplotlib._rustplotlib import format_tick_percent
        return format_tick_percent(float(x), float(self.xmax), self.decimals)

class LogFormatter(Formatter):
    def __init__(self, base=10.0, labelOnlyBase=False):
        self.base = base
        self.labelOnlyBase = labelOnlyBase
    def __call__(self, x, pos=None):
        if x <= 0:
            return ''
        from rustplotlib._rustplotlib import format_tick_log
        return format_tick_log(float(x), float(self.base))

class LogFormatterSciNotation(LogFormatter):
    pass

class LogFormatterMathtext(LogFormatter):
    pass

class EngFormatter(Formatter):
    """Engineering notation: 1k, 1M, 1G, etc."""
    _prefixes = {
        -24: 'y', -21: 'z', -18: 'a', -15: 'f', -12: 'p',
        -9: 'n', -6: 'u', -3: 'm', 0: '', 3: 'k', 6: 'M',
        9: 'G', 12: 'T', 15: 'P', 18: 'E', 21: 'Z', 24: 'Y',
    }
    def __init__(self, unit='', places=None, sep=' '):
        self.unit = unit
        self.places = places
        self.sep = sep
    def __call__(self, x, pos=None):
        from rustplotlib._rustplotlib import format_tick_engineering
        result = format_tick_engineering(float(x), self.places)
        if self.unit:
            return f"{result}{self.sep}{self.unit}"
        return result

class NullFormatter(Formatter):
    def __call__(self, x, pos=None):
        return ''

class FixedFormatter(Formatter):
    def __init__(self, seq):
        self.seq = list(seq)
    def __call__(self, x, pos=None):
        if pos is not None and 0 <= pos < len(self.seq):
            return str(self.seq[pos])
        return ''

# ============================================================
# LOCATORS
# ============================================================

class Locator:
    """Base class for tick locators."""
    def __call__(self):
        return self.tick_values(0, 1)
    def tick_values(self, vmin, vmax):
        return []
    def set_params(self, **kwargs):
        pass

class MaxNLocator(Locator):
    def __init__(self, nbins='auto', steps=None, integer=False, **kwargs):
        self.nbins = 9 if nbins == 'auto' else nbins
        self.integer = integer
    def tick_values(self, vmin, vmax):
        from rustplotlib._rustplotlib import auto_ticks
        return auto_ticks(vmin, vmax)

class MultipleLocator(Locator):
    def __init__(self, base=1.0):
        self.base = base
    def tick_values(self, vmin, vmax):
        from rustplotlib._rustplotlib import tick_values_multiple
        return tick_values_multiple(float(vmin), float(vmax), float(self.base))

class FixedLocator(Locator):
    def __init__(self, locs):
        self.locs = list(locs)
    def tick_values(self, vmin, vmax):
        return [t for t in self.locs if vmin <= t <= vmax]

class LogLocator(Locator):
    def __init__(self, base=10.0, subs=None, numticks=None):
        self.base = base
        self.subs = subs or [1.0]
    def tick_values(self, vmin, vmax):
        from rustplotlib._rustplotlib import tick_values_log
        return tick_values_log(float(max(vmin, 1e-10)), float(vmax), float(self.base))

class AutoLocator(MaxNLocator):
    def __init__(self):
        super().__init__(nbins='auto')

class AutoMinorLocator(Locator):
    def __init__(self, n=None):
        self.n = n or 4
    def tick_values(self, vmin, vmax):
        return []  # Minor ticks computed relative to major ticks

class NullLocator(Locator):
    def tick_values(self, vmin, vmax):
        return []

class LinearLocator(Locator):
    def __init__(self, numticks=None):
        self.numticks = numticks or 11
    def tick_values(self, vmin, vmax):
        from rustplotlib._rustplotlib import tick_values_linear
        return tick_values_linear(float(vmin), float(vmax), int(self.numticks))

class IndexLocator(Locator):
    def __init__(self, base, offset=0):
        self.base = base
        self.offset = offset
    def tick_values(self, vmin, vmax):
        ticks = []
        t = self.offset
        while t <= vmax:
            if t >= vmin:
                ticks.append(t)
            t += self.base
        return ticks
