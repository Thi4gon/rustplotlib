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
| Performance | Baseline | Same or slower (subprocess overhead) | **Up to 18.7x faster** |
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

### 2D Plot Types (40+ types)
| Function | Description |
|---|---|
| `plot()` | Line plots with color, linestyle, linewidth, markers, markevery, labels, alpha, zorder |
| `scatter()` | Scatter plots with per-point sizes, colors, markers, alpha, zorder |
| `bar()` / `barh()` | Vertical and horizontal bar charts (stacked, hatch patterns, zorder) |
| `hist()` | Histograms with configurable bins |
| `imshow()` | Image/heatmap display with 70+ colormaps, bilinear interpolation, annotations |
| `fill_between()` / `fill_betweenx()` | Filled area between curves (vertical and horizontal) |
| `fill()` | Filled polygon from vertex arrays |
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
| `stackplot()` | Stacked area charts |
| `broken_barh()` | Broken horizontal bar charts |
| `eventplot()` | Event/raster plots |
| `pcolormesh()` / `pcolor()` | Pseudocolor plots for irregular grids |
| `matshow()` | Matrix display with integer ticks |
| `radar()` | Radar/spider charts |
| `sankey()` | Sankey flow diagrams |
| `spy()` | Sparsity pattern visualization |
| `stairs()` | Step-wise constant function with edges |
| `ecdf()` | Empirical cumulative distribution |
| `triplot()` | Triangulation edge drawing |
| `hist2d()` | 2D histogram heatmap |
| `specgram()` | Spectrogram (STFT-based) |
| `acorr()` / `xcorr()` | Auto/cross correlation |
| `psd()` | Power spectral density (Welch) |
| `magnitude_spectrum()` | FFT magnitude |
| `angle_spectrum()` / `phase_spectrum()` | FFT phase |
| `cohere()` / `csd()` | Coherence / cross spectral density |
| `semilogx()` / `semilogy()` / `loglog()` | Convenience log-scale plots |
| `arrow()` | Arrow drawing |
| `axline()` | Infinite line through point with slope |

### 3D Plot Types (7 types)
| Function | Description |
|---|---|
| `plot()` (3D) | 3D line plots |
| `scatter()` (3D) | 3D scatter with depth sorting |
| `plot_surface()` | 3D surface plots with colormaps |
| `plot_wireframe()` | 3D wireframe plots |
| `bar3d()` | 3D bar charts with shading |
| `plot_trisurf()` | 3D triangulated surface |
| `contour3D()` | 3D contour lines at Z offset |

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
| `subplot2grid()` | Subplot at specific grid position |
| `fig.add_gridspec()` | GridSpec from figure |
| `fig.legend()` | Figure-level legend |

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
| `twinx()` / `twiny()` | Secondary y-axis / x-axis |
| `zorder` | Drawing order control for all artists |
| `hatch` | Hatch patterns for bars (`/`, `\\`, `|`, `-`, `+`, `x`, `o`, `.`, `*`) |
| `legend()` | Legend with line+marker swatches and positioning |
| `grid()` | Grid lines with color, linewidth, linestyle, alpha, which |
| `text()` | Positioned text annotations |
| `annotate()` | Text with arrow pointing to data |
| `table()` | Data table inside axes |
| `axhline()` / `axvline()` | Horizontal/vertical reference lines |
| `axhspan()` / `axvspan()` | Shaded horizontal/vertical regions |
| `hlines()` / `vlines()` | Multiple reference lines with bounds |
| `colorbar()` | Color scale bar for imshow/contour |
| `get_xlim()` / `get_ylim()` | Get current axis limits (functional) |
| `clear()` | Clear all artists from axes |
| `ax.set(**kwargs)` | Set multiple properties at once |
| `minor=True` in set_xticks/yticks | Minor tick support |
| `bar_label()` | Value labels on bars |
| `set_xlabel(color=...)` | Label color customization |
| `title(loc='left')` | Title alignment |

### Output Formats
| Method | Description |
|---|---|
| `savefig("file.png")` | Raster PNG (with dpi, transparent, bbox_inches='tight') |
| `savefig("file.svg")` | Native vector SVG (real `<line>`, `<text>`, `<rect>` elements) |
| `savefig("file.pdf")` | PDF output |
| `savefig("file.eps")` | EPS output |
| `show()` | Interactive window with backend auto-detection |
| `PdfPages` | Multi-page PDF with zlib compression |
| `pickle` | Save/load figures with pickle |
| `copy_to_clipboard()` | Copy figure as PNG to system clipboard |
| `to_json()` / `save_json()` | Export figure state as JSON |

### Backends & Interactive
| Feature | Description |
|---|---|
| Backend auto-detection | Automatically selects inline (Jupyter), Tk, or Agg backend |
| Tk backend | Interactive window with toolbar, zoom/pan, coordinate display |
| WebAgg backend | Browser-based interactive viewer via HTTP server |
| Navigation toolbar | Home, Back, Forward, Pan, Zoom, Save (Tk + WebAgg) |
| Event system | `mpl_connect` / `mpl_disconnect` with `CallbackRegistry` |
| Pick events | Artist hit testing backed by Rust (`hit_test_points`, `hit_test_line`) |
| Mouse/Key/Scroll events | Full event hierarchy: MouseEvent, KeyEvent, PickEvent, DrawEvent, ResizeEvent, CloseEvent |
| Jupyter rich display | `_repr_png_()`, `_repr_svg_()`, `_repr_html_()` on Figure |
| 3D mouse rotation | Drag to rotate 3D view in Tk backend |

### Animation
| Feature | Description |
|---|---|
| `FuncAnimation` | Function-based animation with blit support, pause/resume |
| `ArtistAnimation` | Frame-based animation from artist lists |
| GIF export | Save animations as GIF (via Pillow) |
| Live animation | Real-time animation in Tk backend |

### Layout Engines
| Feature | Description |
|---|---|
| `tight_layout()` | Dynamic margins from text measurement (Rust) |
| `constrained_layout=True` | Constraint-based layout engine (Rust) |
| `GridSpec(nrows, ncols)` | Grid layout with `rowspan`/`colspan` |
| `SubFigure` | Nested sub-figures with independent layouts |
| `Figure.add_axes([l,b,w,h])` | Custom axes positioning |
| `make_axes_locatable` | Axes divider for colorbar positioning |

### Widgets (Rust-rendered)
| Widget | Features |
|---|---|
| `Slider` | Track, thumb circle, label, value display, on_changed callbacks |
| `RangeSlider` | Tuple values, clamp, callbacks |
| `Button` | Background, border, label, on_clicked callbacks |
| `CheckButtons` | Toggle, get_status, callbacks |
| `RadioButtons` | Exclusive selection, value_selected |
| `TextBox` | on_submit, on_text_change, set_val |
| `Cursor` | Crosshair following mouse, on_moved callbacks |
| `MultiCursor` | Cursor spanning multiple axes |

### Transform System
| Component | Description |
|---|---|
| `Affine2D` | 2D affine matrix (rotate, scale, translate, compose, invert) — **Rust** |
| `BlendedTransform` | Separate X/Y transforms — **Rust** |
| `ax.transData` | Data coordinate transform |
| `ax.transAxes` | Axes coordinate transform (0-1) |
| `fig.transFigure` | Figure coordinate transform |

### Styles & Themes
| Feature | Description |
|---|---|
| `style.use('dark_background')` | 13 built-in themes (default, dark_background, ggplot, seaborn, bmh, fivethirtyeight, grayscale, Solarize_Light2, tableau-colorblind10, seaborn-whitegrid, seaborn-darkgrid, seaborn-dark, fast) |
| `style.available` | List all available styles |
| `style.context('ggplot')` | Context manager for temporary style |
| `rcParams` | Functional global configuration (30+ supported keys) |
| `set_facecolor()` | Figure and axes background colors |

### Colors
- **Named:** 140+ colors including all CSS/X11 colors (steelblue, coral, tomato, gold, crimson, dodgerblue, forestgreen, etc.)
- **Shorthand:** `"r"`, `"g"`, `"b"`, `"c"`, `"m"`, `"y"`, `"k"`, `"w"`
- **Hex:** `"#FF0000"`, `"#f00"`, `"#FF000080"`
- **RGB/RGBA tuples:** `(1.0, 0.0, 0.0)`, `(1.0, 0.0, 0.0, 0.5)`
- **Grey/gray variants:** both spellings supported

### Linestyles & Markers
- **Linestyles:** `-` (solid), `--` (dashed), `-.` (dashdot), `:` (dotted)
- **Markers:** `.` `o` `s` `^` `v` `<` `>` `+` `x` `D` `d` `*` `p` `h` `H` `8` `|` `_` `P` `X` `1` `2` `3` `4` (24 types)
- **Format strings:** `"r--o"` = red + dashed + circle markers (parsed in Rust)
- **markevery:** show marker every N points

### Colormaps (81 base + reversed = ~162 total)
`viridis` `plasma` `inferno` `magma` `cividis` `twilight` `twilight_shifted` `turbo` `hot` `cool` `coolwarm` `bwr` `seismic` `gray` `jet` `spring` `summer` `autumn` `winter` `copper` `bone` `pink` `binary` `ocean` `terrain` `rainbow` `Blues` `Reds` `Greens` `Oranges` `Purples` `Greys` `YlOrRd` `YlOrBr` `YlGnBu` `YlGn` `OrRd` `BuGn` `BuPu` `GnBu` `PuBu` `PuRd` `RdPu` `PuBuGn` `RdYlBu` `RdBu` `RdGy` `RdYlGn` `PiYG` `PRGn` `BrBG` `PuOr` `Spectral` `Set1` `Set2` `Set3` `Pastel1` `Pastel2` `tab10` `tab20` `tab20b` `tab20c` `Paired` `Accent` `Dark2` `afmhot` `brg` `CMRmap` `Wistia` `cubehelix` `hsv` `gnuplot` `gnuplot2` `gist_earth` `gist_heat` `gist_ncar` `gist_rainbow` `gist_stern` `gist_yarg` `flag` `prism`

All also available reversed with `_r` suffix.

### Image Interpolation (15 modes)
`nearest` `bilinear` `bicubic` `lanczos` `spline16` `hanning` `hamming` `hermite` `kaiser` `quadric` `catrom` `gaussian` `bessel` `mitchell` `sinc`

### Text & Math Rendering
- Embedded DejaVu Sans font (no system font dependency)
- **Full TeX math engine in Rust:**
  - Greek letters (24 lowercase + 12 uppercase)
  - Subscript/superscript with Unicode characters
  - Stacked fractions (`\frac{a}{b}`) with horizontal bar
  - Square roots (`\sqrt{x}`) with radical sign
  - Integral/sum/product with limits (`\int_a^b`, `\sum_{i=0}^n`)
  - Accents: `\hat`, `\tilde`, `\vec`, `\dot`, `\ddot`, `\overline`, `\underline`
  - Delimiters: `\left(`, `\right)`, `\langle`, `\rangle`, `\lceil`, `\lfloor`
  - Matrices with bracket rendering
  - 60+ math operators and symbols

### Geographic Projections (5 types)
| Projection | Function |
|---|---|
| Hammer | `hammer_project(lon, lat)` |
| Aitoff | `aitoff_project(lon, lat)` |
| Mollweide | `mollweide_project(lon, lat)` |
| Lambert Conformal Conic | `lambert_project(lon, lat, lat1, lat2)` |
| Stereographic | `stereographic_project(lon, lat, lon0, lat0)` |

All with batch versions and `generate_graticule()` for grid lines.

### Data Integration
- **Pandas:** plot directly from DataFrame/Series (optional dependency)
- **NumPy:** full array support
- **NaN handling:** automatic gaps in line plots for NaN/Inf values
- **Dates:** `date2num()`, `num2date()`, date formatters and locators
- **Categorical axes:** string-based x values automatically converted

### Compatibility Modules (25 modules)
| Module | Status |
|---|---|
| `rustplotlib.pyplot` | Full implementation (50+ functions) |
| `rustplotlib.style` | Full implementation (6 themes) |
| `rustplotlib.animation` | FuncAnimation + GIF export |
| `rustplotlib.widgets` | Functional (Slider, Button, CheckButtons, RadioButtons, TextBox, RangeSlider, Cursor) |
| `rustplotlib.font_manager` | FontProperties |
| `rustplotlib.ticker` | 12 Formatters + 10 Locators (functional) |
| `rustplotlib.patches` | Rectangle, Circle, Polygon, FancyBboxPatch, Wedge, FancyArrowPatch |
| `rustplotlib.colors` | LinearSegmentedColormap, Normalize, LogNorm, BoundaryNorm |
| `rustplotlib.dates` | Date conversion, DateFormatter, DateLocator, Auto/Day/Month/Year/Hour/MinuteLocator |
| `rustplotlib.gridspec` | GridSpec, SubplotSpec |
| `rustplotlib.backends` | Backend system with auto-detection (inline/tk/agg), PdfPages |
| `rustplotlib.backends.backend_base` | FigureCanvasBase, FigureManagerBase, NavigationToolbar2 |
| `rustplotlib.backends.backend_tk` | Tk interactive window with toolbar and events |
| `rustplotlib.mpl_toolkits.mplot3d` | Axes3D for 3D plotting |
| `rustplotlib.cm` | Colormap access by name |
| `rustplotlib.collections` | LineCollection, PathCollection, PatchCollection |
| `rustplotlib.lines` | Line2D with get/set methods |
| `rustplotlib.text` | Text, Annotation |
| `rustplotlib.transforms` | Bbox, Affine2D, BboxTransform |
| `rustplotlib.patheffects` | Stroke, withStroke, SimplePatchShadow |
| `rustplotlib.spines` | Spine |
| `rustplotlib.axes` | Axes class reference |
| `rustplotlib.figure` | Figure class with Jupyter rich display (`_repr_png_`, `_repr_svg_`, `_repr_html_`) |
| `rustplotlib.events` | MouseEvent, KeyEvent, DrawEvent, ResizeEvent, CloseEvent |
| `rustplotlib.callback_registry` | CallbackRegistry for mpl_connect/mpl_disconnect |
| `rustplotlib.cycler` | cycler compatibility |

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

Benchmarked against matplotlib on Apple Silicon (M-series, release build). Each test runs 10 iterations, averaged:

| Benchmark | matplotlib | rustplotlib | Speedup |
|---|---|---|---|
| Line Plot (10k pts) | 0.024s | 0.005s | **4.7x** |
| Scatter (5k pts) | 0.027s | 0.020s | **1.3x** |
| Bar Chart (50 bars) | 0.024s | 0.004s | **5.7x** |
| Histogram (100k pts) | 0.087s | 0.005s | **18.7x** |
| Subplots 2x2 | 0.038s | 0.010s | **3.9x** |
| Heatmap (100x100) | 0.019s | 0.005s | **3.7x** |
| Large Line (100k pts) | 0.109s | 0.110s | **1.0x** |
| Multi-line (20 lines) | 0.085s | 0.029s | **2.9x** |
| Error Bars | 0.021s | 0.004s | **5.9x** |
| Pie Chart | 0.009s | 0.004s | **2.4x** |
| SVG Output | 0.021s | 0.003s | **7.6x** |
| Full Styled Plot | 0.018s | 0.005s | **3.9x** |
| **TOTAL** | **0.481s** | **0.203s** | **2.4x** |

> Wins 11 out of 12 benchmarks. Up to **18.7x faster** on histogram rendering.

Run the benchmark yourself:
```bash
maturin develop --release
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

**Project stats:**
- **53 Rust source files** — 30,000+ lines of native code
- **25+ Python modules** — thin PyO3 wrappers
- **50 plot types** (43 2D + 7 3D) — 100% of matplotlib plot types
- **81 colormaps** + reversed = ~162 total (~95% of matplotlib)
- **550+ tests** passing (including 27 rendering regression tests)
- **110/110 pyplot functions**, 95/95 Axes methods, 36/36 Figure methods
- **15 interpolation modes**: nearest, bilinear, bicubic, lanczos, spline16, hanning, hamming, hermite, kaiser, quadric, catrom, gaussian, bessel, mitchell, sinc
- **5 geographic projections**: Hammer, Aitoff, Mollweide, Lambert, Stereographic
- **4 backends**: Agg, Tk (interactive), Jupyter inline, WebAgg (browser)
- **4 output formats**: PNG, SVG, PDF (multi-page), EPS
- **Full TeX math engine**: Greek, sub/superscript, fractions, sqrt, matrices
- **Full transform system**: Affine2D, BlendedTransform, transData/transAxes
- **Full event system**: mouse, keyboard, pick events with Rust hit testing
- **Widgets**: Slider, Button, CheckButtons, RadioButtons, TextBox (Rust rendering)
- **Layout engines**: tight layout, constrained layout, GridSpec rowspan/colspan, SubFigure
- **Zero `unsafe` blocks**

**Remaining (need external dependencies):**
- Qt/GTK/macOS backends (need PyQt5, PyGObject, PyObjC)
- Math font rendering (need Computer Modern .ttf files)
- Basemap coastlines (need Natural Earth shapefiles)

---

## License

[MIT](LICENSE)
