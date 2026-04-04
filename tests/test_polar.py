"""Tests for polar plot support."""
import rustplotlib.pyplot as plt
import numpy as np
import tempfile
import os


def test_polar_basic():
    """Basic polar plot works."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    theta = np.linspace(0, 2 * np.pi, 100)
    r = 1 + np.cos(theta)
    ax.plot(theta, r)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_polar_subplot():
    """subplot_polar creates polar axes."""
    ax = plt.subplot_polar()
    ax.plot([0, np.pi / 2, np.pi], [1, 2, 1])
    plt.close()


def test_polar_set_rmax():
    """set_rmax sets maximum radius."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.plot([0, 1, 2], [1, 2, 3])
    ax.set_rmax(5)
    plt.close()


def test_polar_set_rmin():
    """set_rmin sets minimum radius."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.plot([0, 1, 2], [1, 2, 3])
    ax.set_rmax(5)
    ax.set_rmin(0.5)
    plt.close()


def test_polar_set_rticks():
    """set_rticks sets radial ticks."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.plot([0, 1, 2], [1, 2, 3])
    ax.set_rticks([0.5, 1, 1.5, 2])
    plt.close()


def test_polar_set_rticks_with_labels():
    """set_rticks with custom labels."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.plot([0, 1, 2], [1, 2, 3])
    ax.set_rticks([0.5, 1.0, 1.5, 2.0], labels=['0.5', '1', '1.5', '2'])
    plt.close()


def test_polar_theta_zero_location():
    """set_theta_zero_location accepts direction strings."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_theta_zero_location('N')
    assert hasattr(ax, '_polar_theta_zero')
    ax.set_theta_zero_location('S')
    ax.set_theta_zero_location('E')
    ax.set_theta_zero_location('W')
    ax.set_theta_zero_location('NE')
    ax.set_theta_zero_location('NW')
    plt.close()


def test_polar_theta_zero_location_offset():
    """set_theta_zero_location accepts offset."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_theta_zero_location('N', offset=10)
    assert ax._polar_theta_zero == 90.0 + 10.0
    plt.close()


def test_polar_theta_direction():
    """set_theta_direction accepts 1 or -1."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_theta_direction(-1)
    assert ax._polar_theta_direction == -1
    ax.set_theta_direction(1)
    assert ax._polar_theta_direction == 1
    ax.set_theta_direction('clockwise')
    assert ax._polar_theta_direction == -1
    ax.set_theta_direction('counterclockwise')
    assert ax._polar_theta_direction == 1
    plt.close()


def test_polar_rlabel_position():
    """set_rlabel_position stores angle."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_rlabel_position(45)
    assert ax._polar_rlabel_position == 45.0
    ax.set_rlabel_position(135.0)
    assert ax._polar_rlabel_position == 135.0
    plt.close()


def test_polar_set_thetagrids():
    """set_thetagrids sets angular grid positions."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_thetagrids([0, 45, 90, 135, 180, 225, 270, 315])
    assert hasattr(ax, '_polar_thetagrids')
    assert len(ax._polar_thetagrids) == 8
    plt.close()


def test_polar_set_thetagrids_custom_labels():
    """set_thetagrids with custom labels."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_thetagrids([0, 90, 180, 270], labels=['E', 'N', 'W', 'S'])
    assert ax._polar_thetagrids_labels == ['E', 'N', 'W', 'S']
    plt.close()


def test_polar_set_rgrids():
    """set_rgrids sets radial grid positions."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    ax.set_rgrids([0.5, 1.0, 1.5, 2.0])
    assert hasattr(ax, '_polar_rgrids')
    assert ax._polar_rgrids == [0.5, 1.0, 1.5, 2.0]
    plt.close()


def test_polar_full_example():
    """Full polar plot with customization."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    theta = np.linspace(0, 2 * np.pi, 50)
    r = 2 + np.sin(5 * theta)
    ax.plot(theta, r, 'r-')
    ax.set_title("Rose Curve")
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_polar_full_customization():
    """Full polar customization pipeline."""
    fig, ax = plt.subplots(subplot_kw={'projection': 'polar'})
    theta = np.linspace(0, 2 * np.pi, 100)
    r = np.abs(np.sin(3 * theta))
    ax.plot(theta, r)
    ax.set_theta_zero_location('N')
    ax.set_theta_direction(-1)
    ax.set_rlabel_position(45)
    ax.set_rmax(1.5)
    ax.set_rticks([0.25, 0.5, 0.75, 1.0, 1.25])
    ax.set_thetagrids([0, 60, 120, 180, 240, 300])
    ax.set_rgrids([0.25, 0.5, 0.75, 1.0, 1.25])
    ax.set_title("3-petal rose")
    plt.close()
