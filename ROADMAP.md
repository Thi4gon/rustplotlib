# RustPlotLib Roadmap

Goal: Full matplotlib reimplementation in Rust.

---

## DONE — v2.0.0

### 2D Plot Types (26 implemented)
- [x] plot, scatter, bar, barh, hist, imshow, fill_between, fill_betweenx, fill
- [x] errorbar, step, pie, boxplot, violinplot, stem
- [x] contour, contourf, hexbin, quiver, streamplot
- [x] stackplot, broken_barh, eventplot, pcolormesh/pcolor, matshow, sankey

### 3D Plot Types (7 implemented)
- [x] plot3D, scatter3D, plot_surface, plot_wireframe, bar3d, plot_trisurf, contour3D

### Drawing Elements (10 implemented)
- [x] arrow, axhline, axvline, axhspan, axvspan, axline, hlines, vlines, annotate, text

### Rendering
- [x] PNG raster (tiny-skia), Native SVG (vector), PDF, Interactive window, GIF animation

### Customization
- [x] Log scale, twinx/twiny, spines, tick_params, zorder, hatch patterns
- [x] Colorbar, 6 style themes, 70+ colormaps, custom fonts, rcParams (30+ keys)
- [x] bbox_inches='tight', bilinear interpolation, title loc, multi-group plot
- [x] Aspect ratio, invert axes, axis off, subplot_mosaic, suptitle, subplots_adjust

### Compatibility Modules (23)
- [x] pyplot, style, animation, widgets, font_manager, ticker, patches, colors
- [x] dates, gridspec, backends, mpl_toolkits.mplot3d, cm, collections, lines
- [x] text, transforms, patheffects, spines, axes, figure, cycler

### Data Integration
- [x] Pandas, NumPy, NaN handling, dates, categorical axes, imread/imsave

### Security
- [x] Zero unsafe, path validation, dimension limits, no panics on user input

---

## IN PROGRESS — v3.0.0

### Missing Plot Types
- [ ] `spy()` — sparsity pattern visualization
- [ ] `specgram()` — spectrogram
- [ ] `acorr()` / `xcorr()` — auto/cross correlation
- [ ] `angle_spectrum()` / `magnitude_spectrum()` / `phase_spectrum()`
- [ ] `cohere()` — coherence plot
- [ ] `csd()` / `psd()` — cross/power spectral density
- [ ] `ecdf()` — empirical cumulative distribution
- [ ] `stairs()` — step plot with edges
- [ ] `tricontour()` / `tricontourf()` — contour on triangulation
- [ ] `tripcolor()` — pseudocolor on triangulation
- [ ] `triplot()` — plot triangulation edges

### Formatters & Locators (functional, not stubs)
- [ ] `ScalarFormatter` — default number formatting
- [ ] `LogFormatter` / `LogFormatterSciNotation`
- [ ] `EngFormatter` — engineering notation (1k, 1M, 1G)
- [ ] `PercentFormatter`
- [ ] `StrMethodFormatter` / `FuncFormatter`
- [ ] `MaxNLocator` — smart tick placement (partial, have auto_ticks)
- [ ] `MultipleLocator` — ticks at multiples
- [ ] `LogLocator` — logarithmic ticks
- [ ] `FixedLocator` — user-specified positions
- [ ] `AutoMinorLocator` — automatic minor ticks
- [ ] `DateFormatter` functional (format ticks as dates)
- [ ] `DateLocator` functional (place ticks at date intervals)

### Advanced Customization
- [ ] `FancyArrowPatch` — complex arrow styles (arc, angle, etc.)
- [ ] `ConnectionPatch` — arrows connecting different axes
- [ ] Spine positioning (`set_position('center')`, `set_position(('data', 0))`)
- [ ] Minor ticks rendering (major/minor tick distinction)
- [ ] Grid major/minor separate styling
- [ ] `ax.set_xticks(minor=True)` functional
- [ ] Colorbar as separate Axes (not inline drawing)
- [ ] `TwoSlopeNorm`, `CenteredNorm`, `BoundaryNorm` functional

### Image Improvements
- [ ] Bicubic, Lanczos, Spline interpolation
- [ ] Image origin ('upper' vs 'lower')
- [ ] Image extent parameter
- [ ] RGB/RGBA image support in imshow

---

## PLANNED — v4.0.0

### Functional Backends
- [ ] Qt backend (QApplication, mouse events, toolbar, save dialog)
- [ ] GTK3/GTK4 backend
- [ ] Tk backend (tkinter integration)
- [ ] Jupyter inline backend (rich display protocol, `_repr_png_`)
- [ ] WebAgg (HTML5 Canvas, browser-based interactive)
- [ ] macOS native backend (NSView/Metal)

### Functional Widgets
- [ ] `Slider` with real callback and visual rendering
- [ ] `RangeSlider`
- [ ] `Button` with click detection
- [ ] `CheckButtons` / `RadioButtons`
- [ ] `TextBox` with keyboard input
- [ ] `SpanSelector` / `RectangleSelector` / `LassoSelector` functional
- [ ] `Cursor` with crosshair rendering

### Interactive Features
- [ ] Mouse events (click, motion, scroll, pick)
- [ ] Keyboard events
- [ ] Zoom/pan navigation toolbar
- [ ] 3D mouse rotation
- [ ] Blitting for fast animation updates
- [ ] Interactive data cursors

---

## PLANNED — v5.0.0

### Full LaTeX Math Rendering
- [ ] TeX layout engine (fractions, roots, integrals, summation)
- [ ] Subscript/superscript positioning
- [ ] Math font rendering (Computer Modern, STIX)
- [ ] `\frac{}{}`, `\sqrt{}`, `\int`, `\sum`, `\prod`
- [ ] Matrices, arrays, aligned equations
- [ ] `\mathbf{}`, `\mathit{}`, `\mathrm{}`

### Advanced Layout
- [ ] Tight layout engine (constraint solver)
- [ ] Constrained layout
- [ ] GridSpec with rowspan/colspan
- [ ] Figure.add_axes() with custom position [left, bottom, width, height]
- [ ] Axes divider (mpl_toolkits.axes_grid1)
- [ ] `make_axes_locatable` for colorbar positioning
- [ ] Nested subplots

### Transform System
- [ ] Composable transforms (data → axes → figure → display)
- [ ] `ax.transData`, `ax.transAxes`, `fig.transFigure`
- [ ] Blended transforms
- [ ] Custom projections
- [ ] Affine2D functional (rotate, scale, translate)
- [ ] `ax.set_transform()` on artists

---

## PLANNED — v6.0.0

### Geographic/Specialized Projections
- [ ] Polar projection improvements (theta direction, offset)
- [ ] Hammer, Aitoff, Mollweide projections
- [ ] Lambert conformal conic
- [ ] Stereographic projection
- [ ] Basemap-like coastlines and borders (optional data)

### Serialization
- [ ] Pickle save/load figures
- [ ] JSON export/import of figure state
- [ ] Copy to system clipboard (PNG)
- [ ] Multi-page PDF (real, not separate files)

### Full Artist Hierarchy
- [ ] Artist base class with all properties (alpha, clip, zorder, transform, visible, picker, animated, label, url)
- [ ] `set_clip_path()`, `set_clip_box()`
- [ ] `contains()` for hit testing
- [ ] `get_children()`, `findobj()`
- [ ] `draw()` dispatch through artist tree
- [ ] `stale` property for incremental redraw

### Remaining API Surface
- [ ] 100% of `matplotlib.pyplot` functions
- [ ] 100% of `Axes` methods
- [ ] 100% of `Figure` methods
- [ ] Image comparison regression tests vs matplotlib output
- [ ] Full API documentation with examples
- [ ] Type stubs (.pyi) for all modules

---

## Contributing

Pick any unchecked item, open an issue to discuss, submit a PR with tests.

**Highest impact areas:**
1. Formatters/Locators (v3.0.0) — makes axes look professional
2. Jupyter backend (v4.0.0) — huge for data scientists
3. LaTeX rendering (v5.0.0) — needed for scientific papers
4. Interactive features (v4.0.0) — needed for exploratory analysis
