# Contributing to RustPlotLib

Thanks for wanting to help rebuild matplotlib in Rust! This guide will help you get started quickly.

## How to Pick a Task

1. Open [ROADMAP.md](ROADMAP.md) — every unchecked `[ ]` item is up for grabs
2. Open a GitHub Issue saying "I'm working on [feature]" to avoid duplicate work
3. Features are organized by priority version (v3, v4, v5, v6)

**Best first contributions:**
- Any item in the **v3.0.0** section (formatters, locators, missing plots)
- Bug fixes and edge cases
- Improving existing stubs (turning them into real implementations)

## Project Structure

```
rustplotlib/
├── src/                          # Rust core (this is where the rendering happens)
│   ├── lib.rs                    # PyO3 module registration
│   ├── figure.rs                 # Figure + PyO3 bindings (savefig, render, etc.)
│   ├── axes.rs                   # Axes (2D plotting area, ~1500 lines)
│   ├── axes3d.rs                 # Axes3D (3D plotting)
│   ├── projection3d.rs           # 3D camera + projection math
│   ├── transforms.rs             # Data coords → pixel coords
│   ├── colors.rs                 # Color parsing (named, hex, RGB, RGBA)
│   ├── text.rs                   # Text rendering (ab_glyph + DejaVu Sans font)
│   ├── ticker.rs                 # Auto-tick computation
│   ├── svg_renderer.rs           # Native SVG output
│   ├── window.rs                 # Interactive window (winit + softbuffer)
│   └── artists/                  # One file per plot type
│       ├── mod.rs                # Artist trait definition
│       ├── line2d.rs             # plot()
│       ├── scatter.rs            # scatter()
│       ├── bar.rs                # bar()
│       ├── hist.rs               # hist()
│       ├── image.rs              # imshow() + colormaps
│       ├── contour.rs            # contour/contourf
│       ├── surface3d.rs          # plot_surface()
│       ├── ... (31 total)
│       └── legend.rs             # Legend rendering
│
├── python/rustplotlib/           # Python API layer
│   ├── __init__.py               # Package exports
│   ├── pyplot.py                 # Main API (drop-in replacement for matplotlib.pyplot)
│   ├── style/__init__.py         # Style themes (dark_background, ggplot, etc.)
│   ├── animation.py              # FuncAnimation + GIF export
│   ├── ticker.py                 # Formatters and Locators
│   ├── dates.py                  # Date handling
│   ├── ... (23 modules total)
│   └── mpl_toolkits/mplot3d/     # 3D axes support
│
├── tests/                        # Python tests
│   ├── test_colors.py
│   ├── test_figure.py
│   ├── test_pyplot.py
│   ├── test_3d.py
│   ├── test_benchmark.py
│   └── ... (200+ tests)
│
├── Cargo.toml                    # Rust dependencies
├── pyproject.toml                # Python package config (maturin)
├── README.md                     # Project documentation
├── ROADMAP.md                    # Feature roadmap with checkboxes
└── CONTRIBUTING.md               # This file
```

## How to Add a New Plot Type

Example: adding `spy()` (sparsity pattern)

### Option A: Pure Python (simple cases)
If the new plot can be built on top of existing Rust functions:

```python
# In python/rustplotlib/pyplot.py, add to AxesProxy:
def spy(self, Z, precision=0, **kwargs):
    import numpy as np
    Z = np.asarray(Z)
    mask = np.abs(Z) > precision
    self.imshow(mask.astype(float), cmap='gray_r', **kwargs)
    self.set_aspect('equal')
```

### Option B: Rust Implementation (performance-critical)
For new rendering that needs to be fast:

**1. Create the artist** (`src/artists/my_plot.rs`):
```rust
use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;

pub struct MyPlot {
    pub data: Vec<f64>,
    pub color: Color,
    pub label: Option<String>,
}

impl Artist for MyPlot {
    fn draw(&self, pixmap: &mut tiny_skia::Pixmap, transform: &Transform) {
        // Draw using tiny-skia
    }
    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        // Return (xmin, xmax, ymin, ymax)
    }
    fn legend_label(&self) -> Option<&str> { self.label.as_deref() }
    fn legend_color(&self) -> Color { self.color }
}
```

**2. Register in** `src/artists/mod.rs`:
```rust
pub mod my_plot;
```

**3. Add method to Axes** (`src/axes.rs`):
```rust
pub fn my_plot(&mut self, data: Vec<f64>, color: Option<Color>) {
    let c = color.unwrap_or_else(|| self.next_color());
    self.artists.push(Box::new(my_plot::MyPlot { data, color: c, label: None }));
}
```

**4. Add PyO3 binding** (`src/figure.rs`):
```rust
fn axes_my_plot(&mut self, ax_id: usize, data: Vec<f64>, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
    let ax = self.axes.get_mut(ax_id).ok_or_else(|| ...)?;
    let color = if let Some(c) = kwargs.get_item("color")? { Some(parse_color_value(&c)?) } else { None };
    ax.my_plot(data, color);
    Ok(())
}
```

**5. Add Python wrapper** (`python/rustplotlib/pyplot.py`):
```python
# In AxesProxy:
def my_plot(self, data, **kwargs):
    self._fig.axes_my_plot(self._id, _to_list(data), kwargs)

# Module level:
def my_plot(data, **kwargs):
    _gca().my_plot(data, **kwargs)
```

**6. Add test** (`tests/test_my_plot.py`):
```python
def test_my_plot():
    import rustplotlib.pyplot as plt
    plt.my_plot([1, 2, 3])
    plt.savefig("/tmp/test_my_plot.png")
    plt.close()
    import os
    assert os.path.exists("/tmp/test_my_plot.png")
    os.remove("/tmp/test_my_plot.png")
```

## Development Setup

```bash
# Clone
git clone https://github.com/Thi4gon/rustplotlib.git
cd rustplotlib

# Setup (requires Rust and Python 3.9+)
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows
pip install maturin numpy pytest

# Build
maturin develop

# Test
pytest tests/ -v

# Build release (for benchmarks)
maturin develop --release
```

## Code Style

- **Rust:** standard rustfmt, no `unsafe`, proper error handling (no `.unwrap()` on user input)
- **Python:** keep it simple, follow existing patterns in pyplot.py
- **Tests:** every new feature needs at least one test
- **Security:** validate all inputs at PyO3 boundary, no path traversal, no panics

## PR Checklist

- [ ] All existing tests pass (`pytest tests/ -v`)
- [ ] New tests added for new features
- [ ] `maturin develop` compiles without errors
- [ ] No new warnings in Rust compilation
- [ ] README.md updated if adding user-facing features
- [ ] ROADMAP.md checkbox checked for completed items
