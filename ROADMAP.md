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

## DONE — v5.0.0 (Phase 1 + Phase 2)

### Backend System
- [x] `backend_base.py` — `FigureCanvasBase`, `FigureManagerBase`, `NavigationToolbar2`
- [x] `backends/__init__.py` — auto-detection (inline/tk/agg), registry, `show_figure()`
- [x] `switch_backend()` / `set_backend()` functional

### Tk Interactive Backend
- [x] `backend_tk.py` — `FigureCanvasTk`, `FigureManagerTk`, `NavigationToolbarTk`
- [x] Tkinter window with PhotoImage rendering
- [x] Mouse events (click, release, motion, scroll)
- [x] Keyboard events
- [x] Navigation toolbar (Home, Back, Fwd, Pan, Zoom, Save)
- [x] Coordinate display on mouse motion
- [x] `show()` and `FigureProxy.show()` delegate to backend system

### Jupyter Rich Display
- [x] `render_to_svg_string()` exposed via PyO3
- [x] `render_to_rgba_buffer()` exposed via PyO3
- [x] `_repr_png_`, `_repr_svg_`, `_repr_html_` on FigureProxy
- [x] `backend_inline.py` rewritten with SVG support and configurable `figure_format`

### Event System
- [x] Event classes: `Event`, `LocationEvent`, `MouseEvent`, `KeyEvent`, `PickEvent`, `DrawEvent`, `ResizeEvent`, `CloseEvent`
- [x] `CallbackRegistry` with `connect(signal, func)`, `disconnect(cid)`, `process(signal, *args)`
- [x] `CanvasProxy.mpl_connect()` / `mpl_disconnect()` functional
- [x] `events` and `callback_registry` modules registered in package

### Zoom/Pan Navigation
- [x] `pixel_to_data()` coordinate mapping on `FigureCanvasTk`
- [x] Functional zoom — rubber-band selection sets xlim/ylim and re-renders
- [x] Functional pan — drag translates data limits with live update
- [x] Home/Back/Forward view stack navigation
- [x] Status bar shows data coordinates on mouse motion

### Functional Widgets
- [x] `Slider` — `on_changed()`, `set_val()`, `disconnect()`, valstep, clamp, inactive mode
- [x] `RangeSlider` — tuple values, clamp, callbacks
- [x] `Button` — `on_clicked()`, `click()` programmatic, inactive mode
- [x] `CheckButtons` — toggle, `get_status()`, callbacks
- [x] `RadioButtons` — exclusive selection, `value_selected`, callbacks
- [x] `TextBox` — `on_submit()`, `on_text_change()`, `set_val()`
- [x] `SpanSelector`, `RectangleSelector`, `LassoSelector` — interfaces with `onselect`
- [x] `Cursor` — interface with horizOn/vertOn

### Compatibility Modules (25 total)
- [x] Added: events, callback_registry (+ backends expanded with backend_base, backend_tk, backend_inline)

---

## PLANNED — v5.1.0

### Remaining Plot Types
- [x] `tricontour()` / `tricontourf()` — contour on triangulation (barycentric interpolation)
- [x] `tripcolor()` — pseudocolor on triangulation

### Advanced Customization
- [ ] `FancyArrowPatch` — complex arrow styles (arc, angle, etc.) — currently basic stub
- [ ] `ConnectionPatch` — arrows connecting different axes — currently basic stub
- [ ] Spine positioning (`set_position('center')`, `set_position(('data', 0))`)
- [ ] Grid major/minor separate styling
- [ ] Colorbar as separate Axes (not inline drawing)
- [ ] `TwoSlopeNorm`, `CenteredNorm` functional

### Image Improvements
- [ ] Bicubic, Lanczos, Spline interpolation

### Additional Backends
- [ ] Qt backend (QApplication, mouse events, toolbar, save dialog)
- [ ] GTK3/GTK4 backend
- [ ] WebAgg (HTML5 Canvas, browser-based interactive)
- [ ] macOS native backend (NSView/Metal)

### Interactive Features (remaining)
- [ ] Widget visual rendering in Tk (Slider/Button draw in axes)
- [ ] Pick events (artist hit testing)
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
1. Qt/GTK backends (v5.1.0) — needed for desktop applications
2. LaTeX rendering (v6.0.0) — needed for scientific papers
3. Pick events / 3D rotation (v5.1.0) — interactive artist selection
