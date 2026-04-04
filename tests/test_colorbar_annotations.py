"""Tests for colorbar and annotation improvements."""
import rustplotlib.pyplot as plt
import numpy as np
import tempfile, os


def test_colorbar_basic():
    """Basic colorbar works."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar()
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_colorbar_with_label():
    """colorbar accepts label kwarg."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar(label='Temperature (°C)')
    plt.close()


def test_colorbar_horizontal():
    """colorbar accepts orientation kwarg."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar(orientation='horizontal')
    plt.close()


def test_colorbar_shrink_pad():
    """colorbar accepts shrink and pad kwargs."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar(shrink=0.8, pad=0.05)
    plt.close()


def test_colorbar_module_level():
    """plt.colorbar() works."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    plt.colorbar()
    plt.close()


def test_annotate_basic():
    """Basic annotate works."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('peak', xy=(3, 9), xytext=(2, 7))
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_annotate_with_arrowprops():
    """annotate accepts arrowprops dict."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('peak', xy=(3, 9), xytext=(2, 6),
                arrowprops=dict(arrowstyle='->', color='red'))
    plt.close()


def test_annotate_with_fontsize():
    """annotate accepts fontsize."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('big text', xy=(2, 4), fontsize=16)
    plt.close()


def test_annotate_with_bbox():
    """annotate accepts bbox for text background."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('boxed', xy=(2, 4), xytext=(1, 6),
                bbox=dict(boxstyle='round', facecolor='#ffe4b5', alpha=0.5))
    plt.close()


def test_annotate_module_level():
    """plt.annotate() module-level works."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    plt.annotate('test', xy=(2, 4), xytext=(1, 6))
    plt.close()


def test_colorbar_all_kwargs():
    """colorbar with all kwargs renders without error."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar(label='Value', orientation='vertical', shrink=0.7, pad=0.03)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_annotate_fontweight_fontstyle():
    """annotate accepts fontweight and fontstyle."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('styled', xy=(2, 4), fontweight='bold', fontstyle='italic')
    plt.close()


def test_colorbar_horizontal_renders():
    """Horizontal colorbar renders to a valid PNG."""
    fig, ax = plt.subplots()
    data = np.random.rand(10, 10).tolist()
    ax.imshow(data)
    ax.colorbar(orientation='horizontal', label='Value', shrink=0.9)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_annotate_bbox_renders():
    """annotate with bbox renders to a valid PNG."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.annotate('boxed', xy=(3, 9), xytext=(2, 7),
                arrowprops=dict(color='blue'),
                bbox=dict(boxstyle='round', facecolor='#ffffcc',
                          edgecolor='gray', alpha=0.8))
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()
