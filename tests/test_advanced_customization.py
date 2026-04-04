"""Tests for advanced customization (norms, grid major/minor)."""
import numpy as np
import rustplotlib.pyplot as plt
from rustplotlib.colors import TwoSlopeNorm, CenteredNorm


def test_two_slope_norm_basic():
    """TwoSlopeNorm normalizes with different rates around center."""
    norm = TwoSlopeNorm(vcenter=0, vmin=-10, vmax=5)
    result = norm(0)
    assert abs(result - 0.5) < 0.01


def test_two_slope_norm_below_center():
    """Values below center use the lower slope."""
    norm = TwoSlopeNorm(vcenter=0, vmin=-10, vmax=5)
    result = norm(-5)
    assert abs(result - 0.25) < 0.01


def test_two_slope_norm_above_center():
    """Values above center use the upper slope."""
    norm = TwoSlopeNorm(vcenter=0, vmin=-10, vmax=5)
    result = norm(2.5)
    assert abs(result - 0.75) < 0.01


def test_two_slope_norm_array():
    """TwoSlopeNorm works on arrays."""
    norm = TwoSlopeNorm(vcenter=0, vmin=-10, vmax=10)
    result = norm(np.array([-10, -5, 0, 5, 10]))
    expected = np.array([0.0, 0.25, 0.5, 0.75, 1.0])
    np.testing.assert_allclose(result, expected, atol=0.01)


def test_two_slope_norm_auto_vmin_vmax():
    """TwoSlopeNorm auto-detects vmin/vmax from data."""
    norm = TwoSlopeNorm(vcenter=0)
    result = norm(np.array([-4, -2, 0, 2, 4]))
    assert abs(result[2] - 0.5) < 0.01  # center should be 0.5


def test_centered_norm_basic():
    """CenteredNorm normalizes symmetrically around center."""
    norm = CenteredNorm(vcenter=0, halfrange=10)
    result = norm(0)
    assert abs(result - 0.5) < 0.01


def test_centered_norm_symmetric():
    """CenteredNorm produces symmetric results."""
    norm = CenteredNorm(vcenter=0, halfrange=10)
    r_pos = norm(5)
    r_neg = norm(-5)
    assert abs(r_pos - (1 - r_neg)) < 0.01


def test_centered_norm_auto_halfrange():
    """CenteredNorm auto-detects halfrange."""
    norm = CenteredNorm(vcenter=0)
    data = np.array([-3, -1, 0, 2, 4])
    result = norm(data)
    # halfrange should be max(|4-0|, |-3-0|) = 4
    assert abs(result[2] - 0.5) < 0.01  # center
    assert abs(result[4] - 1.0) < 0.01  # max


def test_centered_norm_nonzero_center():
    """CenteredNorm works with non-zero center."""
    norm = CenteredNorm(vcenter=5, halfrange=5)
    assert abs(norm(5) - 0.5) < 0.01
    assert abs(norm(0) - 0.0) < 0.01
    assert abs(norm(10) - 1.0) < 0.01


def test_grid_which_major():
    """ax.grid(which='major') does not crash."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.grid(True, which='major')
    plt.close()


def test_grid_which_minor():
    """ax.grid(which='minor') does not crash."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.grid(True, which='minor')
    plt.close()


def test_grid_which_both():
    """ax.grid(which='both') does not crash."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.grid(True, which='both')
    plt.close()


def test_grid_with_kwargs():
    """ax.grid accepts linestyle, linewidth, alpha kwargs."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.grid(True, which='major', linestyle='-', linewidth=0.5, alpha=0.8)
    ax.grid(True, which='minor', linestyle=':', linewidth=0.3, alpha=0.3)
    import tempfile, os
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_norms_importable():
    """Norms are importable from colors module."""
    from rustplotlib.colors import Normalize, LogNorm, BoundaryNorm, TwoSlopeNorm, CenteredNorm
    assert callable(TwoSlopeNorm)
    assert callable(CenteredNorm)
