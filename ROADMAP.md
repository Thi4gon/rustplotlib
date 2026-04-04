# RustPlotLib Roadmap

Goal: Full matplotlib reimplementation in Rust.

---

## DONE — v4.0.0

### 2D Plot Types (40+ implemented)
- [x] plot, scatter, bar, barh, hist, imshow (RGB/RGBA + bilinear), fill_between, fill_betweenx, fill
- [x] errorbar, step, pie, boxplot, violinplot, stem
- [x] contour, contourf, hexbin, quiver, streamplot
- [x] stackplot, broken_barh, eventplot, pcolormesh/pcolor, matshow, sankey
- [x] spy, stairs, ecdf, triplot, hist2d, arrow, axline
- [x] specgram, acorr, xcorr, psd, magnitude_spectrum, angle_spectrum, phase_spectrum
- [x] cohere, csd, semilogx, semilogy, loglog, radar

### 3D Plot Types (7 implemented)
- [x] plot3D, scatter3D, plot_surface, plot_wireframe, bar3d, plot_trisurf, contour3D

### Drawing Elements (12 implemented)
- [x] arrow, axhline, axvline, axhspan, axvspan, axline, hlines, vlines, annotate, text, table, colorbar

### Rendering
- [x] PNG raster (tiny-skia), Native SVG (vector), PDF, Interactive window, GIF animation
- [x] bbox_inches='tight' (real whitespace cropping)

### Customization
- [x] Log scale, twinx/twiny, spines, tick_params, zorder, hatch patterns
- [x] 6 style themes, 70+ colormaps (+ reversed), custom fonts, rcParams (30+ keys)
- [x] Bilinear interpolation, title loc, multi-group plot, image extent, RGB/RGBA imshow
- [x] Aspect ratio, invert axes, axis off, subplot_mosaic, suptitle, subplots_adjust
- [x] Minor ticks rendering + `set_xticks(minor=True)` functional
- [x] Label colors, get_xlim/get_ylim (functional), axes clear
- [x] Image origin ('upper' / 'lower')

### Formatters & Locators (22 implemented)
- [x] ScalarFormatter, LogFormatter, LogFormatterSciNotation, LogFormatterMathtext
- [x] EngFormatter, PercentFormatter, StrMethodFormatter, FuncFormatter, FormatStrFormatter
- [x] MaxNLocator, AutoLocator, MultipleLocator, LogLocator, FixedLocator, AutoMinorLocator
- [x] DateFormatter, AutoDateFormatter, DateLocator, AutoDateLocator
- [x] DayLocator, MonthLocator, YearLocator, HourLocator, MinuteLocator

### Compatibility Modules (23)
- [x] pyplot (50+ functions), style, animation, widgets, font_manager, ticker, patches, colors
- [x] dates, gridspec, backends, mpl_toolkits.mplot3d, cm, collections, lines
- [x] text, transforms, patheffects, spines, axes, figure, cycler

### Data Integration
- [x] Pandas, NumPy, NaN handling, dates, categorical axes, imread/imsave, Jupyter inline

### Security
- [x] Zero unsafe, path validation, dimension limits, no panics on user input

---

## PLANNED — v5.0.0

### Remaining Plot Types
- [ ] `tricontour()` / `tricontourf()` — contour on triangulation (currently stubs)
- [ ] `tripcolor()` — pseudocolor on triangulation (currently stub)

### Advanced Customization
- [ ] `FancyArrowPatch` — complex arrow styles (arc, angle, etc.) — currently basic stub
- [ ] `ConnectionPatch` — arrows connecting different axes — currently basic stub
- [ ] Spine positioning (`set_position('center')`, `set_position(('data', 0))`)
- [ ] Grid major/minor separate styling
- [ ] Colorbar as separate Axes (not inline drawing)
- [ ] `TwoSlopeNorm`, `CenteredNorm` functional

### Image Improvements
- [ ] Bicubic, Lanczos, Spline interpolation

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

## PLANNED — v6.0.0

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

## PLANNED — v7.0.0

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
1. Jupyter backend (v5.0.0) — huge for data scientists
2. Interactive features (v5.0.0) — needed for exploratory analysis
3. LaTeX rendering (v6.0.0) — needed for scientific papers
4. Triangulation plots (v5.0.0) — tricontour, tripcolor
