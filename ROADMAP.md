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
- [x] `FancyArrowPatch` — 7 arrow styles (->, -|>, <-, <->, wedge, -[, |-) + arc3 connection style
- [x] `ConnectionPatch` — arrows connecting different axes (functional via add_patch)
- [x] Spine positioning (`set_position('center')`, `set_position(('data', 0))`)
- [x] Grid major/minor separate styling
- [x] Colorbar as separate Axes (ColorbarArtist in Rust)
- [x] `TwoSlopeNorm`, `CenteredNorm` functional
- [x] Path effects — `withStroke`, `Stroke`, `Normal`, `SimplePatchShadow`, `SimpleLineShadow`
- [x] Pick events — artist hit testing with `set_picker()`, `PickEvent` via `mpl_connect`

### Image Improvements
- [x] Bicubic interpolation (Keys cubic kernel, scalar data)
- [x] Lanczos interpolation (sinc-based kernel, a=3)
- [x] Spline16 (B-spline) interpolation

### Additional Backends
- [ ] Qt backend — **Needs PyQt5/PySide2.** Implement `FigureCanvasQt`, `FigureManagerQt` in `backends/backend_qt.py`. Follow `backend_tk.py` pattern. Use `QApplication`, `QLabel` for image display, `QMouseEvent` mapping. Register in `backends/__init__.py`.
- [ ] GTK3/GTK4 backend — **Needs PyGObject (gi).** Implement `FigureCanvasGTK`, `FigureManagerGTK` in `backends/backend_gtk.py`. Use `Gtk.DrawingArea` with Cairo or direct image. Follow Tk pattern.
- [x] WebAgg (HTML5 Canvas, browser-based interactive via HTTP)
- [ ] macOS native backend — **Needs PyObjC.** Use `NSView` + `NSImage` for display. Most complex backend. Consider wrapping Rust `winit` instead.

### Interactive Features (remaining)
- [x] Widget visual rendering (Slider/Button artists in Rust)
- [x] Pick events (artist hit testing)
- [x] 3D mouse rotation (drag to rotate azim/elev in Tk backend)
- [x] Blitting for fast animation updates (FuncAnimation blit=True, pause/resume)
- [x] Interactive data cursors (Cursor + MultiCursor widgets)

---

## PLANNED — v6.0.0

### Full LaTeX Math Rendering
- [x] Basic math text: Greek letters (24+12), sub/superscript (Unicode), operators (±×÷·≤≥≠≈∞∫Σ√)
- [x] `\frac{}{}` → fraction, `\mathbf{}`, `\text{}` pass-through
- [x] Full TeX layout engine (stacked fractions, sqrt with bar, integral/sum limits)
- [ ] Math font rendering — **Needs .ttf font files.** Embed Computer Modern or STIX fonts in Rust binary (like DejaVu Sans is already embedded in `text.rs`). Modify `mathtext.rs` to use math-specific font for rendered glyphs. Font files: ~2MB. Get from [STIX fonts](https://github.com/stipub/stixfonts) or [Computer Modern Unicode](https://sourceforge.net/projects/cm-unicode/).
- [x] Matrices rendering (parse_matrix, render_matrix with brackets)

### Advanced Layout
- [x] Tight layout engine (Rust dynamic margins from text measurement)
- [x] Constrained layout (Rust dynamic margins)
- [x] GridSpec with rowspan/colspan (Rust grid_span)
- [x] Figure.add_axes() with custom position [left, bottom, width, height]
- [x] Axes divider (mpl_toolkits.axes_grid1)
- [x] `make_axes_locatable` for colorbar positioning
- [x] Nested subplots (SubFigure with subfigures())

### Transform System
- [x] Composable Affine2D transforms in Rust (rotate, scale, translate, compose, invert)
- [x] `ax.transData`, `ax.transAxes`, `fig.transFigure` (functional)
- [x] Blended transforms (BlendedTransform in Rust)
- [x] Custom projections (Hammer, Aitoff, Mollweide, Lambert, Stereographic)
- [x] `ax.set_transform()` on artists (ArtistBase)

---

## PLANNED — v7.0.0

### Geographic/Specialized Projections
- [x] Polar projection improvements (theta direction, offset, set_rmax, set_rticks)
- [x] Hammer, Aitoff, Mollweide projections (Rust with batch + graticule)
- [x] Lambert conformal conic (Rust)
- [x] Stereographic projection (Rust)
- [ ] Basemap-like coastlines and borders — **Needs geographic data.** Download [Natural Earth](https://www.naturalearthdata.com/) shapefiles (~10MB). Parse with a Rust shapefile reader crate or pre-process to JSON coordinate arrays. Store as optional data package. Render coastlines as LineCollection on projected axes.

### Serialization
- [x] Pickle save/load figures
- [x] JSON export of figure state (figure_to_json in Rust)
- [x] Copy to system clipboard (PNG — macOS/Linux/Windows)
- [x] Multi-page PDF (PdfPages with zlib compression)

### Full Artist Hierarchy
- [x] Artist base class with all properties (ArtistBase: alpha, clip, zorder, transform, visible, picker, animated, label, url)
- [x] `set_clip_path()`, `set_clip_box()` 
- [x] `contains()` for hit testing (backed by Rust hit_test)
- [x] `get_children()`, `findobj()`
- [x] `draw()` dispatch through artist tree
- [x] `stale` property for incremental redraw

### Remaining API Surface
- [x] 100% of `matplotlib.pyplot` functions (110/110)
- [x] 100% of `Axes` methods (95/95)
- [x] 100% of `Figure` methods
- [x] Image regression tests (27 rendering tests: all plot types, output formats, features)
- [x] Full API documentation (type stubs .pyi, docstrings, CLAUDE.md)
- [x] All 15 image interpolation modes (hanning, hamming, hermite, kaiser, quadric, catrom, gaussian, bessel, mitchell, sinc)
- [x] Complete TeX accents (hat, tilde, vec, dot, overline, underline, overbrace)
- [x] TeX delimiters (left/right, langle/rangle, ceil/floor)
- [x] 20+ additional math operators (wedge, vee, otimes, oplus, arrows, dots)
- [x] Functional date handling (DateFormatter, AutoDateLocator, DayLocator, etc.)
- [x] sharex/sharey axis synchronization
- [x] indicate_inset / indicate_inset_zoom
- [x] secondary_xaxis / secondary_yaxis
- [x] ArtistAnimation.save() functional
- [x] WebAgg toolbar (Pan, Zoom, Home, Save, scroll wheel)
- [x] Type stubs (.pyi) for pyplot module

---

## Contributing

Pick any unchecked item, open an issue to discuss, submit a PR with tests.

**Highest impact areas:**
1. Qt/GTK backends (v5.1.0) — needed for desktop applications
2. LaTeX rendering (v6.0.0) — needed for scientific papers
3. Pick events / 3D rotation (v5.1.0) — interactive artist selection
