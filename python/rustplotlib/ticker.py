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
        if x == int(x):
            return str(int(x))
        return f"{x:g}"

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
        pct = x / self.xmax * 100
        if self.decimals is not None:
            return f"{pct:.{self.decimals}f}{self.symbol}"
        return f"{pct:g}{self.symbol}"

class LogFormatter(Formatter):
    def __init__(self, base=10.0, labelOnlyBase=False):
        self.base = base
        self.labelOnlyBase = labelOnlyBase
    def __call__(self, x, pos=None):
        if x <= 0:
            return ''
        exp = math.log(x, self.base)
        if abs(exp - round(exp)) < 0.01:
            return f"$10^{{{int(round(exp))}}}$"
        return f"{x:g}"

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
        if x == 0:
            return f"0{self.sep}{self.unit}"
        exp = int(math.floor(math.log10(abs(x)) / 3) * 3)
        exp = max(-24, min(24, exp))
        prefix = self._prefixes.get(exp, f'e{exp}')
        value = x / (10 ** exp)
        if self.places is not None:
            return f"{value:.{self.places}f}{self.sep}{prefix}{self.unit}"
        return f"{value:g}{self.sep}{prefix}{self.unit}"

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
        start = math.ceil(vmin / self.base) * self.base
        ticks = []
        t = start
        while t <= vmax + self.base * 0.001:
            ticks.append(round(t / self.base) * self.base)
            t += self.base
        return ticks

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
        if vmin <= 0:
            vmin = 1e-10
        log_min = math.floor(math.log(vmin, self.base))
        log_max = math.ceil(math.log(vmax, self.base))
        ticks = []
        for exp in range(log_min, log_max + 1):
            for sub in self.subs:
                val = sub * self.base ** exp
                if vmin <= val <= vmax:
                    ticks.append(val)
        return ticks

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
        import numpy as np
        return list(np.linspace(vmin, vmax, self.numticks))

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
