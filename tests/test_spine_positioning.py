"""Tests for spine positioning and customization."""
import rustplotlib.pyplot as plt


def test_spine_set_position_center():
    """Spine accepts set_position('center')."""
    fig, ax = plt.subplots()
    ax.spines['left'].set_position('center')
    plt.close()


def test_spine_set_position_data():
    """Spine accepts set_position(('data', 0))."""
    fig, ax = plt.subplots()
    ax.spines['bottom'].set_position(('data', 0))
    plt.close()


def test_spine_set_position_zero():
    """Spine accepts set_position('zero') — alias for ('data', 0)."""
    fig, ax = plt.subplots()
    ax.spines['left'].set_position('zero')
    plt.close()


def test_spine_set_position_zero_normalises():
    """set_position('zero') stores ('data', 0) internally."""
    fig, ax = plt.subplots()
    spine = ax.spines['left']
    spine.set_position('zero')
    assert spine.get_position() == ('data', 0)
    plt.close()


def test_spine_set_position_axes_fraction():
    """Spine accepts set_position(('axes', 0.5))."""
    fig, ax = plt.subplots()
    ax.spines['bottom'].set_position(('axes', 0.5))
    plt.close()


def test_spine_get_position():
    """get_position() returns the value set by set_position()."""
    fig, ax = plt.subplots()
    spine = ax.spines['bottom']
    spine.set_position(('data', 3.14))
    assert spine.get_position() == ('data', 3.14)
    plt.close()


def test_spine_get_position_none_by_default():
    """get_position() returns None when set_position() was never called."""
    fig, ax = plt.subplots()
    assert ax.spines['top'].get_position() is None
    plt.close()


def test_spines_values():
    """ax.spines.values() returns iterable of SpineProxy."""
    fig, ax = plt.subplots()
    spines = list(ax.spines.values())
    assert len(spines) == 4  # top, right, bottom, left
    plt.close()


def test_spines_items():
    """ax.spines.items() returns iterable of (name, SpineProxy)."""
    fig, ax = plt.subplots()
    items = dict(ax.spines.items())
    assert 'top' in items
    assert 'right' in items
    assert 'bottom' in items
    assert 'left' in items
    plt.close()


def test_spines_keys():
    """ax.spines.keys() returns the four spine names."""
    fig, ax = plt.subplots()
    keys = ax.spines.keys()
    assert set(keys) == {'top', 'right', 'bottom', 'left'}
    plt.close()


def test_spines_contains():
    """'left' in ax.spines works."""
    fig, ax = plt.subplots()
    assert 'left' in ax.spines
    assert 'unknown' not in ax.spines
    plt.close()


def test_math_axes_pattern():
    """Common pattern: math-style axes with spines at origin."""
    fig, ax = plt.subplots()
    ax.plot([-5, 5], [-5, 5])
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['bottom'].set_position('zero')
    ax.spines['left'].set_position('zero')
    # Should not crash and should save
    import tempfile, os
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_spine_set_linewidth():
    """Spine accepts set_linewidth."""
    fig, ax = plt.subplots()
    ax.spines['left'].set_linewidth(2.0)
    plt.close()


def test_spine_set_lw_alias():
    """set_lw is an alias for set_linewidth."""
    fig, ax = plt.subplots()
    ax.spines['bottom'].set_lw(1.5)
    plt.close()


def test_spine_set_color():
    """Spine accepts set_color."""
    fig, ax = plt.subplots()
    ax.spines['left'].set_color('red')
    plt.close()


def test_spine_set_edgecolor():
    """set_edgecolor is an alias for set_color."""
    fig, ax = plt.subplots()
    ax.spines['left'].set_edgecolor('blue')
    plt.close()


def test_spine_set_linestyle():
    """Spine accepts set_linestyle without crashing."""
    fig, ax = plt.subplots()
    ax.spines['bottom'].set_linestyle('--')
    plt.close()


def test_spine_set_bounds():
    """Spine accepts set_bounds without crashing."""
    fig, ax = plt.subplots()
    ax.spines['bottom'].set_bounds(-1, 1)
    plt.close()


def test_spines_iterate():
    """Can iterate over spines with for loop."""
    fig, ax = plt.subplots()
    count = 0
    for spine in ax.spines.values():
        spine.set_visible(True)
        count += 1
    assert count == 4
    plt.close()


def test_spines_iterate_keys():
    """Can iterate over ax.spines directly to get names."""
    fig, ax = plt.subplots()
    names = list(ax.spines)
    assert set(names) == {'top', 'right', 'bottom', 'left'}
    plt.close()


def test_spine_chain_operations():
    """Multiple spine operations in sequence do not crash."""
    fig, ax = plt.subplots()
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_color('#333333')
    ax.spines['left'].set_linewidth(1.5)
    ax.spines['bottom'].set_color('#333333')
    ax.spines['bottom'].set_linewidth(1.5)
    plt.close()
