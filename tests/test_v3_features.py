"""Tests for v3.0.0 features: RGB imshow, image extent, minor ticks, FancyArrowPatch, xlabel color."""

import pytest
import numpy as np
import os
import tempfile

from rustplotlib import pyplot as plt
from rustplotlib.patches import FancyArrowPatch, ConnectionPatch


class TestImshowRGB:
    """Test RGB and RGBA image support in imshow."""

    def setup_method(self):
        plt.close()

    def test_imshow_rgb_array(self):
        """3D array with shape (H, W, 3) should render as RGB."""
        fig, ax = plt.subplots()
        # Create a simple 4x4 RGB image
        rgb = np.zeros((4, 4, 3))
        rgb[0, :, 0] = 1.0  # top row red
        rgb[1, :, 1] = 1.0  # second row green
        rgb[2, :, 2] = 1.0  # third row blue
        rgb[3, :, :] = 1.0  # bottom row white
        ax.imshow(rgb)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_rgba_array(self):
        """3D array with shape (H, W, 4) should render as RGBA."""
        fig, ax = plt.subplots()
        rgba = np.zeros((3, 3, 4))
        rgba[:, :, 0] = 1.0  # red channel
        rgba[:, :, 3] = 0.5  # half alpha
        ax.imshow(rgba)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_2d_still_works(self):
        """2D array should still work with colormap."""
        fig, ax = plt.subplots()
        data = np.random.rand(5, 5)
        ax.imshow(data, cmap='hot')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_rgb_with_origin_lower(self):
        """RGB imshow with origin='lower' should flip the image."""
        fig, ax = plt.subplots()
        rgb = np.zeros((4, 4, 3))
        rgb[0, :, 0] = 1.0  # top row red
        ax.imshow(rgb, origin='lower')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_rgb_uint8_range(self):
        """RGB values in 0-255 range (converted to float)."""
        fig, ax = plt.subplots()
        rgb = np.random.randint(0, 256, (10, 10, 3)).astype(float) / 255.0
        ax.imshow(rgb)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestImageExtent:
    """Test image extent parameter."""

    def setup_method(self):
        plt.close()

    def test_imshow_with_extent(self):
        """Scalar imshow with extent parameter."""
        fig, ax = plt.subplots()
        data = np.random.rand(10, 10)
        ax.imshow(data, extent=[-5, 5, -5, 5])
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_rgb_with_extent(self):
        """RGB imshow with extent parameter."""
        fig, ax = plt.subplots()
        rgb = np.random.rand(8, 8, 3)
        ax.imshow(rgb, extent=[0, 10, 0, 10])
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_imshow_extent_with_interpolation(self):
        """Scalar imshow with extent + bilinear interpolation."""
        fig, ax = plt.subplots()
        data = np.random.rand(5, 5)
        ax.imshow(data, extent=[-1, 1, -1, 1], interpolation='bilinear')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestMinorTicks:
    """Test minor tick rendering."""

    def setup_method(self):
        plt.close()

    def test_set_xticks_minor(self):
        """Minor x-ticks should be accepted and rendered."""
        fig, ax = plt.subplots()
        ax.plot([0, 10], [0, 10])
        ax.set_xticks([0, 2, 4, 6, 8, 10])
        ax.set_xticks([1, 3, 5, 7, 9], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_set_yticks_minor(self):
        """Minor y-ticks should be accepted and rendered."""
        fig, ax = plt.subplots()
        ax.plot([0, 10], [0, 10])
        ax.set_yticks([0, 5, 10])
        ax.set_yticks([1, 2, 3, 4, 6, 7, 8, 9], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_minor_ticks_both_axes(self):
        """Minor ticks on both x and y axes."""
        fig, ax = plt.subplots()
        ax.plot([0, 5], [0, 25])
        ax.set_xticks([0, 1, 2, 3, 4, 5])
        ax.set_xticks([0.5, 1.5, 2.5, 3.5, 4.5], minor=True)
        ax.set_yticks([0, 5, 10, 15, 20, 25])
        ax.set_yticks([2.5, 7.5, 12.5, 17.5, 22.5], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_minor_ticks_without_major(self):
        """Minor ticks can be set even without custom major ticks."""
        fig, ax = plt.subplots()
        ax.plot([0, 10], [0, 10])
        ax.set_xticks([1, 3, 5, 7, 9], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestFancyArrowPatch:
    """Test FancyArrowPatch and ConnectionPatch stubs."""

    def test_fancy_arrow_patch_creation(self):
        """FancyArrowPatch can be created with posA and posB."""
        arrow = FancyArrowPatch(posA=(0, 0), posB=(1, 1), arrowstyle='->')
        assert arrow.posA == (0, 0)
        assert arrow.posB == (1, 1)
        assert arrow.arrowstyle == '->'
        assert arrow.connectionstyle == 'arc3'
        assert arrow.mutation_scale == 1

    def test_fancy_arrow_patch_defaults(self):
        """FancyArrowPatch has sensible defaults."""
        arrow = FancyArrowPatch()
        assert arrow.posA is None
        assert arrow.posB is None
        assert arrow.arrowstyle == '->'
        assert arrow.shrinkA == 2
        assert arrow.shrinkB == 2

    def test_fancy_arrow_patch_with_kwargs(self):
        """FancyArrowPatch accepts Patch kwargs."""
        arrow = FancyArrowPatch(posA=(0, 0), posB=(1, 1),
                                facecolor='red', edgecolor='blue',
                                linewidth=2, alpha=0.5)
        assert arrow.facecolor == 'red'
        assert arrow.edgecolor == 'blue'
        assert arrow.linewidth == 2
        assert arrow.alpha == 0.5

    def test_connection_patch_creation(self):
        """ConnectionPatch extends FancyArrowPatch with coordinate info."""
        cp = ConnectionPatch(xyA=(0, 0), xyB=(1, 1),
                             coordsA='data', coordsB='axes fraction')
        assert cp.posA == (0, 0)
        assert cp.posB == (1, 1)
        assert cp.coordsA == 'data'
        assert cp.coordsB == 'axes fraction'
        assert cp.axesA is None
        assert cp.axesB is None

    def test_connection_patch_with_axes(self):
        """ConnectionPatch can reference specific axes."""
        fig, (ax1, ax2) = plt.subplots(1, 2)
        cp = ConnectionPatch(xyA=(1, 1), xyB=(0, 0),
                             coordsA='data', coordsB='data',
                             axesA=ax1, axesB=ax2,
                             arrowstyle='-|>')
        assert cp.axesA is ax1
        assert cp.axesB is ax2
        assert cp.arrowstyle == '-|>'
        plt.close()


class TestXLabelColor:
    """Test xlabel/ylabel color parameter."""

    def setup_method(self):
        plt.close()

    def test_xlabel_with_color(self):
        """set_xlabel with color kwarg."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_xlabel("X axis", color='red')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_ylabel_with_color(self):
        """set_ylabel with color kwarg."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_ylabel("Y axis", color='blue')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_xlabel_with_fontsize_and_color(self):
        """set_xlabel with both fontsize and color."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_xlabel("X axis", fontsize=16, color='green')
        ax.set_ylabel("Y axis", fontsize=14, color='#FF5500')
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_xlabel_with_fontdict_color(self):
        """set_xlabel with color in fontdict."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_xlabel("X axis", fontdict={'color': 'purple'})
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_xlabel_without_color(self):
        """set_xlabel without color still works (uses text_color)."""
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])
        ax.set_xlabel("X axis")
        ax.set_ylabel("Y axis")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestCombinedV3Features:
    """Integration test combining multiple v3 features."""

    def setup_method(self):
        plt.close()

    def test_rgb_image_with_minor_ticks_and_labels(self):
        """Combined: RGB image + minor ticks + colored labels."""
        fig, ax = plt.subplots(figsize=(8, 6))
        rgb = np.random.rand(20, 20, 3)
        ax.imshow(rgb, extent=[-10, 10, -10, 10])
        ax.set_xticks([-10, -5, 0, 5, 10])
        ax.set_xticks([-7.5, -2.5, 2.5, 7.5], minor=True)
        ax.set_yticks([-10, -5, 0, 5, 10])
        ax.set_yticks([-7.5, -2.5, 2.5, 7.5], minor=True)
        ax.set_xlabel("X Position", color='red', fontsize=14)
        ax.set_ylabel("Y Position", color='blue', fontsize=14)
        ax.set_title("RGB Image with Minor Ticks")
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_svg_output_with_minor_ticks(self):
        """Minor ticks should also render in SVG."""
        fig, ax = plt.subplots()
        ax.plot([0, 10], [0, 10])
        ax.set_xticks([0, 5, 10])
        ax.set_xticks([1, 2, 3, 4, 6, 7, 8, 9], minor=True)
        with tempfile.NamedTemporaryFile(suffix=".svg", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)
