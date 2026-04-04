"""Tests for colormap coverage."""
import rustplotlib.pyplot as plt
import numpy as np
import tempfile, os


EXPECTED_COLORMAPS = [
    'viridis', 'plasma', 'inferno', 'magma', 'cividis',
    'hot', 'cool', 'jet', 'gray', 'bone', 'copper',
    'spring', 'summer', 'autumn', 'winter',
    'RdYlBu', 'RdBu', 'PiYG', 'PRGn', 'BrBG', 'Spectral',
    'Blues', 'Reds', 'Greens', 'Oranges', 'Purples',
    'Set1', 'Set2', 'Set3', 'tab10', 'tab20',
    'twilight', 'turbo', 'ocean', 'terrain',
    'YlOrRd', 'YlGnBu', 'binary', 'pink', 'gist_heat',
    'Pastel1', 'Pastel2', 'tab20b', 'tab20c',
    # New colormaps
    'twilight_shifted', 'hsv',
    'afmhot', 'Wistia',
    'YlOrBr', 'YlGn', 'GnBu', 'PuBu', 'PuRd', 'OrRd', 'BuGn', 'BuPu',
    'Accent', 'Dark2', 'Paired',
    'rainbow', 'gist_rainbow', 'gnuplot', 'gnuplot2',
    'CMRmap', 'cubehelix', 'brg',
    'gist_earth', 'gist_stern', 'gist_ncar',
]


def test_all_expected_colormaps_exist():
    """All expected colormaps are available."""
    available = plt.colormaps()
    for name in EXPECTED_COLORMAPS:
        assert name in available, f"Colormap '{name}' not found"


def test_colormaps_render():
    """Each colormap renders without error."""
    fig, ax = plt.subplots()
    data = np.random.rand(5, 5).tolist()
    for cmap_name in EXPECTED_COLORMAPS[:10]:  # test a subset to keep fast
        ax.imshow(data, cmap=cmap_name)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_new_colormaps_render():
    """New colormaps render without error."""
    new_cmaps = [
        'twilight_shifted', 'hsv', 'afmhot', 'Wistia',
        'YlOrBr', 'YlGn', 'GnBu', 'PuBu', 'PuRd', 'OrRd',
        'BuGn', 'BuPu', 'Accent', 'Dark2', 'Paired',
        'rainbow', 'gist_rainbow', 'gnuplot', 'gnuplot2',
        'CMRmap', 'cubehelix', 'brg', 'gist_earth', 'gist_stern', 'gist_ncar',
    ]
    fig, ax = plt.subplots()
    data = np.random.rand(5, 5).tolist()
    for cmap_name in new_cmaps:
        ax.imshow(data, cmap=cmap_name)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_reversed_colormaps():
    """Reversed colormaps (_r suffix) are available."""
    available = plt.colormaps()
    for name in ['viridis_r', 'plasma_r', 'hot_r', 'RdBu_r', 'jet_r',
                 'cubehelix_r', 'rainbow_r', 'hsv_r', 'gnuplot_r']:
        assert name in available, f"Reversed colormap '{name}' not found"


def test_get_cmap():
    """plt.get_cmap returns a colormap name."""
    cmap = plt.get_cmap('viridis')
    assert cmap is not None


def test_colormap_count():
    """At least 100 colormaps available (base + reversed)."""
    available = plt.colormaps()
    assert len(available) >= 100, f"Only {len(available)} colormaps"


def test_colormap_count_base():
    """At least 50 base colormaps available."""
    available = plt.colormaps()
    base = [c for c in available if not c.endswith('_r')]
    assert len(base) >= 50, f"Only {len(base)} base colormaps"
