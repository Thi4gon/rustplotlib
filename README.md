# RustPlotLib

**Matplotlib reimplemented in Rust.** A high-performance drop-in replacement for Python's matplotlib, built from scratch with a native Rust rendering engine.

No Python runtime dependency for rendering. No wrappers. No subprocess calls. Pure Rust rasterization exposed to Python via [PyO3](https://pyo3.rs/).

[![CI](https://github.com/Thi4gon/rustplotlib/actions/workflows/ci.yml/badge.svg)](https://github.com/Thi4gon/rustplotlib/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Why RustPlotLib?

| | matplotlib | Other Rust plotting libs | **rustplotlib** |
|---|---|---|---|
| Rendering engine | C/C++ (AGG) | None (wrap matplotlib or call Python) | **Rust native (tiny-skia)** |
| External dependencies | NumPy, Pillow, FreeType, etc. | Python + matplotlib required | **Zero** — self-contained |
| Performance | Baseline | Same or slower (subprocess overhead) | **Up to 30x faster** |
| Python API | Original | Rust-only or generates .py scripts | **Drop-in replacement** — same API |
| Approach | Interpreted + C extensions | Wrappers / code generators | **Full reimplementation in Rust** |

---

## Installation

```bash
pip install rustplotlib
```

Or build from source (requires Rust 1.70+ and Python 3.9+):

```bash
git clone https://github.com/Thi4gon/rustplotlib.git
cd rustplotlib
pip install maturin
maturin develop --release
```

---

## Usage

Just swap your import — everything else stays the same:

```python
# Before:
# import matplotlib.pyplot as plt

# After:
import rustplotlib.pyplot as plt

plt.plot([1, 2, 3, 4], [1, 4, 2, 3], label="data")
plt.title("My Plot")
plt.xlabel("X")
plt.ylabel("Y")
plt.legend()
plt.grid(True)
plt.savefig("plot.png")
plt.show()
```

---

## What's Implemented

### Plot Types
| Function | Description |
|---|---|
| `plot()` | Line plots with color, linestyle, linewidth, markers, markevery, labels, alpha |
| `scatter()` | Scatter plots with per-point sizes, colors, markers, alpha |
| `bar()` / `barh()` | Vertical and horizontal bar charts |
| `hist()` | Histograms with configurable bins |
| `imshow()` | Image/heatmap display with 11 colormaps |
| `fill_between()` | Filled area between two curves |
| `errorbar()` | Error bars with caps (xerr/yerr) |
| `step()` | Step plots (pre/post/mid) |
| `pie()` | Pie charts with labels |
| `boxplot()` | Box-and-whisker plots (Q1/median/Q3, whiskers, outliers) |
| `stem()` | Stem plots with baseline |

### Layout & Figure
| Function | Description |
|---|---|
| `subplots(nrows, ncols)` | Grid of axes with figsize/dpi |
| `figure(figsize, dpi)` | Create new figure |
| `suptitle()` | Figure-level super title |
| `subplots_adjust()` | Control hspace/wspace between subplots |
| `tight_layout()` | Auto-adjust spacing |
| `close()` | Close current figure |

### Axes Customization
| Function | Description |
|---|---|
| `title()` / `set_title()` | Plot title with fontsize |
| `xlabel()` / `ylabel()` | Axis labels with fontsize |
| `set_xlim()` / `set_ylim()` | Axis range limits |
| `set_xscale('log')` / `set_yscale('log')` | Logarithmic scale |
| `set_xticks()` / `set_yticks()` | Custom tick positions |
| `set_xticklabels()` / `set_yticklabels()` | Custom tick labels |
| `set_aspect('equal')` | Equal aspect ratio |
| `invert_xaxis()` / `invert_yaxis()` | Invert axis direction |
| `axis('off')` | Hide axes completely |
| `legend()` | Legend with line+marker swatches and positioning |
| `grid()` | Grid lines with color, linewidth, linestyle, alpha |
| `text()` | Positioned text annotations |
| `annotate()` | Text with arrow pointing to data |
| `axhline()` / `axvline()` | Horizontal/vertical reference lines |

### Output
| Method | Description |
|---|---|
| `savefig("file.png")` | Save as PNG (with dpi, transparent options) |
| `savefig("file.svg")` | Save as SVG |
| `savefig("file.pdf")` | Save as PDF |
| `show()` | Interactive window display |

### Colors
- **Named:** `"red"`, `"blue"`, `"green"`, `"orange"`, `"purple"`, `"black"`, `"white"`, + 10 more
- **Shorthand:** `"r"`, `"g"`, `"b"`, `"c"`, `"m"`, `"y"`, `"k"`, `"w"`
- **Hex:** `"#FF0000"`, `"#f00"`, `"#FF000080"`
- **RGB/RGBA tuples:** `(1.0, 0.0, 0.0)`, `(1.0, 0.0, 0.0, 0.5)`

### Linestyles & Markers
- **Linestyles:** `-` (solid), `--` (dashed), `-.` (dashdot), `:` (dotted)
- **Markers:** `.` `o` `s` `^` `v` `+` `x` `D` `*`
- **Format strings:** `"r--o"` = red + dashed + circle markers
- **markevery:** show marker every N points

### Colormaps
`viridis` `plasma` `inferno` `magma` `hot` `cool` `gray` `jet` `Blues` `Reds` `Greens`

### Text Rendering
- Embedded DejaVu Sans font (no system font dependency)
- LaTeX-to-Unicode conversion (`$\theta$` → θ, `$x_1$` → x₁, Greek letters, sub/superscripts)

### Compatibility
- `rcParams` dict (accepts matplotlib config without crashing)
- `font_manager.FontProperties` (stub)
- `ticker.FormatStrFormatter` (stub)
- `patches.Rectangle` (stub)

---

## Subplots Example

```python
import rustplotlib.pyplot as plt
import numpy as np

fig, axes = plt.subplots(2, 2, figsize=(10, 8))

# Line plot
x = np.linspace(0, 10, 100)
axes[0][0].plot(x, np.sin(x), label="sin(x)")
axes[0][0].plot(x, np.cos(x), label="cos(x)", linestyle="--")
axes[0][0].set_title("Trigonometry")
axes[0][0].legend()
axes[0][0].grid(True)

# Scatter plot
axes[0][1].scatter(np.random.randn(200), np.random.randn(200), alpha=0.5)
axes[0][1].set_title("Random Scatter")

# Bar chart
axes[1][0].bar([1, 2, 3, 4, 5], [3, 7, 2, 5, 8], color="green")
axes[1][0].set_title("Bar Chart")

# Histogram
axes[1][1].hist(np.random.randn(5000), bins=40, color="orange")
axes[1][1].set_title("Distribution")

fig.savefig("subplots.png")
```

---

## Performance Benchmark

Benchmarked against matplotlib on Apple Silicon (M-series). Each test runs 10 iterations, averaged:

| Benchmark | matplotlib | rustplotlib | Speedup |
|---|---|---|---|
| Line Plot (10k points) | 0.064s | 0.002s | **30.8x** |
| Scatter (5k points) | 0.029s | 0.017s | **1.7x** |
| Bar Chart (50 bars) | 0.023s | 0.002s | **9.6x** |
| Histogram (100k points) | 0.081s | 0.003s | **27.9x** |
| Subplots 2x2 | 0.041s | 0.002s | **26.7x** |

Run the benchmark yourself:
```bash
python tests/test_benchmark.py
```

---

## Tech Stack

| Component | Technology | Purpose |
|---|---|---|
| Rendering | [tiny-skia](https://github.com/nickel-org/tiny-skia) | 2D rasterization (paths, shapes, antialiasing) |
| Fonts | [ab_glyph](https://github.com/alexheretic/ab-glyph) | Text rendering with embedded DejaVu Sans |
| PNG output | [png](https://crates.io/crates/png) | PNG encoding |
| Window | [winit](https://github.com/rust-windowing/winit) + [softbuffer](https://github.com/rust-windowing/softbuffer) | Interactive display |
| Python bindings | [PyO3](https://pyo3.rs/) + [maturin](https://www.maturin.rs/) | Rust-to-Python bridge |
| NumPy interop | [numpy](https://crates.io/crates/numpy) (PyO3) | Array conversion |

---

## Architecture

```
Python: import rustplotlib.pyplot as plt
          |
          v
    pyplot.py (thin Python wrapper — matplotlib-compatible API)
          |
          v
    PyO3 bridge (Rust <-> Python)
          |
          v
    Rust core:
      Figure -> Axes -> Artists (Line2D, Scatter, Bar, Hist, Image)
                          |
                          v
                    Transform (data coords -> pixel coords)
                          |
                          v
                    tiny-skia Pixmap (rasterization)
                          |
                    +-----+-----+
                    |     |     |
                   PNG   SVG  Window
```

---

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full development plan. Here's the overview:

| Phase | Version | Focus | Status |
|---|---|---|---|
| MVP | v0.1.0 | plot, scatter, bar, hist, imshow, subplots, PNG/SVG/window | **Done** |
| 1 | v0.2.0 | Log scale, errorbar, fill_between, boxplot, pie, PDF, annotations, stem, barh, step | **In Progress** |
| 2 | v0.3.0 | Contour, streamplot, quiver, polar, hexbin, GridSpec, patches | Planned |
| 3 | v0.4.0 | Styles, rcParams, LaTeX, dark mode, custom fonts | Planned |
| 4 | v0.5.0 | Full 3D (surface, wireframe, scatter3D, bar3d) | Planned |
| 5 | v0.6.0 | Animation, widgets, zoom/pan, events, GIF/MP4 | Planned |
| 6 | v0.7.0 | Backends (Qt, GTK, Tk, Jupyter, Web, Cairo) | Planned |
| 7 | v0.8.0 | Pandas/NumPy integration, dates, categories | Planned |
| 8 | v0.9.0 | Sankey, radar, inset axes, multipage PDF | Planned |
| 9 | v1.0.0 | 100% matplotlib API compatibility | Planned |

---

## Contributing

Contributions are welcome! This is an open-source project under the MIT license.

1. Fork the repo
2. Create a feature branch
3. Open a PR against `master`
4. PRs require at least 1 review before merging

See [ROADMAP.md](ROADMAP.md) for priority areas.

---

## License

[MIT](LICENSE)
