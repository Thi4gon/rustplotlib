"""Tests for Phase 8 — advanced and specialized features."""

import os
import tempfile
import pytest

from rustplotlib import pyplot as plt
from rustplotlib._rustplotlib import RustFigure
from rustplotlib.backends.backend_pdf import PdfPages


class TestSubplotMosaic:
    """Tests for subplot_mosaic()."""

    def test_mosaic_list_layout(self):
        fig, axes = plt.subplot_mosaic([['A', 'B'], ['C', 'C']])
        assert 'A' in axes
        assert 'B' in axes
        assert 'C' in axes
        assert len(axes) == 3

    def test_mosaic_string_layout(self):
        fig, axes = plt.subplot_mosaic("AB\nCC")
        assert 'A' in axes
        assert 'B' in axes
        assert 'C' in axes
        assert len(axes) == 3

    def test_mosaic_with_figsize(self):
        fig, axes = plt.subplot_mosaic([['X', 'Y']], figsize=(10, 5))
        assert 'X' in axes
        assert 'Y' in axes

    def test_mosaic_single_cell(self):
        fig, axes = plt.subplot_mosaic([['A']])
        assert 'A' in axes
        assert len(axes) == 1

    def test_mosaic_dot_placeholder(self):
        fig, axes = plt.subplot_mosaic([['A', '.'], ['B', 'C']])
        assert '.' not in axes
        assert len(axes) == 3

    def test_mosaic_axes_can_plot(self):
        fig, axes = plt.subplot_mosaic([['A', 'B']])
        axes['A'].plot([1, 2, 3], [4, 5, 6])
        axes['B'].bar([1, 2], [3, 4])
        # Should not raise

    def test_mosaic_fig_savefig(self):
        fig, axes = plt.subplot_mosaic([['A', 'B']])
        axes['A'].plot([1, 2], [3, 4])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestViolinPlot:
    """Tests for violinplot()."""

    def test_violinplot_basic(self):
        fig, ax = plt.subplots()
        data = [[1, 2, 3, 4, 5], [2, 3, 4, 5, 6, 7]]
        ax.violinplot(data)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_violinplot_with_options(self):
        fig, ax = plt.subplots()
        data = [[1, 2, 3, 4, 5]]
        ax.violinplot(data, positions=[2.0], widths=0.8, showmeans=True,
                      showmedians=True, color='blue', alpha=0.5)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_violinplot_single_dataset(self):
        fig, ax = plt.subplots()
        ax.violinplot([1, 2, 3, 4, 5])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_violinplot_module_level(self):
        plt.close()
        plt.violinplot([[1, 2, 3], [4, 5, 6]])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestTable:
    """Tests for table()."""

    def test_table_basic(self):
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [10, 20, 30])
        ax.table(cellText=[['10', '20', '30']], colLabels=['A', 'B', 'C'])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_table_with_row_labels(self):
        fig, ax = plt.subplots()
        ax.table(cellText=[['1', '2'], ['3', '4']],
                 colLabels=['X', 'Y'],
                 rowLabels=['R1', 'R2'],
                 loc='bottom')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_table_module_level(self):
        plt.close()
        plt.bar([1, 2], [3, 4])
        plt.table(cellText=[['3', '4']], colLabels=['A', 'B'])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestHlinesVlines:
    """Tests for hlines() and vlines()."""

    def test_hlines_basic(self):
        fig, ax = plt.subplots()
        ax.hlines([1, 2, 3], 0, 10, colors='red')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_vlines_basic(self):
        fig, ax = plt.subplots()
        ax.vlines([1, 2, 3], 0, 10, colors='blue')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_hlines_single_value(self):
        fig, ax = plt.subplots()
        ax.hlines(5, 0, 10)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_vlines_single_value(self):
        fig, ax = plt.subplots()
        ax.vlines(5, 0, 10)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_hlines_with_linestyle(self):
        fig, ax = plt.subplots()
        ax.hlines([1, 2], 0, 5, linestyles='--', linewidth=2.0, alpha=0.5)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_hlines_module_level(self):
        plt.close()
        plt.hlines([1, 2, 3], 0, 10)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_vlines_module_level(self):
        plt.close()
        plt.vlines([1, 2, 3], 0, 10)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestSecondaryAxes:
    """Tests for secondary_xaxis() / secondary_yaxis()."""

    def test_secondary_xaxis_returns_self(self):
        fig, ax = plt.subplots()
        result = ax.secondary_xaxis('top')
        assert result is ax

    def test_secondary_yaxis_returns_self(self):
        fig, ax = plt.subplots()
        result = ax.secondary_yaxis('right')
        assert result is ax


class TestPdfPages:
    """Tests for PdfPages multipage output."""

    def test_pdfpages_single_page(self):
        with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
            fname = f.name
        try:
            with PdfPages(fname) as pdf:
                fig, ax = plt.subplots()
                ax.plot([1, 2, 3], [4, 5, 6])
                pdf.savefig(fig)
            # Single page saves as PNG
            base, _ = os.path.splitext(fname)
            png_file = base + '.png'
            assert os.path.exists(png_file)
            assert os.path.getsize(png_file) > 0
            os.unlink(png_file)
        finally:
            if os.path.exists(fname):
                os.unlink(fname)

    def test_pdfpages_multi_page(self):
        with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
            fname = f.name
        try:
            with PdfPages(fname) as pdf:
                fig1, ax1 = plt.subplots()
                ax1.plot([1, 2], [3, 4])
                pdf.savefig(fig1)

                fig2, ax2 = plt.subplots()
                ax2.bar([1, 2], [5, 6])
                pdf.savefig(fig2)

            base, _ = os.path.splitext(fname)
            page1 = f"{base}_page1.png"
            page2 = f"{base}_page2.png"
            assert os.path.exists(page1)
            assert os.path.exists(page2)
            assert os.path.getsize(page1) > 0
            assert os.path.getsize(page2) > 0
            os.unlink(page1)
            os.unlink(page2)
        finally:
            if os.path.exists(fname):
                os.unlink(fname)

    def test_pdfpages_empty(self):
        with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
            fname = f.name
        try:
            with PdfPages(fname) as pdf:
                pass  # no pages
            # Should not create any files (beyond the temp file)
            base, _ = os.path.splitext(fname)
            assert not os.path.exists(base + '.png')
        finally:
            if os.path.exists(fname):
                os.unlink(fname)

    def test_pdfpages_import_from_backends(self):
        from rustplotlib.backends import PdfPages as PP
        assert PP is PdfPages


class TestFillBetweenX:
    """Tests for fill_betweenx()."""

    def test_fill_betweenx_basic(self):
        fig, ax = plt.subplots()
        y = [0, 1, 2, 3, 4]
        x1 = [0, 1, 2, 1, 0]
        ax.fill_betweenx(y, x1)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_fill_betweenx_with_x2(self):
        fig, ax = plt.subplots()
        y = [0, 1, 2, 3, 4]
        x1 = [0, 1, 2, 1, 0]
        x2 = [1, 2, 3, 2, 1]
        ax.fill_betweenx(y, x1, x2, alpha=0.5, color='green')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_fill_betweenx_module_level(self):
        plt.close()
        plt.fill_betweenx([0, 1, 2], [0, 1, 0], [1, 2, 1])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestCombinedFeatures:
    """Test that Phase 8 features work together."""

    def test_all_new_features_in_subplots(self):
        """Smoke test combining all Phase 8 features."""
        fig, axes = plt.subplots(2, 2, figsize=(10, 8))

        # Subplot 0: violinplot
        axes[0][0].violinplot([[1, 2, 3, 4, 5], [2, 3, 4, 5, 6]])
        axes[0][0].set_title("Violin")

        # Subplot 1: hlines and vlines
        axes[0][1].hlines([1, 2, 3], 0, 5, colors='red')
        axes[0][1].vlines([1, 2, 3], 0, 5, colors='blue')
        axes[0][1].set_title("H/V Lines")

        # Subplot 2: fill_betweenx
        axes[1][0].fill_betweenx([0, 1, 2, 3], [0, 1, 2, 1], [1, 2, 3, 2])
        axes[1][0].set_title("Fill Between X")

        # Subplot 3: table
        axes[1][1].bar([1, 2, 3], [10, 20, 15])
        axes[1][1].table(cellText=[['10', '20', '15']], colLabels=['A', 'B', 'C'])
        axes[1][1].set_title("Table")

        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_mosaic_with_new_features(self):
        """Test subplot_mosaic with Phase 8 features."""
        fig, axes = plt.subplot_mosaic([['violin', 'lines'], ['fill', 'fill']])

        axes['violin'].violinplot([[1, 2, 3, 4, 5]])
        axes['lines'].hlines([1, 2], 0, 5)
        axes['lines'].vlines([1, 2], 0, 5)
        axes['fill'].fill_betweenx([0, 1, 2], [0, 1, 0], [1, 2, 1])

        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)
