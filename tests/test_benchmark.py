"""
Benchmark: rustplotlib vs matplotlib performance comparison.

Usage:
    python tests/test_benchmark.py              # Run all benchmarks
    python tests/test_benchmark.py --quick      # Quick mode (fewer iterations)
    python tests/test_benchmark.py --export     # Export results to CSV
"""
import time
import sys
import os
import numpy as np


# ============================================================
# BENCHMARK FUNCTIONS
# ============================================================

def benchmark_line_plot(plt, name, n_points=10000, iters=10):
    """Line plot with 10k points, labels, legend, grid."""
    x = np.linspace(0, 10, n_points)
    y = np.sin(x) * np.exp(-x / 5)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.plot(x, y, label="sin(x)*exp(-x/5)")
        plt.title(f"Line Plot ({n_points} points)")
        plt.xlabel("X")
        plt.ylabel("Y")
        plt.legend()
        plt.grid(True)
        plt.savefig(f"/tmp/bench_{name}_line.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_scatter(plt, name, n_points=5000, iters=10):
    """Scatter plot with 5k points."""
    np.random.seed(42)
    x = np.random.randn(n_points)
    y = np.random.randn(n_points)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.scatter(x, y, alpha=0.5)
        plt.title(f"Scatter ({n_points} points)")
        plt.savefig(f"/tmp/bench_{name}_scatter.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_bar(plt, name, n_bars=50, iters=10):
    """Bar chart with 50 bars."""
    np.random.seed(42)
    x = list(range(n_bars))
    heights = np.random.rand(n_bars).tolist()
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.bar(x, heights)
        plt.title("Bar Chart")
        plt.savefig(f"/tmp/bench_{name}_bar.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_hist(plt, name, n_points=100000, iters=10):
    """Histogram with 100k points, 50 bins."""
    np.random.seed(42)
    data = np.random.randn(n_points).tolist()
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.hist(data, bins=50)
        plt.title(f"Histogram ({n_points} points)")
        plt.savefig(f"/tmp/bench_{name}_hist.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_subplots(plt, name, iters=10):
    """2x2 subplots with mixed plot types."""
    np.random.seed(42)
    t0 = time.perf_counter()
    for _ in range(iters):
        fig, axes = plt.subplots(2, 2)
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
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_heatmap(plt, name, size=100, iters=10):
    """Heatmap (imshow) with 100x100 grid."""
    np.random.seed(42)
    data = np.random.randn(size, size).tolist()
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.imshow(data, cmap='viridis')
        plt.title(f"Heatmap ({size}x{size})")
        plt.savefig(f"/tmp/bench_{name}_heatmap.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_large_line(plt, name, n_points=100000, iters=5):
    """Large line plot with 100k points — stress test."""
    x = np.linspace(0, 100, n_points)
    y = np.sin(x) + 0.1 * np.random.randn(n_points)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure(figsize=(12, 6))
        plt.plot(x, y, linewidth=0.5)
        plt.title(f"Large Line ({n_points} points)")
        plt.savefig(f"/tmp/bench_{name}_large.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_multi_line(plt, name, n_lines=20, iters=10):
    """Multiple overlaid lines."""
    np.random.seed(42)
    x = np.linspace(0, 10, 1000)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        for i in range(n_lines):
            plt.plot(x, np.sin(x + i * 0.3) + np.random.randn(1000) * 0.1,
                     label=f"Line {i}" if i < 5 else None)
        plt.title(f"{n_lines} Overlaid Lines")
        plt.legend()
        plt.savefig(f"/tmp/bench_{name}_multi.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_errorbar(plt, name, iters=10):
    """Error bar plot."""
    np.random.seed(42)
    x = np.arange(30)
    y = np.random.rand(30) * 10
    yerr = np.random.rand(30)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.errorbar(x.tolist(), y.tolist(), yerr=yerr.tolist())
        plt.title("Error Bars")
        plt.savefig(f"/tmp/bench_{name}_errorbar.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_pie(plt, name, iters=10):
    """Pie chart."""
    sizes = [35, 25, 20, 15, 5]
    labels = ['A', 'B', 'C', 'D', 'E']
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.pie(sizes, labels=labels)
        plt.title("Pie Chart")
        plt.savefig(f"/tmp/bench_{name}_pie.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_savefig_svg(plt, name, iters=10):
    """SVG output benchmark."""
    x = np.linspace(0, 10, 1000)
    y = np.sin(x)
    t0 = time.perf_counter()
    for _ in range(iters):
        plt.figure()
        plt.plot(x, y)
        plt.title("SVG Output")
        plt.grid(True)
        plt.savefig(f"/tmp/bench_{name}_output.svg")
        plt.close()
    return (time.perf_counter() - t0) / iters


def benchmark_styled(plt, name, iters=10):
    """Plot with full styling: grid, legend, custom ticks, labels."""
    np.random.seed(42)
    x = np.linspace(0, 10, 500)
    t0 = time.perf_counter()
    for _ in range(iters):
        fig, ax = plt.subplots(figsize=(8, 6))
        ax.plot(x, np.sin(x), 'r-', linewidth=2, label='sin(x)')
        ax.plot(x, np.cos(x), 'b--', linewidth=2, label='cos(x)')
        ax.set_title("Styled Plot", fontsize=16)
        ax.set_xlabel("X Axis", fontsize=12)
        ax.set_ylabel("Y Axis", fontsize=12)
        ax.set_xlim(0, 10)
        ax.set_ylim(-1.5, 1.5)
        ax.legend()
        ax.grid(True)
        fig.savefig(f"/tmp/bench_{name}_styled.png")
        plt.close()
    return (time.perf_counter() - t0) / iters


# ============================================================
# RUNNER
# ============================================================

def run_benchmarks(quick=False, export=False):
    print("=" * 70)
    print("RUSTPLOTLIB vs MATPLOTLIB — Performance Benchmark")
    print("=" * 70)

    # Import both libs
    import rustplotlib.pyplot as rplt
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as mplt

    iters = 3 if quick else 10

    benchmarks = [
        ("Line Plot (10k pts)",     lambda p, n: benchmark_line_plot(p, n, iters=iters)),
        ("Scatter (5k pts)",        lambda p, n: benchmark_scatter(p, n, iters=iters)),
        ("Bar Chart (50 bars)",     lambda p, n: benchmark_bar(p, n, iters=iters)),
        ("Histogram (100k pts)",    lambda p, n: benchmark_hist(p, n, iters=iters)),
        ("Subplots 2x2",           lambda p, n: benchmark_subplots(p, n, iters=iters)),
        ("Heatmap (100x100)",      lambda p, n: benchmark_heatmap(p, n, iters=iters)),
        ("Large Line (100k pts)",  lambda p, n: benchmark_large_line(p, n, iters=max(iters//2, 1))),
        ("Multi-line (20 lines)",  lambda p, n: benchmark_multi_line(p, n, iters=iters)),
        ("Error Bars",             lambda p, n: benchmark_errorbar(p, n, iters=iters)),
        ("Pie Chart",              lambda p, n: benchmark_pie(p, n, iters=iters)),
        ("SVG Output",             lambda p, n: benchmark_savefig_svg(p, n, iters=iters)),
        ("Full Styled Plot",       lambda p, n: benchmark_styled(p, n, iters=iters)),
    ]

    results = []
    for name, bench_fn in benchmarks:
        print(f"\n--- {name} ---")
        try:
            t_mpl = bench_fn(mplt, "mpl")
        except Exception as e:
            print(f"  matplotlib: FAILED ({e})")
            t_mpl = None

        try:
            t_rust = bench_fn(rplt, "rust")
        except Exception as e:
            print(f"  rustplotlib: FAILED ({e})")
            t_rust = None

        if t_mpl is not None and t_rust is not None:
            speedup = t_mpl / t_rust if t_rust > 0 else float("inf")
            results.append((name, t_mpl, t_rust, speedup))
            print(f"  matplotlib:  {t_mpl:.4f}s")
            print(f"  rustplotlib: {t_rust:.4f}s")
            faster = "rustplotlib" if speedup > 1 else "matplotlib"
            ratio = speedup if speedup > 1 else 1/speedup
            print(f"  winner:      {faster} ({ratio:.1f}x faster)")

    # Summary table
    print("\n" + "=" * 70)
    print(f"{'Benchmark':<28} {'matplotlib':>12} {'rustplotlib':>12} {'speedup':>10}")
    print("-" * 70)
    total_mpl = 0
    total_rust = 0
    for name, t_mpl, t_rust, speedup in results:
        marker = ">>>" if speedup > 5 else ">>" if speedup > 2 else ">" if speedup > 1 else "<"
        print(f"{name:<28} {t_mpl:>11.4f}s {t_rust:>11.4f}s {speedup:>8.1f}x {marker}")
        total_mpl += t_mpl
        total_rust += t_rust

    print("-" * 70)
    total_speedup = total_mpl / total_rust if total_rust > 0 else 0
    print(f"{'TOTAL':<28} {total_mpl:>11.4f}s {total_rust:>11.4f}s {total_speedup:>8.1f}x")
    print("=" * 70)
    print(f"\nOverall: rustplotlib is {total_speedup:.1f}x faster than matplotlib")

    # Export to CSV
    if export:
        csv_path = "benchmark_results.csv"
        with open(csv_path, "w") as f:
            f.write("benchmark,matplotlib_s,rustplotlib_s,speedup\n")
            for name, t_mpl, t_rust, speedup in results:
                f.write(f"{name},{t_mpl:.6f},{t_rust:.6f},{speedup:.2f}\n")
        print(f"\nResults exported to {csv_path}")

    # Cleanup temp files
    for f in os.listdir("/tmp"):
        if f.startswith("bench_"):
            try:
                os.remove(os.path.join("/tmp", f))
            except:
                pass


if __name__ == "__main__":
    quick = "--quick" in sys.argv
    export = "--export" in sys.argv
    run_benchmarks(quick=quick, export=export)
