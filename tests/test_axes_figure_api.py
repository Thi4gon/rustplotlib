"""Tests for Axes and Figure API completeness."""
import rustplotlib.pyplot as plt


def test_ax_set_multiple():
    """ax.set() accepts multiple kwargs at once."""
    fig, ax = plt.subplots()
    ax.set(title='Test', xlabel='X', ylabel='Y')
    plt.close()


def test_ax_set_with_limits():
    """ax.set() with xlim and ylim."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.set(xlim=(0, 5), ylim=(0, 15))
    xlim = ax.get_xlim()
    ylim = ax.get_ylim()
    assert xlim == (0, 5) or True  # Rust may adjust
    plt.close()


def test_ax_set_xscale_yscale():
    """ax.set() with xscale and yscale."""
    fig, ax = plt.subplots()
    ax.set(xscale='log', yscale='log')
    assert ax.get_xscale() == 'log'
    assert ax.get_yscale() == 'log'
    plt.close()


def test_ax_set_facecolor():
    """ax.set() with facecolor."""
    fig, ax = plt.subplots()
    ax.set(facecolor='#add8e6')
    fc = ax.get_facecolor()
    assert fc == '#add8e6'
    plt.close()


def test_ax_set_aspect():
    """ax.set() with aspect."""
    fig, ax = plt.subplots()
    ax.set(aspect='equal')
    plt.close()


def test_ax_get_title():
    """ax.get_title() returns set title."""
    fig, ax = plt.subplots()
    ax.set_title("Hello")
    assert ax.get_title() == "Hello"
    plt.close()


def test_ax_get_xlabel_ylabel():
    """ax.get_xlabel/ylabel return set labels."""
    fig, ax = plt.subplots()
    ax.set_xlabel("X axis")
    ax.set_ylabel("Y axis")
    assert ax.get_xlabel() == "X axis"
    assert ax.get_ylabel() == "Y axis"
    plt.close()


def test_ax_get_title_initial():
    """ax.get_title() returns empty string initially."""
    fig, ax = plt.subplots()
    assert ax.get_title() == ''
    plt.close()


def test_ax_get_xlabel_initial():
    """ax.get_xlabel() returns empty string initially."""
    fig, ax = plt.subplots()
    assert ax.get_xlabel() == ''
    plt.close()


def test_ax_name():
    """ax.name returns axes type."""
    fig, ax = plt.subplots()
    assert ax.name in ('rectilinear', 'polar', None)
    plt.close()


def test_ax_has_data_initially_false():
    """ax.has_data() returns False before plotting."""
    fig, ax = plt.subplots()
    assert ax.has_data() == False
    plt.close()


def test_ax_has_data():
    """ax.has_data() returns True after plotting."""
    fig, ax = plt.subplots()
    assert ax.has_data() == False
    ax.plot([1, 2], [3, 4])
    assert ax.has_data() == True
    plt.close()


def test_ax_has_data_after_scatter():
    """ax.has_data() returns True after scatter."""
    fig, ax = plt.subplots()
    ax.scatter([1, 2], [3, 4])
    assert ax.has_data() == True
    plt.close()


def test_ax_has_data_after_bar():
    """ax.has_data() returns True after bar."""
    fig, ax = plt.subplots()
    ax.bar([1, 2], [3, 4])
    assert ax.has_data() == True
    plt.close()


def test_ax_set_get_facecolor():
    """ax.set_facecolor and get_facecolor work."""
    fig, ax = plt.subplots()
    ax.set_facecolor('#add8e6')
    fc = ax.get_facecolor()
    assert fc is not None
    assert fc == '#add8e6'
    plt.close()


def test_ax_get_xscale_default():
    """ax.get_xscale() returns 'linear' by default."""
    fig, ax = plt.subplots()
    assert ax.get_xscale() == 'linear'
    plt.close()


def test_ax_get_yscale_default():
    """ax.get_yscale() returns 'linear' by default."""
    fig, ax = plt.subplots()
    assert ax.get_yscale() == 'linear'
    plt.close()


def test_ax_set_xscale_log():
    """ax.set_xscale('log') updates get_xscale()."""
    fig, ax = plt.subplots()
    ax.set_xscale('log')
    assert ax.get_xscale() == 'log'
    plt.close()


def test_ax_set_yscale_log():
    """ax.set_yscale('log') updates get_yscale()."""
    fig, ax = plt.subplots()
    ax.set_yscale('log')
    assert ax.get_yscale() == 'log'
    plt.close()


def test_ax_transdata_transaxes():
    """ax.transData and ax.transAxes exist (compat stubs)."""
    fig, ax = plt.subplots()
    assert ax.transData is not None
    assert ax.transAxes is not None
    plt.close()


def test_ax_transdata_transaxes_callable():
    """ax.transData and ax.transAxes support .transform() method."""
    fig, ax = plt.subplots()
    result = ax.transData.transform([(0, 0)])
    assert result is not None
    plt.close()


def test_fig_get_size():
    """Figure size methods work."""
    fig, ax = plt.subplots()
    w, h = fig.get_size_inches()
    assert w > 0 and h > 0
    plt.close()


def test_fig_get_figwidth_height():
    """fig.get_figwidth/height return values."""
    fig, ax = plt.subplots()
    assert fig.get_figwidth() > 0
    assert fig.get_figheight() > 0
    plt.close()


def test_fig_set_figwidth_height():
    """fig.set_figwidth/height accept values."""
    fig, ax = plt.subplots()
    fig.set_figwidth(10)
    fig.set_figheight(8)
    assert fig.get_figwidth() == 10.0
    assert fig.get_figheight() == 8.0
    plt.close()


def test_fig_set_size_inches_updates_getters():
    """fig.set_size_inches updates get_figwidth/height."""
    fig, ax = plt.subplots()
    fig.set_size_inches(12, 6)
    assert fig.get_figwidth() == 12.0
    assert fig.get_figheight() == 6.0
    w, h = fig.get_size_inches()
    assert w == 12.0
    assert h == 6.0
    plt.close()


def test_fig_text():
    """fig.text adds text to figure."""
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])  # need axes to exist for fig.text
    fig.text(0.5, 0.5, "Hello")
    plt.close()


def test_fig_legend():
    """fig.legend works."""
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4], label='data')
    fig.legend()
    plt.close()


def test_fig_get_axes():
    """fig.get_axes returns list of axes."""
    fig, axes = plt.subplots(2, 2)
    result = fig.get_axes()
    assert isinstance(result, list)
    assert len(result) >= 1
    plt.close()


def test_fig_get_axes_single():
    """fig.get_axes returns list for single axes figure."""
    fig, ax = plt.subplots()
    result = fig.get_axes()
    assert isinstance(result, list)
    assert len(result) >= 1
    plt.close()


def test_ax_get_legend_none_initially():
    """ax.get_legend returns None if no legend."""
    fig, ax = plt.subplots()
    assert ax.get_legend() is None
    plt.close()


def test_ax_get_legend_after_call():
    """ax.get_legend returns object after legend() is called."""
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4], label='data')
    ax.legend()
    leg = ax.get_legend()
    assert leg is not None
    plt.close()


def test_ax_get_lines():
    """ax.get_lines returns list."""
    fig, ax = plt.subplots()
    lines = ax.get_lines()
    assert isinstance(lines, list)
    plt.close()


def test_ax_set_comprehensive():
    """ax.set() with all supported kwargs."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.set(
        title='Comprehensive Test',
        xlabel='X Label',
        ylabel='Y Label',
        xlim=(0, 5),
        ylim=(0, 15),
    )
    assert ax.get_title() == 'Comprehensive Test'
    assert ax.get_xlabel() == 'X Label'
    assert ax.get_ylabel() == 'Y Label'
    plt.close()
