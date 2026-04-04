"""Date handling utilities for rustplotlib — functional implementation."""

import datetime
import math


# Epoch: same as matplotlib (0001-01-01 is day 1)
_EPOCH = datetime.datetime(1970, 1, 1)


class DateFormatter:
    """Format dates on axes using strftime patterns."""

    def __init__(self, fmt="%Y-%m-%d"):
        self.fmt = fmt

    def __call__(self, x, pos=None):
        dt = num2date(x)
        return dt.strftime(self.fmt)


class AutoDateFormatter(DateFormatter):
    """Automatically choose date format based on scale."""

    def __init__(self, locator=None, **kwargs):
        super().__init__("%Y-%m-%d")
        self.locator = locator

    def __call__(self, x, pos=None):
        dt = num2date(x)
        # Auto-detect appropriate format
        if hasattr(self, '_scale'):
            if self._scale == 'hours':
                return dt.strftime("%H:%M")
            elif self._scale == 'minutes':
                return dt.strftime("%H:%M:%S")
        # Default: date only
        if dt.hour == 0 and dt.minute == 0:
            return dt.strftime("%Y-%m-%d")
        return dt.strftime("%Y-%m-%d %H:%M")


class DateLocator:
    """Base class for date locators."""

    def tick_values(self, vmin, vmax):
        return []


class AutoDateLocator(DateLocator):
    """Automatically choose tick locations based on date range."""

    def tick_values(self, vmin, vmax):
        dmin = num2date(vmin)
        dmax = num2date(vmax)
        span = (dmax - dmin).total_seconds()

        if span <= 86400:  # <= 1 day: hourly ticks
            return _generate_hour_ticks(dmin, dmax)
        elif span <= 86400 * 30:  # <= 1 month: daily ticks
            return _generate_day_ticks(dmin, dmax)
        elif span <= 86400 * 365:  # <= 1 year: monthly ticks
            return _generate_month_ticks(dmin, dmax)
        else:  # > 1 year: yearly ticks
            return _generate_year_ticks(dmin, dmax)


class DayLocator(DateLocator):
    """Locate ticks at day boundaries."""

    def __init__(self, interval=1, **kwargs):
        self.interval = interval

    def tick_values(self, vmin, vmax):
        return _generate_day_ticks(num2date(vmin), num2date(vmax), self.interval)


class MonthLocator(DateLocator):
    """Locate ticks at month boundaries."""

    def __init__(self, bymonth=None, interval=1, **kwargs):
        self.interval = interval
        self.bymonth = bymonth

    def tick_values(self, vmin, vmax):
        return _generate_month_ticks(num2date(vmin), num2date(vmax), self.interval)


class YearLocator(DateLocator):
    """Locate ticks at year boundaries."""

    def __init__(self, base=1, **kwargs):
        self.base = base

    def tick_values(self, vmin, vmax):
        return _generate_year_ticks(num2date(vmin), num2date(vmax), self.base)


class HourLocator(DateLocator):
    """Locate ticks at hour boundaries."""

    def __init__(self, interval=1, **kwargs):
        self.interval = interval

    def tick_values(self, vmin, vmax):
        return _generate_hour_ticks(num2date(vmin), num2date(vmax), self.interval)


class MinuteLocator(DateLocator):
    """Locate ticks at minute boundaries."""

    def __init__(self, interval=1, **kwargs):
        self.interval = interval

    def tick_values(self, vmin, vmax):
        ticks = []
        current = num2date(vmin).replace(second=0, microsecond=0)
        end = num2date(vmax)
        while current <= end:
            ticks.append(date2num(current))
            current += datetime.timedelta(minutes=self.interval)
        return ticks


class WeekdayLocator(DateLocator):
    """Locate ticks on specific weekdays."""

    def __init__(self, byweekday=0, **kwargs):
        self.byweekday = byweekday

    def tick_values(self, vmin, vmax):
        ticks = []
        current = num2date(vmin).replace(hour=0, minute=0, second=0, microsecond=0)
        end = num2date(vmax)
        while current <= end:
            if current.weekday() == self.byweekday:
                ticks.append(date2num(current))
            current += datetime.timedelta(days=1)
        return ticks


# --- Conversion functions ---

def date2num(dates):
    """Convert dates to matplotlib-compatible floats (days since epoch)."""
    if isinstance(dates, (list, tuple)):
        return [_date_to_num(d) for d in dates]
    if hasattr(dates, '__iter__') and not isinstance(dates, (str, datetime.datetime, datetime.date)):
        return [_date_to_num(d) for d in dates]
    return _date_to_num(dates)


def num2date(nums):
    """Convert matplotlib floats back to datetime objects."""
    if isinstance(nums, (list, tuple)):
        return [_num_to_date(n) for n in nums]
    return _num_to_date(nums)


def _date_to_num(d):
    """Convert a single date to days since epoch."""
    if isinstance(d, datetime.datetime):
        return (d - _EPOCH).total_seconds() / 86400.0
    if isinstance(d, datetime.date):
        epoch_date = _EPOCH.date()
        return (d - epoch_date).days
    return float(d)


def _num_to_date(n):
    """Convert days since epoch to datetime."""
    return _EPOCH + datetime.timedelta(days=float(n))


def datestr2num(datestr, fmt=None):
    """Convert date string to number."""
    if fmt:
        d = datetime.datetime.strptime(datestr, fmt)
    else:
        d = datetime.datetime.fromisoformat(datestr)
    return _date_to_num(d)


# --- Tick generation helpers ---

def _generate_hour_ticks(dmin, dmax, interval=1):
    ticks = []
    current = dmin.replace(minute=0, second=0, microsecond=0)
    while current <= dmax:
        if current >= dmin:
            ticks.append(date2num(current))
        current += datetime.timedelta(hours=interval)
    return ticks


def _generate_day_ticks(dmin, dmax, interval=1):
    ticks = []
    current = dmin.replace(hour=0, minute=0, second=0, microsecond=0)
    while current <= dmax:
        if current >= dmin:
            ticks.append(date2num(current))
        current += datetime.timedelta(days=interval)
    return ticks


def _generate_month_ticks(dmin, dmax, interval=1):
    ticks = []
    year, month = dmin.year, dmin.month
    while True:
        dt = datetime.datetime(year, month, 1)
        if dt > dmax:
            break
        if dt >= dmin:
            ticks.append(date2num(dt))
        month += interval
        while month > 12:
            month -= 12
            year += 1
    return ticks


def _generate_year_ticks(dmin, dmax, base=1):
    ticks = []
    year = dmin.year
    while True:
        dt = datetime.datetime(year, 1, 1)
        if dt > dmax:
            break
        if dt >= dmin:
            ticks.append(date2num(dt))
        year += base
    return ticks


# Matplotlib compat aliases
MO, TU, WE, TH, FR, SA, SU = range(7)
MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY = range(7)
