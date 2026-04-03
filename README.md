# RustPlot

A high-performance matplotlib drop-in replacement powered by Rust.

RustPlot reimplements matplotlib's core plotting functionality in Rust using [tiny-skia](https://github.com/nickel-org/tiny-skia) for rasterization and [PyO3](https://pyo3.rs/) for Python bindings. Just swap your import and get **up to 30x faster** plot generation.

## Installation

```bash
pip install rustplot
```

## Usage

```python
# Replace this:
# import matplotlib.pyplot as plt

# With this:
import rustplot.pyplot as plt

# Everything else stays the same!
plt.plot([1, 2, 3, 4], [1, 4, 2, 3], label="data")
plt.title("My Plot")
plt.xlabel("X")
plt.ylabel("Y")
plt.legend()
plt.grid(True)
plt.savefig("plot.png")
plt.show()
```

## Supported Features

- **Plot types:** `plot()`, `scatter()`, `bar()`, `hist()`, `imshow()`
- **Layout:** `subplots()`, `figure()`, `tight_layout()`
- **Customization:** titles, labels, legends, grid, colors, linestyles, markers
- **Output:** PNG, SVG, interactive window
- **Colors:** named colors, hex, RGB/RGBA tuples, shorthand (r/g/b/k/w/c/m/y)
- **Colormaps:** viridis, plasma, inferno, magma, hot, cool, gray, jet, Blues, Reds, Greens

## Performance

Benchmarked against matplotlib (lower is better):

| Benchmark | matplotlib | rustplot | Speedup |
|---|---|---|---|
| Line Plot (10k pts) | 0.064s | 0.002s | **30.8x** |
| Scatter (5k pts) | 0.029s | 0.017s | **1.7x** |
| Bar Chart (50 bars) | 0.023s | 0.002s | **9.6x** |
| Histogram (100k pts) | 0.081s | 0.003s | **27.9x** |
| Subplots 2x2 | 0.041s | 0.002s | **26.7x** |

## Subplots Example

```python
import rustplot.pyplot as plt
import numpy as np

fig, axes = plt.subplots(2, 2, figsize=(10, 8))

axes[0][0].plot(np.linspace(0, 10, 100), np.sin(np.linspace(0, 10, 100)))
axes[0][0].set_title("Sine Wave")

axes[0][1].scatter(np.random.randn(200), np.random.randn(200), alpha=0.5)
axes[0][1].set_title("Scatter")

axes[1][0].bar([1, 2, 3, 4, 5], [3, 7, 2, 5, 8], color="green")
axes[1][0].set_title("Bar Chart")

axes[1][1].hist(np.random.randn(5000), bins=40, color="orange")
axes[1][1].set_title("Histogram")

fig.savefig("subplots.png")
```

## Tech Stack

- **Rust** core with [tiny-skia](https://github.com/nickel-org/tiny-skia) (2D rasterization)
- **[ab_glyph](https://github.com/alexheretic/ab-glyph)** for font rendering
- **[PyO3](https://pyo3.rs/)** + **[maturin](https://www.maturin.rs/)** for Python bindings
- **[winit](https://github.com/rust-windowing/winit)** + **[softbuffer](https://github.com/rust-windowing/softbuffer)** for interactive display

## Building from Source

Requires Rust and Python 3.9+:

```bash
git clone https://github.com/Thi4gon/rustplot.git
cd rustplot
pip install maturin
maturin develop --release
```

## License

MIT
