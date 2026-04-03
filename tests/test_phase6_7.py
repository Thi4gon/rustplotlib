"""Tests for Phase 6 (backends) and Phase 7 (data integration) features."""

import datetime
import math
import os
import tempfile

import numpy as np
import pytest


# ── Phase 6: Backend stubs ──────────────────────────────────────────────────

class TestBackendSystem:
    def test_get_backend_default(self):
        from rustplotlib import backends
        assert backends.get_backend() == "agg"

    def test_switch_backend_via_backends(self):
        from rustplotlib import backends
        backends._current_backend = "svg"
        assert backends.get_backend() == "svg"
        backends._current_backend = "agg"  # reset

    def test_use_from_init(self):
        import rustplotlib
        rustplotlib.use("SVG")
        from rustplotlib import backends
        assert backends.get_backend() == "svg"
        backends._current_backend = "agg"  # reset

    def test_use_from_pyplot(self):
        from rustplotlib import pyplot as plt
        plt.use("PDF")
        from rustplotlib import backends
        assert backends.get_backend() == "pdf"
        backends._current_backend = "agg"  # reset

    def test_switch_backend_from_pyplot(self):
        from rustplotlib import pyplot as plt
        plt.switch_backend("TkAgg")
        from rustplotlib import backends
        assert backends.get_backend() == "tkagg"
        backends._current_backend = "agg"  # reset

    def test_backend_inline_import(self):
        from rustplotlib.backends.backend_inline import configure_inline_support
        # Should not raise
        configure_inline_support(None, "inline")


# ── Phase 7: Data Integration ───────────────────────────────────────────────

class TestPandasIntegration:
    """Tests for pandas Series/DataFrame support (skipped if pandas not installed)."""

    @pytest.fixture(autouse=True)
    def _check_pandas(self):
        pytest.importorskip("pandas")

    def test_to_list_series(self):
        import pandas as pd
        from rustplotlib.pyplot import _to_list

        s = pd.Series([1, 2, 3])
        result = _to_list(s)
        assert result == [1.0, 2.0, 3.0]

    def test_to_list_index(self):
        import pandas as pd
        from rustplotlib.pyplot import _to_list

        idx = pd.Index([10, 20, 30])
        result = _to_list(idx)
        assert result == [10.0, 20.0, 30.0]

    def test_to_2d_list_dataframe(self):
        import pandas as pd
        from rustplotlib.pyplot import _to_2d_list

        df = pd.DataFrame({"a": [1, 2], "b": [3, 4]})
        result = _to_2d_list(df)
        assert result == [[1.0, 3.0], [2.0, 4.0]]

    def test_plot_with_series(self):
        import pandas as pd
        from rustplotlib import pyplot as plt

        plt.close()
        s = pd.Series([1.0, 4.0, 9.0])
        fig, ax = plt.subplots()
        ax.plot(s)  # Should not raise


class TestNaNHandling:
    """Tests for NaN gap handling in line plots."""

    def test_plot_with_nan(self):
        from rustplotlib import pyplot as plt

        plt.close()
        fig, ax = plt.subplots()
        x = [0.0, 1.0, 2.0, 3.0, 4.0]
        y = [1.0, 2.0, float('nan'), 4.0, 5.0]
        ax.plot(x, y)
        # Should not raise — NaN creates a gap in the line

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_plot_with_inf(self):
        from rustplotlib import pyplot as plt

        plt.close()
        fig, ax = plt.subplots()
        x = [0.0, 1.0, 2.0, 3.0]
        y = [1.0, float('inf'), 3.0, 4.0]
        ax.plot(x, y)

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_plot_all_nan(self):
        from rustplotlib import pyplot as plt

        plt.close()
        fig, ax = plt.subplots()
        x = [0.0, 1.0, 2.0]
        y = [float('nan'), float('nan'), float('nan')]
        ax.plot(x, y)

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestDateHandling:
    """Tests for date utility functions."""

    def test_date2num_datetime(self):
        from rustplotlib.dates import date2num
        d = datetime.datetime(1970, 1, 2)
        assert date2num(d) == pytest.approx(1.0)

    def test_date2num_date(self):
        from rustplotlib.dates import date2num
        d = datetime.date(1970, 1, 2)
        assert date2num(d) == 1

    def test_date2num_list(self):
        from rustplotlib.dates import date2num
        dates = [datetime.date(1970, 1, 1), datetime.date(1970, 1, 11)]
        result = date2num(dates)
        assert result == [0, 10]

    def test_num2date(self):
        from rustplotlib.dates import num2date
        result = num2date(1.0)
        assert result == datetime.datetime(1970, 1, 2)

    def test_num2date_list(self):
        from rustplotlib.dates import num2date
        result = num2date([0, 1])
        assert result[0] == datetime.datetime(1970, 1, 1)
        assert result[1] == datetime.datetime(1970, 1, 2)

    def test_datestr2num_iso(self):
        from rustplotlib.dates import datestr2num
        result = datestr2num("1970-01-02")
        assert result == pytest.approx(1.0)

    def test_datestr2num_custom_fmt(self):
        from rustplotlib.dates import datestr2num
        result = datestr2num("02/01/1970", fmt="%d/%m/%Y")
        assert result == pytest.approx(1.0)

    def test_roundtrip(self):
        from rustplotlib.dates import date2num, num2date
        original = datetime.datetime(2025, 6, 15, 12, 0, 0)
        n = date2num(original)
        restored = num2date(n)
        assert abs((original - restored).total_seconds()) < 1

    def test_locators_instantiate(self):
        from rustplotlib.dates import (
            AutoDateLocator, DayLocator, MonthLocator, YearLocator,
            HourLocator, MinuteLocator
        )
        # All should be instantiable without errors
        AutoDateLocator()
        DayLocator()
        MonthLocator()
        YearLocator()
        HourLocator()
        MinuteLocator()

    def test_formatters_instantiate(self):
        from rustplotlib.dates import DateFormatter, AutoDateFormatter
        f = DateFormatter("%Y-%m-%d")
        assert f.fmt == "%Y-%m-%d"
        af = AutoDateFormatter()
        assert af.fmt == "%Y-%m-%d"


class TestCategoricalAxes:
    """Tests for string-based (categorical) x values."""

    def test_handle_categorical_strings(self):
        from rustplotlib.pyplot import _handle_categorical

        positions, labels = _handle_categorical(["A", "B", "C"])
        assert positions == [0, 1, 2]
        assert labels == ["A", "B", "C"]

    def test_handle_categorical_numeric(self):
        from rustplotlib.pyplot import _handle_categorical

        positions, labels = _handle_categorical([1.0, 2.0, 3.0])
        assert labels is None
        assert positions == [1.0, 2.0, 3.0]

    def test_bar_with_string_x(self):
        from rustplotlib import pyplot as plt

        plt.close()
        fig, ax = plt.subplots()
        ax.bar(["Mon", "Tue", "Wed"], [10, 20, 15])

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_plot_with_string_x(self):
        from rustplotlib import pyplot as plt

        plt.close()
        fig, ax = plt.subplots()
        ax.plot(["Jan", "Feb", "Mar"], [100, 200, 150])

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestModuleExports:
    """Test that new modules are properly exposed."""

    def test_dates_import(self):
        from rustplotlib import dates
        assert hasattr(dates, 'date2num')
        assert hasattr(dates, 'num2date')

    def test_backends_import(self):
        from rustplotlib import backends
        assert hasattr(backends, 'get_backend')
