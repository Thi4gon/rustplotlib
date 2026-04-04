"""Tests for trisurf, radar, broken_barh, eventplot, stackplot features."""

import os
import numpy as np
import rustplotlib.pyplot as plt


class TestTriSurf:
    """Tests for 3D triangulated surface plot."""

    def test_trisurf_basic(self):
        """Test basic trisurf plot from point cloud."""
        fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
        np.random.seed(42)
        n = 50
        x = np.random.rand(n)
        y = np.random.rand(n)
        z = np.sin(x * 3) * np.cos(y * 3)
        ax.plot_trisurf(x, y, z, cmap='viridis', alpha=0.8)
        ax.set_title('TriSurf Basic')
        fig.savefig('/tmp/test_trisurf_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_trisurf_basic.png')
        assert os.path.getsize('/tmp/test_trisurf_basic.png') > 0
        os.remove('/tmp/test_trisurf_basic.png')

    def test_trisurf_with_triangles(self):
        """Test trisurf with explicit triangle indices."""
        fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
        x = [0, 1, 0.5, 0, 1]
        y = [0, 0, 1, 1, 1]
        z = [0, 0, 1, 0, 0]
        triangles = [[0, 1, 2], [2, 3, 4]]
        ax.plot_trisurf(x, y, z, triangles=triangles, cmap='coolwarm')
        ax.set_title('TriSurf Explicit Triangles')
        fig.savefig('/tmp/test_trisurf_explicit.png')
        plt.close()
        assert os.path.exists('/tmp/test_trisurf_explicit.png')
        assert os.path.getsize('/tmp/test_trisurf_explicit.png') > 0
        os.remove('/tmp/test_trisurf_explicit.png')

    def test_trisurf_colormaps(self):
        """Test trisurf with different colormaps."""
        for cmap in ['viridis', 'coolwarm', 'plasma', 'hot']:
            fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
            np.random.seed(0)
            x = np.random.rand(30)
            y = np.random.rand(30)
            z = x + y
            ax.plot_trisurf(x, y, z, cmap=cmap, alpha=0.7)
            fig.savefig(f'/tmp/test_trisurf_{cmap}.png')
            plt.close()
            assert os.path.exists(f'/tmp/test_trisurf_{cmap}.png')
            os.remove(f'/tmp/test_trisurf_{cmap}.png')


class TestRadar:
    """Tests for radar / spider chart."""

    def test_radar_basic(self):
        """Test basic radar chart with one series."""
        fig, ax = plt.subplots()
        categories = ['Speed', 'Power', 'Defense', 'Agility', 'Stamina']
        values = [[8, 6, 7, 9, 5]]
        ax.radar(categories, values)
        ax.set_title('Radar Basic')
        fig.savefig('/tmp/test_radar_basic.png')
        plt.close()
        assert os.path.exists('/tmp/test_radar_basic.png')
        assert os.path.getsize('/tmp/test_radar_basic.png') > 0
        os.remove('/tmp/test_radar_basic.png')

    def test_radar_multiple_series(self):
        """Test radar chart with multiple series."""
        fig, ax = plt.subplots()
        categories = ['A', 'B', 'C', 'D', 'E', 'F']
        values = [[4, 3, 5, 2, 4, 3], [3, 5, 2, 4, 3, 5]]
        ax.radar(categories, values, colors=['red', 'blue'],
                 labels=['Series 1', 'Series 2'], fill=True, alpha=0.6)
        fig.savefig('/tmp/test_radar_multi.png')
        plt.close()
        assert os.path.exists('/tmp/test_radar_multi.png')
        assert os.path.getsize('/tmp/test_radar_multi.png') > 0
        os.remove('/tmp/test_radar_multi.png')

    def test_radar_no_fill(self):
        """Test radar chart without fill."""
        fig, ax = plt.subplots()
        categories = ['X', 'Y', 'Z']
        values = [[5, 8, 3]]
        ax.radar(categories, values, fill=False)
        fig.savefig('/tmp/test_radar_nofill.png')
        plt.close()
        assert os.path.exists('/tmp/test_radar_nofill.png')
        os.remove('/tmp/test_radar_nofill.png')


class TestBrokenBarH:
    """Tests for broken horizontal bar chart."""

    def test_broken_barh_basic(self):
        """Test basic broken_barh plot."""
        fig, ax = plt.subplots()
        ax.broken_barh([(10, 5), (20, 10), (35, 5)], (10, 9),
                        facecolors='blue')
        ax.broken_barh([(5, 10), (25, 15)], (20, 9),
                        facecolors='red')
        ax.set_xlabel('Time')
        ax.set_ylabel('Category')
        ax.set_title('Broken BarH')
        fig.savefig('/tmp/test_broken_barh.png')
        plt.close()
        assert os.path.exists('/tmp/test_broken_barh.png')
        assert os.path.getsize('/tmp/test_broken_barh.png') > 0
        os.remove('/tmp/test_broken_barh.png')

    def test_broken_barh_module_level(self):
        """Test module-level broken_barh function."""
        plt.figure()
        plt.broken_barh([(1, 2), (4, 3)], (0, 1))
        plt.savefig('/tmp/test_broken_barh_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_broken_barh_module.png')
        os.remove('/tmp/test_broken_barh_module.png')


class TestEventPlot:
    """Tests for event / raster plot."""

    def test_eventplot_basic(self):
        """Test basic event plot."""
        fig, ax = plt.subplots()
        np.random.seed(42)
        events = [np.random.rand(20) * 10, np.random.rand(15) * 10,
                  np.random.rand(25) * 10]
        ax.eventplot(events)
        ax.set_title('Event Plot')
        fig.savefig('/tmp/test_eventplot.png')
        plt.close()
        assert os.path.exists('/tmp/test_eventplot.png')
        assert os.path.getsize('/tmp/test_eventplot.png') > 0
        os.remove('/tmp/test_eventplot.png')

    def test_eventplot_single_row(self):
        """Test event plot with single row of events."""
        fig, ax = plt.subplots()
        events = [1.0, 2.5, 3.0, 5.5, 7.0]
        ax.eventplot(events)
        fig.savefig('/tmp/test_eventplot_single.png')
        plt.close()
        assert os.path.exists('/tmp/test_eventplot_single.png')
        os.remove('/tmp/test_eventplot_single.png')

    def test_eventplot_with_options(self):
        """Test event plot with orientation and style options."""
        fig, ax = plt.subplots()
        np.random.seed(0)
        events = [np.random.rand(10) * 5, np.random.rand(10) * 5]
        ax.eventplot(events, orientation='horizontal', linewidths=2.0,
                     colors=['red', 'blue'], linelength=0.5)
        fig.savefig('/tmp/test_eventplot_opts.png')
        plt.close()
        assert os.path.exists('/tmp/test_eventplot_opts.png')
        os.remove('/tmp/test_eventplot_opts.png')

    def test_eventplot_module_level(self):
        """Test module-level eventplot function."""
        plt.figure()
        plt.eventplot([[1, 2, 3], [2, 3, 5]])
        plt.savefig('/tmp/test_eventplot_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_eventplot_module.png')
        os.remove('/tmp/test_eventplot_module.png')


class TestStackPlot:
    """Tests for stacked area chart."""

    def test_stackplot_basic(self):
        """Test basic stackplot."""
        fig, ax = plt.subplots()
        x = [1, 2, 3, 4, 5]
        y1 = [1, 1, 2, 3, 5]
        y2 = [2, 3, 4, 2, 1]
        y3 = [1, 2, 1, 3, 2]
        ax.stackplot(x, y1, y2, y3, labels=['A', 'B', 'C'], alpha=0.5)
        ax.legend()
        ax.set_title('Stack Plot')
        fig.savefig('/tmp/test_stackplot.png')
        plt.close()
        assert os.path.exists('/tmp/test_stackplot.png')
        assert os.path.getsize('/tmp/test_stackplot.png') > 0
        os.remove('/tmp/test_stackplot.png')

    def test_stackplot_with_colors(self):
        """Test stackplot with custom colors."""
        fig, ax = plt.subplots()
        x = np.arange(10)
        y1 = np.random.rand(10)
        y2 = np.random.rand(10)
        ax.stackplot(x, y1, y2, colors=['red', 'blue'], alpha=0.7)
        fig.savefig('/tmp/test_stackplot_colors.png')
        plt.close()
        assert os.path.exists('/tmp/test_stackplot_colors.png')
        os.remove('/tmp/test_stackplot_colors.png')

    def test_stackplot_module_level(self):
        """Test module-level stackplot function."""
        plt.figure()
        x = [1, 2, 3, 4]
        plt.stackplot(x, [1, 2, 3, 4], [4, 3, 2, 1])
        plt.savefig('/tmp/test_stackplot_module.png')
        plt.close()
        assert os.path.exists('/tmp/test_stackplot_module.png')
        os.remove('/tmp/test_stackplot_module.png')


class TestCombinedNewFeatures:
    """Test combining new features in subplots."""

    def test_all_new_2d_features(self):
        """Test all new 2D features in a subplot layout."""
        fig, axes = plt.subplots(2, 2, figsize=(12, 10))

        # Radar
        axes[0][0].radar(
            ['A', 'B', 'C', 'D', 'E'],
            [[5, 4, 3, 2, 5], [3, 5, 4, 3, 2]],
            fill=True, alpha=0.7
        )
        axes[0][0].set_title('Radar')

        # Broken BarH
        axes[0][1].broken_barh([(10, 5), (20, 10)], (5, 4))
        axes[0][1].broken_barh([(5, 8), (18, 12)], (10, 4))
        axes[0][1].set_title('Broken BarH')

        # Event Plot
        np.random.seed(42)
        axes[1][0].eventplot([np.random.rand(15) * 10,
                              np.random.rand(20) * 10])
        axes[1][0].set_title('Event Plot')

        # Stack Plot
        x = np.arange(10)
        axes[1][1].stackplot(x, np.random.rand(10), np.random.rand(10),
                             np.random.rand(10), alpha=0.5)
        axes[1][1].set_title('Stack Plot')

        fig.savefig('/tmp/test_all_new_2d.png')
        plt.close()
        assert os.path.exists('/tmp/test_all_new_2d.png')
        assert os.path.getsize('/tmp/test_all_new_2d.png') > 1000
        os.remove('/tmp/test_all_new_2d.png')
