"""Tests for output formats (PNG, SVG, PDF, EPS)."""
import rustplotlib.pyplot as plt
import tempfile
import os


def test_savefig_png():
    """savefig produces valid PNG."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        with open(f.name, 'rb') as pf:
            assert pf.read(4) == b'\x89PNG'
        os.unlink(f.name)
    plt.close()


def test_savefig_svg():
    """savefig produces valid SVG."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    with tempfile.NamedTemporaryFile(suffix='.svg', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        with open(f.name, 'r') as sf:
            content = sf.read()
            assert '<svg' in content
        os.unlink(f.name)
    plt.close()


def test_savefig_pdf():
    """savefig produces valid PDF."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        with open(f.name, 'rb') as pf:
            assert pf.read(5) == b'%PDF-'
        os.unlink(f.name)
    plt.close()


def test_savefig_eps():
    """savefig produces valid EPS."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    with tempfile.NamedTemporaryFile(suffix='.eps', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        with open(f.name, 'r') as ef:
            content = ef.read()
            assert '%!PS-Adobe' in content
            assert 'BoundingBox' in content
        os.unlink(f.name)
    plt.close()


def test_rcparams_common_keys():
    """Common rcParams keys exist without KeyError."""
    keys_to_check = [
        'figure.figsize', 'figure.dpi', 'figure.facecolor',
        'axes.facecolor', 'axes.edgecolor', 'axes.linewidth',
        'axes.grid', 'axes.titlesize', 'axes.labelsize',
        'text.color', 'font.family', 'font.size',
        'lines.linewidth', 'lines.markersize',
        'grid.color', 'grid.linestyle', 'grid.linewidth',
        'legend.fontsize', 'legend.frameon',
        'savefig.dpi', 'savefig.transparent',
        'image.cmap',
    ]
    for key in keys_to_check:
        assert key in plt.rcParams, f"rcParams missing key: {key}"


def test_rcparams_new_keys():
    """Newly added rcParams keys exist."""
    new_keys = [
        'axes.spines.left', 'axes.spines.bottom',
        'axes.spines.top', 'axes.spines.right',
        'patch.linewidth', 'hist.bins', 'scatter.marker',
        'lines.linestyle', 'lines.color',
    ]
    for key in new_keys:
        assert key in plt.rcParams, f"rcParams missing key: {key}"
