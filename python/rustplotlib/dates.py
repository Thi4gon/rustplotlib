"""Date handling utilities for rustplotlib."""

import datetime
import numpy as np


class DateFormatter:
    def __init__(self, fmt):
        self.fmt = fmt


class DateLocator:
    pass


class AutoDateLocator(DateLocator):
    pass


class AutoDateFormatter(DateFormatter):
    def __init__(self, locator=None, **kwargs):
        super().__init__("%Y-%m-%d")


class DayLocator(DateLocator):
    def __init__(self, **kwargs):
        pass


class MonthLocator(DateLocator):
    def __init__(self, **kwargs):
        pass


class YearLocator(DateLocator):
    def __init__(self, **kwargs):
        pass


class HourLocator(DateLocator):
    def __init__(self, **kwargs):
        pass


class MinuteLocator(DateLocator):
    def __init__(self, **kwargs):
        pass


def date2num(dates):
    """Convert dates to matplotlib-compatible floats."""
    if isinstance(dates, (list, tuple)):
        return [_date_to_num(d) for d in dates]
    return _date_to_num(dates)


def num2date(nums):
    """Convert matplotlib floats back to dates."""
    if isinstance(nums, (list, tuple)):
        return [_num_to_date(n) for n in nums]
    return _num_to_date(nums)


def _date_to_num(d):
    """Convert a single date to days since epoch (like matplotlib)."""
    if isinstance(d, datetime.datetime):
        epoch = datetime.datetime(1970, 1, 1)
        return (d - epoch).total_seconds() / 86400.0
    if isinstance(d, datetime.date):
        epoch = datetime.date(1970, 1, 1)
        return (d - epoch).days
    return float(d)


def _num_to_date(n):
    """Convert days since epoch to datetime."""
    epoch = datetime.datetime(1970, 1, 1)
    return epoch + datetime.timedelta(days=float(n))


def datestr2num(datestr, fmt=None):
    """Convert date string to number."""
    if fmt:
        d = datetime.datetime.strptime(datestr, fmt)
    else:
        d = datetime.datetime.fromisoformat(datestr)
    return _date_to_num(d)
