"""Tests for marker types."""
import rustplotlib.pyplot as plt
import tempfile, os


MARKERS_TO_TEST = ['.', 'o', 's', '^', 'v', '<', '>', '+', 'x', 'D', 'd', '*',
                   'p', 'h', 'H', '8', '|', '_', 'P', 'X',
                   '1', '2', '3', '4']


def test_scatter_all_markers():
    """All markers render without error in scatter."""
    fig, ax = plt.subplots()
    for i, m in enumerate(MARKERS_TO_TEST):
        ax.scatter([i], [0], marker=m)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_plot_all_markers():
    """All markers render without error in plot."""
    fig, ax = plt.subplots()
    for i, m in enumerate(MARKERS_TO_TEST):
        ax.plot([i], [0], marker=m)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_triangle_left_right():
    """Triangle left and right markers work."""
    fig, ax = plt.subplots()
    ax.scatter([1, 2], [1, 2], marker='<')
    ax.scatter([3, 4], [3, 4], marker='>')
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_polygon_markers():
    """Pentagon, hexagon, octagon markers work."""
    fig, ax = plt.subplots()
    ax.scatter([1], [1], marker='p', s=200)
    ax.scatter([2], [1], marker='h', s=200)
    ax.scatter([3], [1], marker='H', s=200)
    ax.scatter([4], [1], marker='8', s=200)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_line_markers():
    """Vline and hline markers work."""
    fig, ax = plt.subplots()
    ax.scatter([1, 2], [1, 2], marker='|')
    ax.scatter([3, 4], [3, 4], marker='_')
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tri_markers():
    """Tri markers (1,2,3,4) work."""
    fig, ax = plt.subplots()
    for i, m in enumerate(['1', '2', '3', '4']):
        ax.scatter([i], [0], marker=m)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_marker_none_no_crash():
    """marker='none' or marker='' produces no marker."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9], marker='none')
    ax.plot([1, 2, 3], [1, 4, 9], marker='')
    plt.close()
