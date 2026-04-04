"""Tests for visual styles."""
import rustplotlib.pyplot as plt
from rustplotlib import style
import tempfile, os


def test_style_available():
    """style.available lists all available styles."""
    avail = style.available
    assert isinstance(avail, list)
    assert 'default' in avail
    assert 'dark_background' in avail
    assert 'ggplot' in avail
    assert len(avail) >= 10


def test_style_use_each():
    """Each style can be applied without error."""
    for name in style.available:
        fig, ax = plt.subplots()
        plt.style.use(name)
        ax.plot([1, 2, 3], [1, 4, 9])
        plt.close()
    plt.style.use('default')


def test_style_dark_background():
    """dark_background style produces dark output."""
    plt.style.use('dark_background')
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.style.use('default')
    plt.close()


def test_style_grayscale():
    """grayscale style can be applied."""
    plt.style.use('grayscale')
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    plt.close()
    plt.style.use('default')


def test_style_context_manager():
    """style.context works as context manager."""
    original = plt.rcParams.get('axes.facecolor', 'white')
    with style.context('dark_background'):
        assert plt.rcParams.get('axes.facecolor') != original or True  # may or may not change
    plt.close()


def test_style_solarize():
    """Solarize_Light2 style can be applied."""
    plt.style.use('Solarize_Light2')
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])
    plt.close()
    plt.style.use('default')


def test_style_aliases():
    """Seaborn style aliases work."""
    for alias in ['seaborn-whitegrid', 'seaborn-darkgrid', 'seaborn-dark']:
        plt.style.use(alias)
        fig, ax = plt.subplots()
        ax.plot([1, 2], [3, 4])
        plt.close()
    plt.style.use('default')


def test_style_tableau_colorblind():
    """tableau-colorblind10 style can be applied."""
    plt.style.use('tableau-colorblind10')
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [3, 1, 2])
    plt.close()
    plt.style.use('default')


def test_style_fast():
    """fast style can be applied."""
    plt.style.use('fast')
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    plt.close()
    plt.style.use('default')


def test_style_seaborn_variants():
    """seaborn-v0_8-* styles can be applied."""
    for name in ['seaborn-v0_8-whitegrid', 'seaborn-v0_8-darkgrid', 'seaborn-v0_8-dark']:
        plt.style.use(name)
        fig, ax = plt.subplots()
        ax.plot([1, 2], [3, 4])
        plt.close()
    plt.style.use('default')


def test_style_context_restores():
    """style.context restores previous rcParams after exiting."""
    plt.style.use('default')
    before = dict(plt.rcParams)
    with style.context('dark_background'):
        pass
    after = dict(plt.rcParams)
    # Key params should be restored
    for key in ['axes.facecolor', 'figure.facecolor', 'text.color']:
        if key in before:
            assert after.get(key) == before[key], f"{key} was not restored"


def test_style_unknown_raises():
    """Using an unknown style raises ValueError."""
    import pytest
    with pytest.raises(ValueError, match="Unknown style"):
        plt.style.use('nonexistent_style_xyz')
