# RustPlotLib

**Matplotlib reimplemented in Rust.** A high-performance drop-in replacement for Python's matplotlib, built from scratch with a native Rust rendering engine.

No Python runtime dependency for rendering. No wrappers. No subprocess calls. Pure Rust rasterization exposed to Python via [PyO3](https://pyo3.rs/).

[![CI](https://github.com/Thi4gon/rustplotlib/actions/workflows/ci.yml/badge.svg)](https://github.com/Thi4gon/rustplotlib/actions)
[![PyPI](https://img.shields.io/pypi/v/rustplotlib.svg)](https://pypi.org/project/rustplotlib/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Python](https://img.shields.io/pypi/pyversions/rustplotlib.svg)](https://pypi.org/project/rustplotlib/)

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

Supports Python 3.9-3.13 on macOS, Linux, and Windows (pre-built wheels available).

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

### 2D Plot Types (17 types)
| Function | Description |
|---|---|
| `plot()` | Line plots with color, linestyle, linewidth, markers, markevery, labels, alpha |
| `scatter()` | Scatter plots with per-point sizes, colors, markers, alpha |
| `bar()` / `barh()` | Vertical and horizontal bar charts (stacked via `bottom` param) |
| `hist()` | Histograms with configurable bins |
| `imshow()` | Image/heatmap display with 35+ colormaps |
| `fill_between()` / `fill_betweenx()` | Filled area between curves (vertical and horizontal) |
| `errorbar()` | Error bars with caps (xerr/yerr) |
| `step()` | Step plots (pre/post/mid) |
| `pie()` | Pie charts with labels |
| `boxplot()` | Box-and-whisker plots (Q1/median/Q3, whiskers, outliers) |
| `violinplot()` | Violin plots with Gaussian KDE |
| `stem()` | Stem plots with baseline |
| `contour()` / `contourf()` | Contour lines and filled contours (marching squares) |
| `hexbin()` | Hexagonal binning for 2D histograms |
| `quiver()` | Vector field arrows |
| `streamplot()` | Streamlines for vector fields (Euler integration) |

### 3D Plot Types (5 types)
| Function | Description |
|---|---|
| `plot()` (3D) | 3D line plots |
| `scatter()` (3D) | 3D scatter with depth sorting |
| `plot_surface()` | 3D surface plots with colormaps |
| `plot_wireframe()` | 3D wireframe plots |
| `bar3d()` | 3D bar charts with shading |

### Layout & Figure
| Function | Description |
|---|---|
| `subplots(nrows, ncols)` | Grid of axes with figsize/dpi |
| `subplot(nrows, ncols, index)` | Add single subplot |
| `subplot_mosaic()` | Named subplot layouts from ASCII art |
| `figure(figsize, dpi)` | Create new figure |
| `suptitle()` | Figure-level super title |
| `subplots_adjust()` | Control hspace/wspace between subplots |
| `tight_layout()` | Auto-adjust spacing |
| `add_subplot(projection='3d')` | Add 3D subplot |
| `clf()` / `cla()` / `close()` | Clear and close figures |
| `gcf()` / `gca()` | Get current figure/axes |

### Axes Customization
| Function | Description |
|---|---|
| `title()` / `set_title()` | Plot title with fontsize |
| `xlabel()` / `ylabel()` | Axis labels with fontsize |
| `set_xlim()` / `set_ylim()` | Axis range limits |
| `set_xscale('log')` / `set_yscale('log')` | Logarithmic scale |
| `set_xticks()` / `set_yticks()` | Custom tick positions |
| `set_xticklabels()` / `set_yticklabels()` | Custom tick labels |
| `tick_params()` | Tick direction, length, width, labelsize |
| `set_aspect('equal')` | Equal aspect ratio |
| `invert_xaxis()` / `invert_yaxis()` | Invert axis direction |
| `axis('off')` | Hide axes completely |
| `set_facecolor()` | Axes background color |
| `spines['right'].set_visible(False)` | Spine customization |
| `twinx()` | Secondary y-axis |
| `legend()` | Legend with line+marker swatches and positioning |
| `grid()` | Grid lines with color, linewidth, linestyle, alpha, which |
| `text()` | Positioned text annotations |
| `annotate()` | Text with arrow pointing to data |
| `table()` | Data table inside axes |
| `axhline()` / `axvline()` | Horizontal/vertical reference lines |
| `axhspan()` / `axvspan()` | Shaded horizontal/vertical regions |
| `hlines()` / `vlines()` | Multiple reference lines with bounds |
| `colorbar()` | Color scale bar for imshow/contour |

### Output Formats
| Method | Description |
|---|---|
| `savefig("file.png")` | Raster PNG (with dpi, transparent options) |
| `savefig("file.svg")` | Native vector SVG (real `<line>`, `<text>`, `<rect>` elements) |
| `savefig("file.pdf")` | PDF output |
| `show()` | Interactive window display |
| `PdfPages` | Multi-page PDF export |

### Animation
| Feature | Description |
|---|---|
| `FuncAnimation` | Function-based animation with frame generation |
| GIF export | Save animations as GIF (via Pillow) |
| PNG sequence | Save animation frames as individual PNGs |

### Styles & Themes
| Feature | Description |
|---|---|
| `style.use('dark_background')` | 6 built-in themes (default, dark_background, ggplot, seaborn, bmh, fivethirtyeight) |
| `rcParams` | Functional global configuration (30+ supported keys) |
| `set_facecolor()` | Figure and axes background colors |

### Colors
- **Named:** 17 colors (red, blue, green, orange, purple, black, white, cyan, magenta, yellow, brown, pink, gray, olive, navy, teal, lime)
- **Shorthand:** `"r"`, `"g"`, `"b"`, `"c"`, `"m"`, `"y"`, `"k"`, `"w"`
- **Hex:** `"#FF0000"`, `"#f00"`, `"#FF000080"`
- **RGB/RGBA tuples:** `(1.0, 0.0, 0.0)`, `(1.0, 0.0, 0.0, 0.5)`

### Linestyles & Markers
- **Linestyles:** `-` (solid), `--` (dashed), `-.` (dashdot), `:` (dotted)
- **Markers:** `.` `o` `s` `^` `v` `+` `x` `D` `*`
- **Format strings:** `"r--o"` = red + dashed + circle markers
- **markevery:** show marker every N points

### Colormaps (35+)
`viridis` `plasma` `inferno` `magma` `cividis` `twilight` `turbo` `hot` `cool` `gray` `jet` `spring` `summer` `autumn` `winter` `copper` `bone` `pink` `binary` `gist_heat` `ocean` `terrain` `Blues` `Reds` `Greens` `YlOrRd` `YlGnBu` `RdYlBu` `RdBu` `PiYG` `PRGn` `BrBG` `Spectral` `Set1` `Set2` `Set3` `Pastel1` `Pastel2` `tab20`

### Text Rendering
- Embedded DejaVu Sans font (no system font dependency)
- LaTeX-to-Unicode conversion (`$\theta$` -> theta, `$x_1$` -> x1, Greek letters, sub/superscripts, math operators)

### Data Integration
- **Pandas:** plot directly from DataFrame/Series (optional dependency)
- **NumPy:** full array support
- **NaN handling:** automatic gaps in line plots for NaN/Inf values
- **Dates:** `date2num()`, `num2date()`, date formatters and locators
- **Categorical axes:** string-based x values automatically converted

### Compatibility Modules
| Module | Status |
|---|---|
| `rustplotlib.pyplot` | Full implementation |
| `rustplotlib.style` | Full implementation (6 themes) |
| `rustplotlib.animation` | FuncAnimation + GIF export |
| `rustplotlib.widgets` | Stubs (Slider, Button, CheckButtons, RadioButtons, TextBox, Cursor) |
| `rustplotlib.font_manager` | FontProperties stub |
| `rustplotlib.ticker` | FormatStrFormatter stub |
| `rustplotlib.patches` | Rectangle, Circle, Polygon, FancyBboxPatch, Wedge |
| `rustplotlib.colors` | LinearSegmentedColormap, Normalize, LogNorm |
| `rustplotlib.dates` | Date conversion, formatters, locators |
| `rustplotlib.gridspec` | GridSpec, SubplotSpec |
| `rustplotlib.backends` | Backend system with `use()` |
| `rustplotlib.mpl_toolkits.mplot3d` | Axes3D for 3D plotting |

---

## Examples

### Subplots

```python
import rustplotlib.pyplot as plt
import numpy as np

fig, axes = plt.subplots(2, 2, figsize=(10, 8))

x = np.linspace(0, 10, 100)
axes[0][0].plot(x, np.sin(x), label="sin(x)")
axes[0][0].plot(x, np.cos(x), label="cos(x)", linestyle="--")
axes[0][0].set_title("Trigonometry")
axes[0][0].legend()
axes[0][0].grid(True)

axes[0][1].scatter(np.random.randn(200), np.random.randn(200), alpha=0.5)
axes[0][1].set_title("Random Scatter")

axes[1][0].bar([1, 2, 3, 4, 5], [3, 7, 2, 5, 8], color="green")
axes[1][0].set_title("Bar Chart")

axes[1][1].hist(np.random.randn(5000), bins=40, color="orange")
axes[1][1].set_title("Distribution")

fig.savefig("subplots.png")
```

### 3D Surface Plot

```python
import rustplotlib.pyplot as plt
import numpy as np

fig = plt.figure(figsize=(10, 8))
ax = fig.add_subplot(111, projection='3d')

X = np.linspace(-5, 5, 50)
Y = np.linspace(-5, 5, 50)
X, Y = np.meshgrid(X, Y)
Z = np.sin(np.sqrt(X**2 + Y**2))

ax.plot_surface(X.tolist(), Y.tolist(), Z.tolist(), cmap='viridis')
ax.set_xlabel("X")
ax.set_ylabel("Y")
ax.set_zlabel("Z")
plt.savefig("surface3d.png")
```

### Dark Mode

```python
import rustplotlib.pyplot as plt
import rustplotlib.style as style

style.use('dark_background')
plt.plot([1, 2, 3, 4], [1, 4, 2, 3], 'c-o', linewidth=2)
plt.title("Dark Mode")
plt.savefig("dark.png")
```

### Animation

```python
import rustplotlib.pyplot as plt
from rustplotlib.animation import FuncAnimation
import numpy as np

fig, ax = plt.subplots()
x = np.linspace(0, 2 * np.pi, 100)

def update(frame):
    ax.plot(x, np.sin(x + frame * 0.1), color='blue')
    ax.set_title(f"Frame {frame}")

anim = FuncAnimation(fig, update, frames=50)
anim.save("wave.gif")
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
| 2D Rendering | [tiny-skia](https://github.com/nickel-org/tiny-skia) | Rasterization (paths, shapes, antialiasing) |
| SVG Rendering | Custom SVG renderer | Native vector SVG output |
| Fonts | [ab_glyph](https://github.com/alexheretic/ab-glyph) | Text rendering with embedded DejaVu Sans |
| PNG output | [png](https://crates.io/crates/png) | PNG encoding |
| Window | [winit](https://github.com/rust-windowing/winit) + [softbuffer](https://github.com/rust-windowing/softbuffer) | Interactive display |
| 3D Projection | Custom | Orthographic 3D-to-2D with camera control |
| Python bindings | [PyO3](https://pyo3.rs/) + [maturin](https://www.maturin.rs/) | Rust-to-Python bridge |
| NumPy interop | [numpy](https://crates.io/crates/numpy) (PyO3) | Array conversion |

---

## Architecture

```
Python: import rustplotlib.pyplot as plt
          |
          v
    pyplot.py (matplotlib-compatible API layer)
          |
          v
    PyO3 bridge (Rust <-> Python)
          |
          v
    Rust core:
      Figure --> Axes (2D) ---------> Artists (Line2D, Scatter, Bar, ...)
             |                              |
             --> Axes3D -----------> Artists3D (Line3D, Surface3D, ...)
                                           |
                                           v
                                    Transform / Camera
                                           |
                              +------------+------------+
                              |            |            |
                         tiny-skia    SvgRenderer    PDF gen
                         (raster)     (vector)      (document)
                              |            |            |
                             PNG          SVG          PDF
```

---

## Security

- **Zero `unsafe` blocks** in the entire Rust codebase
- Path validation in `savefig()` (extension whitelist, path traversal rejection)
- Dimension validation (max 32768x32768 pixels)
- Unique temp file names (no symlink race conditions)
- Input validation on all PyO3 boundaries
- No `.unwrap()` on user-controlled data — proper error handling throughout

---

## Contributing

Contributions are welcome! This is an open-source project under the MIT license.

1. Fork the repo
2. Create a feature branch
3. Open a PR against `master`
4. PRs require at least 1 review before merging

**Priority areas for contribution:**
- Turning stub modules into full implementations (widgets, formatters/locators)
- Adding more colormaps with exact matplotlib data
- Improving SVG output fidelity
- Jupyter inline display support
- More comprehensive test coverage

---

## License

[MIT](LICENSE)
