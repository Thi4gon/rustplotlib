"""Tests for layout management (tight_layout, constrained_layout, add_axes)."""
import rustplotlib.pyplot as plt
import tempfile
import os


def test_tight_layout_no_crash():
    """tight_layout runs without error."""
    fig, axes = plt.subplots(2, 2)
    flat_axes = []
    if hasattr(axes, 'flat'):
        flat_axes = list(axes.flat)
    elif isinstance(axes, list):
        for item in axes:
            if isinstance(item, list):
                flat_axes.extend(item)
            else:
                flat_axes.append(item)
    else:
        flat_axes = [axes]

    for ax in flat_axes:
        ax.plot([1, 2, 3], [1, 4, 9])
        ax.set_title("Test")
        ax.set_xlabel("X")
        ax.set_ylabel("Y")
    plt.tight_layout()
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        plt.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tight_layout_with_pad():
    """tight_layout accepts pad parameter."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    fig.tight_layout(pad=2.0)
    plt.close()


def test_tight_layout_with_hpad_wpad():
    """tight_layout accepts h_pad and w_pad."""
    fig, axes = plt.subplots(2, 2)
    fig.tight_layout(h_pad=3.0, w_pad=3.0)
    plt.close()


def test_constrained_layout_subplots():
    """subplots accepts constrained_layout parameter."""
    fig, ax = plt.subplots(constrained_layout=True)
    ax.plot([1, 2, 3], [1, 4, 9])
    plt.close()


def test_constrained_layout_figure():
    """figure accepts constrained_layout parameter."""
    fig = plt.figure(constrained_layout=True)
    plt.close()


def test_fig_add_axes_returns_proxy():
    """fig.add_axes returns a usable AxesProxy."""
    fig, ax = plt.subplots()
    ax2 = fig.add_axes([0.6, 0.6, 0.3, 0.3])
    assert ax2 is not None
    assert hasattr(ax2, 'plot')
    ax2.plot([1, 2], [3, 4])
    plt.close()


def test_subplots_adjust_functional():
    """subplots_adjust changes layout."""
    fig, axes = plt.subplots(2, 2)
    fig.subplots_adjust(hspace=0.5, wspace=0.5)
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_tight_layout_module_level():
    """plt.tight_layout() works."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    plt.tight_layout()
    plt.close()


def test_suptitle_with_tight_layout():
    """suptitle + tight_layout doesn't crash."""
    fig, axes = plt.subplots(2, 1)
    fig.suptitle("Main Title")
    plt.tight_layout()
    plt.close()


def test_tight_layout_single_axes():
    """tight_layout works with a single axes."""
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [1, 4, 9])
    ax.set_xlabel("X label")
    ax.set_ylabel("Y label")
    ax.set_title("A title")
    fig.tight_layout()
    plt.close()


def test_constrained_layout_grid():
    """constrained_layout works with a 2x3 grid."""
    fig, axes = plt.subplots(2, 3, constrained_layout=True)
    for row in axes:
        for ax in row:
            ax.plot([0, 1], [0, 1])
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        fig.savefig(f.name)
        assert os.path.getsize(f.name) > 0
        os.unlink(f.name)
    plt.close()


def test_get_tight_layout():
    """get_tight_layout returns False by default."""
    fig, ax = plt.subplots()
    assert fig.get_tight_layout() is False
    plt.close()


def test_set_tight_layout():
    """set_tight_layout enables tight layout."""
    fig, ax = plt.subplots()
    ax.plot([1, 2], [3, 4])
    fig.set_tight_layout(True)
    assert fig.get_tight_layout() is True
    plt.close()
