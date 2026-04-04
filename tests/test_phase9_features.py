"""Tests for hatch patterns, zorder, fill polygon, pcolormesh, matshow, sankey."""

import os
import numpy as np
import rustplotlib.pyplot as plt


class TestHatchPatterns:
    """Tests for bar hatch patterns."""

    def test_bar_hatch_slash(self):
        """Test bars with / hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='/')
        fig.savefig('/tmp/test_hatch_slash.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_slash.png')
        assert os.path.getsize('/tmp/test_hatch_slash.png') > 0
        os.remove('/tmp/test_hatch_slash.png')

    def test_bar_hatch_backslash(self):
        """Test bars with backslash hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='\\')
        fig.savefig('/tmp/test_hatch_backslash.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_backslash.png')
        assert os.path.getsize('/tmp/test_hatch_backslash.png') > 0
        os.remove('/tmp/test_hatch_backslash.png')

    def test_bar_hatch_vertical(self):
        """Test bars with | hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='|')
        fig.savefig('/tmp/test_hatch_vertical.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_vertical.png')
        assert os.path.getsize('/tmp/test_hatch_vertical.png') > 0
        os.remove('/tmp/test_hatch_vertical.png')

    def test_bar_hatch_horizontal(self):
        """Test bars with - hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='-')
        fig.savefig('/tmp/test_hatch_horizontal.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_horizontal.png')
        assert os.path.getsize('/tmp/test_hatch_horizontal.png') > 0
        os.remove('/tmp/test_hatch_horizontal.png')

    def test_bar_hatch_grid(self):
        """Test bars with + (grid) hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='+')
        fig.savefig('/tmp/test_hatch_grid.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_grid.png')
        assert os.path.getsize('/tmp/test_hatch_grid.png') > 0
        os.remove('/tmp/test_hatch_grid.png')

    def test_bar_hatch_cross(self):
        """Test bars with x (diagonal cross) hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='x')
        fig.savefig('/tmp/test_hatch_cross.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_cross.png')
        assert os.path.getsize('/tmp/test_hatch_cross.png') > 0
        os.remove('/tmp/test_hatch_cross.png')

    def test_bar_hatch_dots(self):
        """Test bars with . hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='.')
        fig.savefig('/tmp/test_hatch_dots.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_dots.png')
        assert os.path.getsize('/tmp/test_hatch_dots.png') > 0
        os.remove('/tmp/test_hatch_dots.png')

    def test_bar_hatch_circles(self):
        """Test bars with o hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='o')
        fig.savefig('/tmp/test_hatch_circles.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_circles.png')
        assert os.path.getsize('/tmp/test_hatch_circles.png') > 0
        os.remove('/tmp/test_hatch_circles.png')

    def test_bar_hatch_stars(self):
        """Test bars with * hatch pattern."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], hatch='*')
        fig.savefig('/tmp/test_hatch_stars.png')
        plt.close()
        assert os.path.exists('/tmp/test_hatch_stars.png')
        assert os.path.getsize('/tmp/test_hatch_stars.png') > 0
        os.remove('/tmp/test_hatch_stars.png')

    def test_bar_no_hatch(self):
        """Test bars without hatch (should work normally)."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6])
        fig.savefig('/tmp/test_bar_no_hatch.png')
        plt.close()
        assert os.path.exists('/tmp/test_bar_no_hatch.png')
        assert os.path.getsize('/tmp/test_bar_no_hatch.png') > 0
        os.remove('/tmp/test_bar_no_hatch.png')

    def test_bar_all_hatches_subplot(self):
        """Test all hatch patterns in a single figure."""
        fig, axes = plt.subplots(2, 3)
        patterns = ['/', '\\', '|', '-', '+', 'x']
        flat_axes = [ax for row in axes for ax in row]
        for ax, pat in zip(flat_axes, patterns):
            ax.bar([1, 2], [3, 4], hatch=pat)
            ax.set_title(f'hatch={pat}')
        fig.savefig('/tmp/test_all_hatches.png')
        plt.close()
        assert os.path.exists('/tmp/test_all_hatches.png')
        assert os.path.getsize('/tmp/test_all_hatches.png') > 0
        os.remove('/tmp/test_all_hatches.png')


class TestZorder:
    """Tests for zorder drawing order control."""

    def test_bar_zorder(self):
        """Test setting zorder on bars."""
        fig, ax = plt.subplots()
        ax.bar([1, 2, 3], [4, 5, 6], zorder=5)
        ax.plot([1, 2, 3], [3, 4, 5], zorder=10)
        fig.savefig('/tmp/test_zorder_bar.png')
        plt.close()
        assert os.path.exists('/tmp/test_zorder_bar.png')
        assert os.path.getsize('/tmp/test_zorder_bar.png') > 0
        os.remove('/tmp/test_zorder_bar.png')

    def test_plot_zorder(self):
        """Test setting zorder on line plots."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3], zorder=1, label='z=1')
        ax.plot([1, 2, 3], [3, 2, 1], zorder=10, label='z=10')
        fig.savefig('/tmp/test_zorder_plot.png')
        plt.close()
        assert os.path.exists('/tmp/test_zorder_plot.png')
        assert os.path.getsize('/tmp/test_zorder_plot.png') > 0
        os.remove('/tmp/test_zorder_plot.png')

    def test_scatter_zorder(self):
        """Test setting zorder on scatter plots."""
        fig, ax = plt.subplots()
        ax.scatter([1, 2, 3], [1, 2, 3], zorder=5)
        ax.plot([0, 4], [0, 4], zorder=1)
        fig.savefig('/tmp/test_zorder_scatter.png')
        plt.close()
        assert os.path.exists('/tmp/test_zorder_scatter.png')
        assert os.path.getsize('/tmp/test_zorder_scatter.png') > 0
        os.remove('/tmp/test_zorder_scatter.png')

    def test_default_zorder(self):
        """Test default zorder works without explicit setting."""
        fig, ax = plt.subplots()
        ax.bar([1, 2], [3, 4])
        ax.plot([1, 2], [2, 3])
        ax.scatter([1, 2], [1, 2])
        fig.savefig('/tmp/test_default_zorder.png')
        plt.close()
        assert os.path.exists('/tmp/test_default_zorder.png')
        assert os.path.getsize('/tmp/test_default_zorder.png') > 0
        os.remove('/tmp/test_default_zorder.png')


class TestFillPolygon:
    """Tests for ax.fill() - filled polygon."""

    def test_fill_triangle(self):
        """Test filling a triangle."""
        fig, ax = plt.subplots()
        ax.fill([0, 1, 0.5], [0, 0, 1], color='blue', alpha=0.5)
        fig.savefig('/tmp/test_fill_triangle.png')
        plt.close()
        assert os.path.exists('/tmp/test_fill_triangle.png')
        assert os.path.getsize('/tmp/test_fill_triangle.png') > 0
        os.remove('/tmp/test_fill_triangle.png')

    def test_fill_pentagon(self):
        """Test filling a pentagon."""
        fig, ax = plt.subplots()
        import math
        n = 5
        angles = [2 * math.pi * i / n - math.pi / 2 for i in range(n)]
        x = [math.cos(a) for a in angles]
        y = [math.sin(a) for a in angles]
        ax.fill(x, y, color='green', alpha=0.6)
        fig.savefig('/tmp/test_fill_pentagon.png')
        plt.close()
        assert os.path.exists('/tmp/test_fill_pentagon.png')
        assert os.path.getsize('/tmp/test_fill_pentagon.png') > 0
        os.remove('/tmp/test_fill_pentagon.png')

    def test_fill_with_color_arg(self):
        """Test fill with color as third positional arg."""
        fig, ax = plt.subplots()
        ax.fill([0, 1, 1, 0], [0, 0, 1, 1], 'red')
        fig.savefig('/tmp/test_fill_color_arg.png')
        plt.close()
        assert os.path.exists('/tmp/test_fill_color_arg.png')
        assert os.path.getsize('/tmp/test_fill_color_arg.png') > 0
        os.remove('/tmp/test_fill_color_arg.png')

    def test_fill_with_label(self):
        """Test fill polygon with a label for legend."""
        fig, ax = plt.subplots()
        ax.fill([0, 2, 1], [0, 0, 2], color='orange', alpha=0.5, label='triangle')
        ax.legend()
        fig.savefig('/tmp/test_fill_label.png')
        plt.close()
        assert os.path.exists('/tmp/test_fill_label.png')
        assert os.path.getsize('/tmp/test_fill_label.png') > 0
        os.remove('/tmp/test_fill_label.png')

    def test_fill_module_level(self):
        """Test module-level fill function."""
        plt.figure()
        plt.fill([0, 1, 0.5], [0, 0, 1])
        plt.savefig('/tmp/test_fill_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_fill_module.png')
        assert os.path.getsize('/tmp/test_fill_module.png') > 0
        os.remove('/tmp/test_fill_module.png')


class TestPColorMesh:
    """Tests for pcolormesh pseudocolor plot."""

    def test_pcolormesh_basic(self):
        """Test basic pcolormesh with data only."""
        fig, ax = plt.subplots()
        data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        ax.pcolormesh(data)
        fig.savefig('/tmp/test_pcolormesh_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolormesh_basic.png')
        assert os.path.getsize('/tmp/test_pcolormesh_basic.png') > 0
        os.remove('/tmp/test_pcolormesh_basic.png')

    def test_pcolormesh_with_cmap(self):
        """Test pcolormesh with different colormap."""
        fig, ax = plt.subplots()
        data = np.random.rand(5, 5).tolist()
        ax.pcolormesh(data, cmap='hot')
        fig.savefig('/tmp/test_pcolormesh_cmap.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolormesh_cmap.png')
        assert os.path.getsize('/tmp/test_pcolormesh_cmap.png') > 0
        os.remove('/tmp/test_pcolormesh_cmap.png')

    def test_pcolormesh_with_alpha(self):
        """Test pcolormesh with alpha."""
        fig, ax = plt.subplots()
        data = [[1, 2], [3, 4]]
        ax.pcolormesh(data, alpha=0.5)
        fig.savefig('/tmp/test_pcolormesh_alpha.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolormesh_alpha.png')
        assert os.path.getsize('/tmp/test_pcolormesh_alpha.png') > 0
        os.remove('/tmp/test_pcolormesh_alpha.png')

    def test_pcolormesh_module_level(self):
        """Test module-level pcolormesh function."""
        plt.figure()
        plt.pcolormesh([[1, 2], [3, 4]])
        plt.savefig('/tmp/test_pcolormesh_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolormesh_module.png')
        assert os.path.getsize('/tmp/test_pcolormesh_module.png') > 0
        os.remove('/tmp/test_pcolormesh_module.png')


class TestPColor:
    """Tests for pcolor (pcolormesh with edges)."""

    def test_pcolor_basic(self):
        """Test basic pcolor with cell outlines."""
        fig, ax = plt.subplots()
        data = [[1, 2, 3], [4, 5, 6]]
        ax.pcolor(data)
        fig.savefig('/tmp/test_pcolor_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolor_basic.png')
        assert os.path.getsize('/tmp/test_pcolor_basic.png') > 0
        os.remove('/tmp/test_pcolor_basic.png')

    def test_pcolor_module_level(self):
        """Test module-level pcolor function."""
        plt.figure()
        plt.pcolor([[1, 2], [3, 4]])
        plt.savefig('/tmp/test_pcolor_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_pcolor_module.png')
        assert os.path.getsize('/tmp/test_pcolor_module.png') > 0
        os.remove('/tmp/test_pcolor_module.png')


class TestMatshow:
    """Tests for matshow (matrix display)."""

    def test_matshow_basic(self):
        """Test basic matshow."""
        fig, ax = plt.subplots()
        data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        ax.matshow(data)
        fig.savefig('/tmp/test_matshow_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_matshow_basic.png')
        assert os.path.getsize('/tmp/test_matshow_basic.png') > 0
        os.remove('/tmp/test_matshow_basic.png')

    def test_matshow_with_cmap(self):
        """Test matshow with a colormap."""
        fig, ax = plt.subplots()
        np.random.seed(42)
        data = np.random.rand(4, 4).tolist()
        ax.matshow(data, cmap='coolwarm')
        fig.savefig('/tmp/test_matshow_cmap.png')
        plt.close()
        assert os.path.exists('/tmp/test_matshow_cmap.png')
        assert os.path.getsize('/tmp/test_matshow_cmap.png') > 0
        os.remove('/tmp/test_matshow_cmap.png')

    def test_matshow_module_level(self):
        """Test module-level matshow function."""
        plt.figure()
        plt.matshow([[1, 0], [0, 1]])
        plt.savefig('/tmp/test_matshow_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_matshow_module.png')
        assert os.path.getsize('/tmp/test_matshow_module.png') > 0
        os.remove('/tmp/test_matshow_module.png')


class TestSankey:
    """Tests for basic Sankey diagram."""

    def test_sankey_basic(self):
        """Test basic Sankey with inputs and outputs."""
        fig, ax = plt.subplots()
        ax.sankey(
            flows=[0.5, 0.3, 0.2, -0.4, -0.6],
            labels=['A', 'B', 'C', 'D', 'E']
        )
        fig.savefig('/tmp/test_sankey_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_sankey_basic.png')
        assert os.path.getsize('/tmp/test_sankey_basic.png') > 0
        os.remove('/tmp/test_sankey_basic.png')

    def test_sankey_simple(self):
        """Test simple Sankey with one input, one output."""
        fig, ax = plt.subplots()
        ax.sankey(flows=[1.0, -1.0], labels=['in', 'out'])
        fig.savefig('/tmp/test_sankey_simple.png')
        plt.close()
        assert os.path.exists('/tmp/test_sankey_simple.png')
        assert os.path.getsize('/tmp/test_sankey_simple.png') > 0
        os.remove('/tmp/test_sankey_simple.png')

    def test_sankey_with_alpha(self):
        """Test Sankey with custom alpha."""
        fig, ax = plt.subplots()
        ax.sankey(
            flows=[0.5, 0.5, -0.3, -0.7],
            labels=['A', 'B', 'C', 'D'],
            alpha=0.5
        )
        fig.savefig('/tmp/test_sankey_alpha.png')
        plt.close()
        assert os.path.exists('/tmp/test_sankey_alpha.png')
        assert os.path.getsize('/tmp/test_sankey_alpha.png') > 0
        os.remove('/tmp/test_sankey_alpha.png')


class TestStackplotWithLegend:
    """Verify stackplot works with labels and legend."""

    def test_stackplot_with_labels(self):
        """Test stackplot with labels for legend."""
        fig, ax = plt.subplots()
        x = [1, 2, 3, 4, 5]
        y1 = [1, 2, 3, 2, 1]
        y2 = [2, 1, 2, 3, 2]
        y3 = [1, 1, 1, 1, 1]
        ax.stackplot(x, y1, y2, y3, labels=['A', 'B', 'C'])
        ax.legend()
        fig.savefig('/tmp/test_stackplot_legend.png')
        plt.close()
        assert os.path.exists('/tmp/test_stackplot_legend.png')
        assert os.path.getsize('/tmp/test_stackplot_legend.png') > 0
        os.remove('/tmp/test_stackplot_legend.png')

    def test_stackplot_with_colors(self):
        """Test stackplot with custom colors."""
        fig, ax = plt.subplots()
        x = [1, 2, 3]
        ax.stackplot(x, [1, 2, 3], [3, 2, 1],
                      colors=['red', 'blue'],
                      labels=['Red', 'Blue'])
        ax.legend()
        fig.savefig('/tmp/test_stackplot_colors.png')
        plt.close()
        assert os.path.exists('/tmp/test_stackplot_colors.png')
        assert os.path.getsize('/tmp/test_stackplot_colors.png') > 0
        os.remove('/tmp/test_stackplot_colors.png')


class TestCombinedNewFeatures:
    """Test combining multiple new features in one figure."""

    def test_all_new_features(self):
        """Test all new features in a single multi-subplot figure."""
        fig, axes = plt.subplots(2, 3)

        # Hatch patterns
        axes[0][0].bar([1, 2, 3], [4, 5, 6], hatch='/', color='blue')
        axes[0][0].set_title('Hatch /')

        # zorder
        axes[0][1].bar([1, 2, 3], [4, 5, 6], zorder=1, alpha=0.5)
        axes[0][1].plot([1, 2, 3], [3, 5, 4], zorder=10, color='red')
        axes[0][1].set_title('zorder')

        # fill polygon
        axes[0][2].fill([0, 1, 0.5], [0, 0, 1], color='green', alpha=0.5)
        axes[0][2].set_title('fill')

        # pcolormesh
        data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        axes[1][0].pcolormesh(data, cmap='viridis')
        axes[1][0].set_title('pcolormesh')

        # matshow
        axes[1][1].matshow([[1, 0], [0, 1]], cmap='hot')
        axes[1][1].set_title('matshow')

        # sankey
        axes[1][2].sankey(flows=[0.5, 0.5, -0.3, -0.7], labels=['A', 'B', 'C', 'D'])
        axes[1][2].set_title('sankey')

        fig.savefig('/tmp/test_all_new_features.png')
        plt.close()
        assert os.path.exists('/tmp/test_all_new_features.png')
        assert os.path.getsize('/tmp/test_all_new_features.png') > 0
        os.remove('/tmp/test_all_new_features.png')
