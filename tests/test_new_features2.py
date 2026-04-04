"""Tests for new features: arrows, axline, title loc, multi-group plot, imread/imsave, stubs."""

import pytest
import numpy as np
import os
import tempfile

from rustplotlib import pyplot as plt


class TestArrow:
    """Test ax.arrow() and plt.arrow()."""

    def setup_method(self):
        plt.close()

    def test_arrow_basic(self):
        fig, ax = plt.subplots()
        ax.arrow(0, 0, 1, 1)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_arrow_with_params(self):
        fig, ax = plt.subplots()
        ax.arrow(0.5, 0.5, 2.0, 1.0, head_width=0.3, head_length=0.2,
                 width=3.0, color='red', alpha=0.7)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_arrow_module_level(self):
        plt.figure()
        plt.arrow(0, 0, 3, 4, color='blue')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_arrow_returns_self(self):
        fig, ax = plt.subplots()
        result = ax.arrow(0, 0, 1, 1)
        assert result is ax


class TestAxline:
    """Test ax.axline() and plt.axline()."""

    def setup_method(self):
        plt.close()

    def test_axline_with_slope(self):
        fig, ax = plt.subplots()
        ax.plot([0, 5], [0, 5])
        ax.axline((0, 0), slope=1.0, color='red', linestyle='--')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_axline_with_two_points(self):
        fig, ax = plt.subplots()
        ax.plot([0, 10], [0, 10])
        ax.axline((1, 1), xy2=(5, 5), color='blue')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_axline_module_level(self):
        plt.figure()
        plt.plot([0, 5], [0, 5])
        plt.axline((0, 0), slope=2.0)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_axline_returns_self(self):
        fig, ax = plt.subplots()
        ax.plot([0, 5], [0, 5])
        result = ax.axline((0, 0), slope=1.0)
        assert result is ax


class TestTitleLoc:
    """Test set_title with loc parameter."""

    def setup_method(self):
        plt.close()

    def test_title_center(self):
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_title("Centered Title", loc="center")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_title_left(self):
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_title("Left Title", loc="left")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_title_right(self):
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_title("Right Title", loc="right")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_title_default_no_loc(self):
        """Title without loc should still work (defaults to center)."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_title("Default Title")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestMultiGroupPlot:
    """Test plot(x1, y1, 'r-', x2, y2, 'b--') style multi-group plotting."""

    def setup_method(self):
        plt.close()

    def test_two_groups_with_fmt(self):
        fig, ax = plt.subplots()
        x1 = [1, 2, 3]
        y1 = [1, 4, 9]
        x2 = [1, 2, 3]
        y2 = [2, 5, 10]
        result = ax.plot(x1, y1, 'r-', x2, y2, 'b--')
        assert len(result) == 2
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_two_groups_no_fmt(self):
        fig, ax = plt.subplots()
        x1 = [1, 2, 3]
        y1 = [1, 4, 9]
        x2 = [1, 2, 3]
        y2 = [2, 5, 10]
        result = ax.plot(x1, y1, x2, y2)
        assert len(result) == 2

    def test_single_group_still_works(self):
        fig, ax = plt.subplots()
        result = ax.plot([1, 2, 3], [1, 4, 9])
        assert len(result) == 1

    def test_single_y_still_works(self):
        fig, ax = plt.subplots()
        result = ax.plot([1, 4, 9])
        assert len(result) == 1

    def test_y_with_fmt_still_works(self):
        fig, ax = plt.subplots()
        result = ax.plot([1, 4, 9], 'r-')
        assert len(result) == 1

    def test_module_level_multi_group(self):
        plt.figure()
        result = plt.plot([1, 2, 3], [1, 4, 9], 'r-', [1, 2, 3], [2, 5, 10], 'b--')
        assert len(result) == 2


class TestSetTicksMinor:
    """Test set_xticks and set_yticks with minor parameter."""

    def setup_method(self):
        plt.close()

    def test_set_xticks_minor_false(self):
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 4, 9])
        ax.set_xticks([1, 2, 3], minor=False)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_set_xticks_minor_true(self):
        """Minor ticks should be silently accepted (no crash)."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 4, 9])
        ax.set_xticks([1.5, 2.5], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_set_yticks_minor_true(self):
        """Minor ticks should be silently accepted (no crash)."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 4, 9])
        ax.set_yticks([2, 6], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_set_xticks_with_labels(self):
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 4, 9])
        ax.set_xticks([1, 2, 3], labels=['a', 'b', 'c'])
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestSubplot2grid:
    """Test subplot2grid."""

    def setup_method(self):
        plt.close()

    def test_subplot2grid_basic(self):
        plt.figure()
        ax = plt.subplot2grid((2, 2), (0, 0))
        assert ax is not None
        ax.plot([1, 2, 3], [1, 4, 9])

    def test_subplot2grid_different_positions(self):
        plt.figure()
        ax1 = plt.subplot2grid((2, 2), (0, 0))
        ax2 = plt.subplot2grid((2, 2), (0, 1))
        ax3 = plt.subplot2grid((2, 2), (1, 0))
        assert ax1 is not None
        assert ax2 is not None
        assert ax3 is not None


class TestStubs:
    """Test stub methods that should exist without crashing."""

    def setup_method(self):
        plt.close()

    def test_indicate_inset(self):
        fig, ax = plt.subplots()
        result = ax.indicate_inset([0.1, 0.1, 0.5, 0.5])
        assert result is None

    def test_indicate_inset_zoom(self):
        fig, ax = plt.subplots()
        result = ax.indicate_inset_zoom(ax)
        assert result is None

    def test_bar_label(self):
        fig, ax = plt.subplots()
        container = ax.bar([1, 2, 3], [4, 5, 6])
        ax.bar_label(container)  # should not crash

    def test_margins_stub(self):
        fig, ax = plt.subplots()
        ax.margins(0.1)  # should not crash
        ax.margins(x=0.1, y=0.2)  # should not crash

    def test_margins_module_level(self):
        plt.figure()
        plt.margins()  # should not crash


class TestImreadImsave:
    """Test imread and imsave."""

    def setup_method(self):
        plt.close()

    def test_imsave_and_imread(self):
        """Test round-trip save and load if Pillow is available."""
        try:
            from PIL import Image
        except ImportError:
            pytest.skip("Pillow not installed")

        arr = np.random.rand(10, 10, 3)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fname = f.name

        try:
            plt.imsave(fname, arr)
            assert os.path.getsize(fname) > 0

            loaded = plt.imread(fname)
            assert loaded.shape[0] == 10
            assert loaded.shape[1] == 10
        finally:
            os.unlink(fname)

    def test_imread_missing_pillow(self):
        """imread should raise ImportError with a helpful message if Pillow is missing."""
        # We can't easily test this without uninstalling Pillow, so skip if installed
        try:
            from PIL import Image
            pytest.skip("Pillow is installed, can't test missing import")
        except ImportError:
            with pytest.raises(ImportError, match="Pillow"):
                plt.imread("nonexistent.png")


class TestCombinedNewFeatures:
    """Integration test using multiple new features together."""

    def setup_method(self):
        plt.close()

    def test_combined_arrow_axline_title(self):
        fig, ax = plt.subplots()
        ax.plot([0, 5], [0, 5])
        ax.arrow(1, 1, 2, 2, color='red', head_width=0.3)
        ax.axline((0, 0), slope=0.5, color='green', linestyle='--')
        ax.set_title("Combined Features", loc="left")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)
