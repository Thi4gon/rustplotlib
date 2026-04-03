import os
import rustplot.pyplot as plt


def test_basic_plot():
    plt.plot([1, 2, 3], [4, 5, 6])
    plt.title("Test")
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.savefig("/tmp/test_basic.png")
    plt.close()
    assert os.path.exists("/tmp/test_basic.png")
    os.remove("/tmp/test_basic.png")


def test_subplots():
    fig, axes = plt.subplots(2, 2)
    assert len(axes) == 2
    assert len(axes[0]) == 2
    axes[0][0].plot([1, 2], [3, 4])
    axes[1][1].scatter([1, 2, 3], [4, 5, 6])
    fig.savefig("/tmp/test_subplots.png")
    plt.close()
    assert os.path.exists("/tmp/test_subplots.png")
    os.remove("/tmp/test_subplots.png")


def test_scatter():
    plt.scatter([1, 2, 3], [4, 5, 6], c="red", label="pts")
    plt.legend()
    plt.savefig("/tmp/test_scatter.png")
    plt.close()
    assert os.path.exists("/tmp/test_scatter.png")
    os.remove("/tmp/test_scatter.png")


def test_bar():
    plt.bar([1, 2, 3], [4, 5, 6])
    plt.savefig("/tmp/test_bar.png")
    plt.close()
    assert os.path.exists("/tmp/test_bar.png")
    os.remove("/tmp/test_bar.png")


def test_hist():
    import random
    random.seed(42)
    data = [random.gauss(0, 1) for _ in range(1000)]
    plt.hist(data, bins=30)
    plt.savefig("/tmp/test_hist.png")
    plt.close()
    assert os.path.exists("/tmp/test_hist.png")
    os.remove("/tmp/test_hist.png")


def test_grid_and_legend():
    plt.plot([1, 2, 3], [1, 4, 9], label="quadratic")
    plt.plot([1, 2, 3], [1, 2, 3], label="linear")
    plt.grid(True)
    plt.legend()
    plt.savefig("/tmp/test_styled.png")
    plt.close()
    assert os.path.exists("/tmp/test_styled.png")
    os.remove("/tmp/test_styled.png")


def test_figure_function():
    fig = plt.figure(figsize=(10, 8))
    plt.plot([1, 2], [3, 4])
    plt.savefig("/tmp/test_figure.png")
    plt.close()
    assert os.path.exists("/tmp/test_figure.png")
    os.remove("/tmp/test_figure.png")


def test_format_string():
    plt.plot([1, 2, 3], [1, 4, 9], "r--o")
    plt.savefig("/tmp/test_fmt.png")
    plt.close()
    assert os.path.exists("/tmp/test_fmt.png")
    os.remove("/tmp/test_fmt.png")


def test_plot_single_arg():
    plt.plot([10, 20, 30])
    plt.savefig("/tmp/test_single.png")
    plt.close()
    assert os.path.exists("/tmp/test_single.png")
    os.remove("/tmp/test_single.png")
