import os
import rustplotlib.pyplot as plt


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


def test_log_scale():
    fig, ax = plt.subplots()
    ax.plot([1, 10, 100, 1000], [1, 10, 100, 1000])
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_title("Log-Log Plot")
    fig.savefig("/tmp/test_log.png")
    plt.close()
    assert os.path.exists("/tmp/test_log.png")
    os.remove("/tmp/test_log.png")


def test_xscale_yscale_functions():
    plt.plot([0.1, 1, 10, 100], [1, 2, 3, 4])
    plt.xscale("log")
    plt.savefig("/tmp/test_xscale.png")
    plt.close()
    assert os.path.exists("/tmp/test_xscale.png")
    os.remove("/tmp/test_xscale.png")


def test_errorbar():
    fig, ax = plt.subplots()
    ax.errorbar([1, 2, 3, 4], [10, 20, 15, 25],
                yerr=[1, 2, 1.5, 3], marker="o", capsize=5.0,
                label="measurements")
    ax.legend()
    fig.savefig("/tmp/test_errorbar.png")
    plt.close()
    assert os.path.exists("/tmp/test_errorbar.png")
    os.remove("/tmp/test_errorbar.png")


def test_errorbar_function():
    plt.errorbar([1, 2, 3], [10, 20, 15], yerr=[1, 2, 1.5])
    plt.savefig("/tmp/test_errorbar_func.png")
    plt.close()
    assert os.path.exists("/tmp/test_errorbar_func.png")
    os.remove("/tmp/test_errorbar_func.png")


def test_barh():
    fig, ax = plt.subplots()
    ax.barh([1, 2, 3], [10, 20, 15], color="green", label="bars")
    ax.set_title("Horizontal Bars")
    fig.savefig("/tmp/test_barh.png")
    plt.close()
    assert os.path.exists("/tmp/test_barh.png")
    os.remove("/tmp/test_barh.png")


def test_barh_function():
    plt.barh([1, 2, 3, 4], [40, 30, 20, 10])
    plt.savefig("/tmp/test_barh_func.png")
    plt.close()
    assert os.path.exists("/tmp/test_barh_func.png")
    os.remove("/tmp/test_barh_func.png")


def test_boxplot():
    import random
    random.seed(42)
    data1 = [random.gauss(0, 1) for _ in range(50)]
    data2 = [random.gauss(2, 0.5) for _ in range(50)]
    fig, ax = plt.subplots()
    ax.boxplot([data1, data2])
    ax.set_title("Box Plot")
    fig.savefig("/tmp/test_boxplot.png")
    plt.close()
    assert os.path.exists("/tmp/test_boxplot.png")
    os.remove("/tmp/test_boxplot.png")


def test_boxplot_function():
    plt.boxplot([[1, 2, 3, 4, 5, 10, 20]])
    plt.savefig("/tmp/test_boxplot_func.png")
    plt.close()
    assert os.path.exists("/tmp/test_boxplot_func.png")
    os.remove("/tmp/test_boxplot_func.png")


def test_stem():
    fig, ax = plt.subplots()
    ax.stem([1, 2, 3, 4, 5], [1, 4, 2, 5, 3], label="stem")
    ax.legend()
    fig.savefig("/tmp/test_stem.png")
    plt.close()
    assert os.path.exists("/tmp/test_stem.png")
    os.remove("/tmp/test_stem.png")


def test_stem_single_arg():
    fig, ax = plt.subplots()
    ax.stem([3, 1, 4, 1, 5, 9])
    fig.savefig("/tmp/test_stem_single.png")
    plt.close()
    assert os.path.exists("/tmp/test_stem_single.png")
    os.remove("/tmp/test_stem_single.png")


def test_stem_function():
    plt.stem([1, 2, 3], [4, 5, 6])
    plt.savefig("/tmp/test_stem_func.png")
    plt.close()
    assert os.path.exists("/tmp/test_stem_func.png")
    os.remove("/tmp/test_stem_func.png")
