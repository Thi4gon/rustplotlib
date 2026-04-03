"""Benchmark: rustplotlib vs matplotlib performance comparison."""
import time
import numpy as np
import os


def benchmark_line_plot(plt_module, name, n_points=10000):
    x = np.linspace(0, 10, n_points)
    y = np.sin(x) * np.exp(-x / 5)
    t0 = time.perf_counter()
    for _ in range(10):
        plt_module.figure()
        plt_module.plot(x, y, label="sin(x)*exp(-x/5)")
        plt_module.title(f"Line Plot ({n_points} points)")
        plt_module.xlabel("X")
        plt_module.ylabel("Y")
        plt_module.legend()
        plt_module.grid(True)
        plt_module.savefig(f"/tmp/bench_{name}_line.png")
        plt_module.close()
    return (time.perf_counter() - t0) / 10


def benchmark_scatter(plt_module, name, n_points=5000):
    np.random.seed(42)
    x = np.random.randn(n_points)
    y = np.random.randn(n_points)
    t0 = time.perf_counter()
    for _ in range(10):
        plt_module.figure()
        plt_module.scatter(x, y, alpha=0.5)
        plt_module.title(f"Scatter ({n_points} points)")
        plt_module.savefig(f"/tmp/bench_{name}_scatter.png")
        plt_module.close()
    return (time.perf_counter() - t0) / 10


def benchmark_bar(plt_module, name):
    x = list(range(50))
    heights = np.random.rand(50).tolist()
    t0 = time.perf_counter()
    for _ in range(10):
        plt_module.figure()
        plt_module.bar(x, heights)
        plt_module.title("Bar Chart")
        plt_module.savefig(f"/tmp/bench_{name}_bar.png")
        plt_module.close()
    return (time.perf_counter() - t0) / 10


def benchmark_hist(plt_module, name, n_points=100000):
    np.random.seed(42)
    data = np.random.randn(n_points).tolist()
    t0 = time.perf_counter()
    for _ in range(10):
        plt_module.figure()
        plt_module.hist(data, bins=50)
        plt_module.title(f"Histogram ({n_points} points)")
        plt_module.savefig(f"/tmp/bench_{name}_hist.png")
        plt_module.close()
    return (time.perf_counter() - t0) / 10


def benchmark_subplots(plt_module, name):
    np.random.seed(42)
    t0 = time.perf_counter()
    for _ in range(10):
        fig, axes = plt_module.subplots(2, 2)
        axes[0][0].plot([1, 2, 3, 4], [1, 4, 2, 3])
        axes[0][0].set_title("Line")
        axes[0][1].scatter(
            np.random.randn(100).tolist(),
            np.random.randn(100).tolist(),
        )
        axes[0][1].set_title("Scatter")
        axes[1][0].bar([1, 2, 3], [3, 1, 4])
        axes[1][0].set_title("Bar")
        axes[1][1].hist(np.random.randn(1000).tolist(), bins=20)
        axes[1][1].set_title("Hist")
        fig.savefig(f"/tmp/bench_{name}_subplots.png")
        plt_module.close()
    return (time.perf_counter() - t0) / 10


def run_benchmarks():
    print("=" * 60)
    print("RUSTPLOTLIB vs MATPLOTLIB — Performance Benchmark")
    print("=" * 60)

    import rustplotlib.pyplot as rplt
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as mplt

    benchmarks = [
        ("Line Plot (10k pts)", benchmark_line_plot),
        ("Scatter (5k pts)", benchmark_scatter),
        ("Bar Chart (50 bars)", benchmark_bar),
        ("Histogram (100k pts)", benchmark_hist),
        ("Subplots 2x2", benchmark_subplots),
    ]

    results = []
    for name, bench_fn in benchmarks:
        print(f"\n--- {name} ---")
        t_mpl = bench_fn(mplt, "mpl")
        t_rust = bench_fn(rplt, "rust")
        speedup = t_mpl / t_rust if t_rust > 0 else float("inf")
        results.append((name, t_mpl, t_rust, speedup))
        print(f"  matplotlib: {t_mpl:.4f}s")
        print(f"  rustplotlib: {t_rust:.4f}s")
        print(f"  speedup:    {speedup:.1f}x")

    print("\n" + "=" * 60)
    print(f"{'Benchmark':<25} {'matplotlib':>12} {'rustplotlib':>12} {'speedup':>10}")
    print("-" * 60)
    for name, t_mpl, t_rust, speedup in results:
        print(f"{name:<25} {t_mpl:>11.4f}s {t_rust:>11.4f}s {speedup:>9.1f}x")
    print("=" * 60)


if __name__ == "__main__":
    run_benchmarks()
