# RustPlotLib Roadmap

This roadmap outlines the path to full matplotlib feature parity, implemented entirely in Rust.

---

## Phase 1: Essential Enhancements (v0.2.0) — COMPLETE

- [x] **Logarithmic scale** — `set_xscale('log')`, `set_yscale('log')`
- [x] **Twin axes** — `twinx()` for dual Y axes
- [x] **Error bars** — `errorbar()` with xerr/yerr
- [x] **Fill between** — `fill_between()`, `fill_betweenx()`
- [x] **Horizontal bars** — `barh()`
- [x] **Stacked bars** — `bottom` parameter for stacked bars
- [x] **Text annotations** — `text()`, `annotate()` with arrows
- [x] **Colorbar** — `colorbar()` for imshow/contour
- [x] **Axis formatting** — custom tick labels, tick rotation, tick fontsize
- [x] **Aspect ratio** — `set_aspect('equal')`, `set_aspect('auto')`
- [x] **Box plots** — `boxplot()`
- [x] **Pie charts** — `pie()`
- [x] **Stem plots** — `stem()`
- [x] **Step plots** — `step()`
- [x] **PDF output** — savefig to PDF
- [x] **DPI control** — proper DPI handling in savefig
- [x] **Figure suptitle** — `suptitle()` for figure-level title
- [x] **Subplot spacing** — `subplots_adjust()`, `hspace`, `wspace`
- [x] **Invert axes** — `invert_xaxis()`, `invert_yaxis()`
- [x] **Axis visibility** — `set_visible(False)`, `axis('off')`

## Phase 2: Advanced 2D (v0.3.0) — COMPLETE

- [x] **Contour plots** — `contour()`, `contourf()` with marching squares
- [x] **Quiver plots** — `quiver()` for vector arrows
- [x] **Streamplots** — `streamplot()` with Euler integration
- [x] **Hexbin plots** — `hexbin()` for 2D histograms
- [x] **Polar plots** — `subplot_polar()`, polar coordinates
- [x] **Span selectors** — `axhspan()`, `axvspan()`, `axhline()`, `axvline()`
- [x] **Patch objects** — `Rectangle`, `Circle`, `Polygon`, `FancyBboxPatch`, `Wedge`
- [x] **GridSpec** — `GridSpec`, `SubplotSpec` (compatibility stubs)
- [x] **Multiple colormaps** — 35+ colormaps with accurate data
- [x] **Custom colormaps** — `LinearSegmentedColormap`, `Normalize`, `LogNorm`
- [ ] **Heatmap annotations** — text inside imshow cells
- [ ] **Table** — `table()` inside axes (partial)
- [ ] **Path effects** — shadows, strokes
- [ ] **Clipping** — proper clip to axes bounds
- [ ] **Constrained layout** — automatic spacing algorithm

## Phase 3: Styles and Themes (v0.3.0) — COMPLETE

- [x] **Style sheets** — `plt.style.use()` with 6 built-in themes
- [x] **rcParams** — functional global configuration (30+ keys)
- [x] **Dark mode** — `dark_background` theme
- [x] **Spine customization** — `spines['top'].set_visible(False)`
- [x] **Tick parameters** — `tick_params()` with direction, length, width, colors
- [x] **Figure facecolor/edgecolor** — background customization
- [x] **Axes facecolor** — per-axes background
- [x] **Savefig options** — `transparent`, `dpi` parameters
- [x] **LaTeX-to-Unicode** — Greek letters, subscripts, superscripts, math operators
- [ ] **Custom fonts** — load user fonts, font families
- [ ] **Full LaTeX rendering** — math text with TeX-like layout engine
- [ ] **Unicode support** — full Unicode text rendering
- [ ] **Cycle customization** — `cycler()` (stub exists)
- [ ] **Grid customization** — major/minor grid separation

## Phase 4: 3D Plotting (v0.4.0) — COMPLETE

- [x] **3D axes** — `add_subplot(projection='3d')`
- [x] **3D line plots** — `plot(x, y, z)`
- [x] **3D scatter** — `scatter(x, y, z)` with depth sorting
- [x] **3D surface** — `plot_surface(X, Y, Z)` with colormaps
- [x] **3D wireframe** — `plot_wireframe(X, Y, Z)`
- [x] **3D bar charts** — `bar3d()` with shading
- [x] **Camera control** — `view_init(elev, azim)`
- [x] **Z-axis** — labels, limits
- [ ] **3D contour** — `contour3D()`, `contourf3D()`
- [ ] **Trisurf** — `plot_trisurf()`
- [ ] **Mouse rotation** — interactive 3D navigation
- [ ] **Projection types** — perspective projection

## Phase 5: Interactivity and Animation (v0.4.0) — COMPLETE

- [x] **Animation** — `FuncAnimation` with frame generation
- [x] **GIF export** — save animations as GIF (via Pillow)
- [x] **PNG sequence** — save animation frames individually
- [x] **Event stubs** — `canvas.mpl_connect()` compatibility
- [x] **Widget stubs** — `Slider`, `Button`, `CheckButtons`, `RadioButtons`, `TextBox`
- [ ] **Real event handling** — mouse click, motion, key press, scroll
- [ ] **Zoom and pan** — interactive navigation
- [ ] **Functional widgets** — working Slider, Button with callbacks
- [ ] **Blitting** — fast animation via partial redraw
- [ ] **MP4 export** — save animations as video

## Phase 6: Backend System (v0.4.0) — PARTIAL

- [x] **Backend selection** — `rustplotlib.use('Agg')` equivalent
- [x] **Headless backend** — pure raster, no display
- [ ] **Qt backend** — QApplication integration
- [ ] **GTK backend** — GTK3/GTK4 integration
- [ ] **Tk backend** — Tkinter integration
- [ ] **Web/HTML backend** — render to HTML5 Canvas
- [ ] **Jupyter backend** — inline display in Jupyter notebooks
- [ ] **Cairo backend** — vector rendering via Cairo
- [ ] **macOS native backend** — Cocoa/Metal integration

## Phase 7: Data Integration (v0.4.0) — COMPLETE

- [x] **Pandas integration** — plot directly from DataFrame/Series
- [x] **NumPy support** — full array support
- [x] **NaN handling** — automatic gaps in line plots
- [x] **Date handling** — `date2num()`, `num2date()`, formatters, locators
- [x] **Categorical axes** — string-based categorical data
- [ ] **xarray support** — labeled multi-dimensional arrays
- [ ] **Units support** — pint, astropy.units
- [ ] **NumPy zero-copy** — via PyO3 without `.tolist()`

## Phase 8: Advanced Features (v0.4.0) — PARTIAL

- [x] **Violin plots** — `violinplot()` with Gaussian KDE
- [x] **Subplot mosaic** — `subplot_mosaic()` for named layouts
- [x] **Table** — `table()` inside axes
- [x] **hlines/vlines** — multiple reference lines
- [x] **PdfPages** — multi-page PDF export
- [x] **fill_betweenx** — horizontal fill between
- [ ] **Sankey diagrams** — flow diagrams
- [ ] **Radar/spider charts** — polar bar charts
- [ ] **Inset axes** — `inset_axes()`, `zoomed_inset_axes()`
- [ ] **Broken axes** — axes with discontinuities
- [ ] **Geographic projections** — Basemap-like functionality
- [ ] **Pickle support** — serialize/deserialize figures
- [ ] **Copy to clipboard** — system clipboard integration

## Phase 9: Full API Compatibility (v1.0.0) — PARTIAL

- [x] **pyplot core API** — 50+ functions implemented
- [x] **Axes methods** — 40+ methods implemented
- [x] **Figure methods** — 15+ methods implemented
- [x] **Type stubs** — `.pyi` files for IDE autocompletion
- [x] **Native SVG** — real vector SVG output
- [x] **Security hardening** — path validation, dimension limits, no panics
- [ ] **100% pyplot coverage** — remaining pyplot functions
- [ ] **Default style parity** — match matplotlib's appearance pixel-for-pixel
- [ ] **Image comparison tests** — regression testing against matplotlib output
- [ ] **Full documentation** — API docs with examples
- [ ] **Backward compatibility** — support for deprecated matplotlib APIs

---

## What's Next

Priority areas for continued development:

1. **Jupyter inline display** — `%matplotlib inline` support
2. **Full LaTeX math rendering** — proper TeX layout engine
3. **Custom fonts** — load user .ttf/.otf files
4. **NumPy zero-copy** — eliminate `.tolist()` overhead
5. **Interactive 3D** — mouse rotation for 3D plots
6. **Functional widgets** — working Slider, Button with event callbacks
7. **Qt/GTK backends** — native GUI integration
8. **Image comparison tests** — automated visual regression testing

---

## Contributing

We welcome contributions! Pick any unchecked item from a phase, open an issue to discuss the approach, and submit a PR with tests.

**Priority areas:**
1. Turning stub modules into real implementations
2. Bug fixes and edge cases
3. Performance optimizations
4. Documentation and examples
