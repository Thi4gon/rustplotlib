"""Tests for triangulation plots (tripcolor, tricontour, tricontourf)."""
import numpy as np
import rustplotlib.pyplot as plt
import tempfile
import os


def test_tripcolor_basic():
    """tripcolor renders colored triangles (per-triangle C)."""
    fig, ax = plt.subplots()
    x = [0, 1, 0.5, 0, 1]
    y = [0, 0, 1, 1, 1]
    triangles = [[0, 1, 2], [0, 2, 3], [1, 4, 2]]
    C = [0.1, 0.5, 0.9]  # per-triangle
    ax.tripcolor(x, y, triangles, C)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tripcolor_per_vertex():
    """tripcolor with per-vertex colors (averaged per triangle)."""
    fig, ax = plt.subplots()
    x = [0, 1, 0.5]
    y = [0, 0, 1]
    triangles = [[0, 1, 2]]
    C = [0.0, 0.5, 1.0]  # per-vertex
    ax.tripcolor(x, y, triangles, C)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tripcolor_no_triangles():
    """tripcolor without explicit triangles uses fan triangulation."""
    fig, ax = plt.subplots()
    x = [0, 1, 1, 0]
    y = [0, 0, 1, 1]
    C = [0.0, 0.5, 1.0, 0.3]
    ax.tripcolor(x, y, C)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tripcolor_module_level():
    """Module-level tripcolor works."""
    x = [0, 1, 0.5]
    y = [0, 0, 1]
    triangles = [[0, 1, 2]]
    C = [0.5]
    plt.tripcolor(x, y, triangles, C)
    plt.close()


def test_tripcolor_vmin_vmax():
    """tripcolor respects vmin/vmax kwargs."""
    fig, ax = plt.subplots()
    x = [0, 1, 0.5, 0]
    y = [0, 0, 1, 1]
    triangles = [[0, 1, 2], [0, 2, 3]]
    C = [0.2, 0.8]
    ax.tripcolor(x, y, triangles, C, vmin=0.0, vmax=1.0)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tripcolor_alpha():
    """tripcolor with alpha transparency."""
    fig, ax = plt.subplots()
    x = [0, 1, 0.5]
    y = [0, 0, 1]
    triangles = [[0, 1, 2]]
    C = [0.7]
    ax.tripcolor(x, y, triangles, C, alpha=0.5)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tricontourf_basic():
    """tricontourf renders filled contours from triangulated data."""
    fig, ax = plt.subplots()
    np.random.seed(42)
    x = np.random.rand(20)
    y = np.random.rand(20)
    z = np.sin(x * 3) + np.cos(y * 3)
    n = len(x)
    triangles = [[i, (i + 1) % n, (i + 2) % n] for i in range(0, n - 2)]
    ax.tricontourf(x, y, triangles, z, levels=5)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tricontour_basic():
    """tricontour renders contour lines from triangulated data."""
    fig, ax = plt.subplots()
    np.random.seed(42)
    x = np.random.rand(20)
    y = np.random.rand(20)
    z = x ** 2 + y ** 2
    n = len(x)
    triangles = [[i, (i + 1) % n, (i + 2) % n] for i in range(0, n - 2)]
    ax.tricontour(x, y, triangles, z, levels=5)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tricontourf_no_triangles():
    """tricontourf without explicit triangles uses fan triangulation."""
    fig, ax = plt.subplots()
    x = [0, 1, 2, 0.5, 1.5, 1]
    y = [0, 0, 0, 1, 1, 2]
    z = [0, 1, 0, 1, 2, 1]
    ax.tricontourf(x, y, z, levels=3)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tricontour_module_level():
    """Module-level tricontour works."""
    x = [0, 1, 0.5, 0, 1]
    y = [0, 0, 1, 1, 1]
    z = [0, 1, 0.5, 0.3, 0.8]
    triangles = [[0, 1, 2], [0, 2, 3], [1, 4, 2]]
    plt.tricontour(x, y, triangles, z)
    plt.close()


def test_tricontourf_module_level():
    """Module-level tricontourf works."""
    x = [0, 1, 0.5, 0, 1]
    y = [0, 0, 1, 1, 1]
    z = [0, 1, 0.5, 0.3, 0.8]
    triangles = [[0, 1, 2], [0, 2, 3], [1, 4, 2]]
    plt.tricontourf(x, y, triangles, z)
    plt.close()


def test_tricontour_no_triangles():
    """tricontour without explicit triangles."""
    fig, ax = plt.subplots()
    x = [0, 1, 2, 0.5, 1.5, 1]
    y = [0, 0, 0, 1, 1, 2]
    z = [0, 1, 0, 1, 2, 1]
    ax.tricontour(x, y, z, levels=4)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()
