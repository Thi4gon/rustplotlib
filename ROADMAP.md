# RustPlotLib Roadmap

This roadmap outlines the path to full matplotlib feature parity, implemented entirely in Rust.

## Current Status (v0.1.0 — MVP)

- [x] Line plots (`plot()`)
- [x] Scatter plots (`scatter()`)
- [x] Bar charts (`bar()`)
- [x] Histograms (`hist()`)
- [x] Image/heatmaps (`imshow()`)
- [x] Subplots (`subplots()`)
- [x] PNG and SVG output
- [x] Interactive window display
- [x] Legends, grid, titles, labels
- [x] Colors: named, hex, RGB/RGBA, shorthand
- [x] Linestyles: solid, dashed, dashdot, dotted
- [x] Markers: . o s ^ v + x D *
- [x] Colormaps: viridis, plasma, inferno, magma, hot, cool, gray, jet, Blues, Reds, Greens
- [x] Auto-tick calculation
- [x] Format strings ("r--o")
- [x] Tab10 color cycle
- [x] Embedded font (DejaVu Sans)

---

## Phase 1: Essential Enhancements (v0.2.0)

Core features that most matplotlib users expect.

- [ ] **Logarithmic scale** — `set_xscale('log')`, `set_yscale('log')`
- [ ] **Twin axes** — `twinx()`, `twiny()` for dual Y/X axes
- [ ] **Error bars** — `errorbar()` with xerr/yerr
- [ ] **Fill between** — `fill_between()`, `fill_betweenx()`
- [ ] **Horizontal bars** — `barh()`
- [ ] **Stacked bars/histograms** — `bottom` parameter, stacked hist
- [ ] **Text annotations** — `text()`, `annotate()` with arrows
- [ ] **Colorbar** — `colorbar()` for imshow/contour
- [ ] **Axis formatting** — custom tick labels, tick rotation, tick fontsize
- [ ] **Aspect ratio** — `set_aspect('equal')`, `set_aspect('auto')`
- [ ] **Box plots** — `boxplot()` / `violinplot()`
- [ ] **Pie charts** — `pie()`
- [ ] **Stem plots** — `stem()`
- [ ] **Step plots** — `step()`
- [ ] **PDF output** — savefig to PDF
- [ ] **DPI control** — proper DPI handling in savefig
- [ ] **Figure suptitle** — `suptitle()` for figure-level title
- [ ] **Subplot spacing** — `subplots_adjust()`, `hspace`, `wspace`
- [ ] **Invert axes** — `invert_xaxis()`, `invert_yaxis()`
- [ ] **Axis visibility** — `set_visible(False)`, `axis('off')`

## Phase 2: Advanced 2D (v0.3.0)

Specialized 2D plot types and customization.

- [ ] **Contour plots** — `contour()`, `contourf()` with levels
- [ ] **Streamplots** — `streamplot()` for vector fields
- [ ] **Quiver plots** — `quiver()` for vector arrows
- [ ] **Heatmap annotations** — text inside imshow cells
- [ ] **Broken bar charts** — `broken_barh()`
- [ ] **Hexbin plots** — `hexbin()` for 2D histograms
- [ ] **Polar plots** — `subplot_polar()`, polar coordinates
- [ ] **Table** — `table()` inside axes
- [ ] **Span selectors** — `axhspan()`, `axvspan()`, `axhline()`, `axvline()`
- [ ] **Custom line collections** — `LineCollection`, `PathCollection`
- [ ] **Patch objects** — `Rectangle`, `Circle`, `Polygon`, `Arrow`, `FancyArrow`, `Arc`, `Wedge`
- [ ] **Path effects** — shadows, strokes, normal
- [ ] **Clipping** — proper clip to axes bounds
- [ ] **Secondary axes** — `secondary_xaxis()`, `secondary_yaxis()`
- [ ] **GridSpec** — `GridSpec`, `SubplotSpec` for complex layouts
- [ ] **Constrained layout** — automatic spacing algorithm
- [ ] **Multiple colormaps** — full matplotlib colormap library (100+)
- [ ] **Custom colormaps** — `LinearSegmentedColormap`, `ListedColormap`
- [ ] **Colormap normalization** — `Normalize`, `LogNorm`, `SymLogNorm`, `PowerNorm`

## Phase 3: Styles and Themes (v0.4.0)

Visual customization and styling system.

- [ ] **Style sheets** — `plt.style.use('ggplot')`, `seaborn`, `dark_background`, etc.
- [ ] **rcParams** — global configuration system
- [ ] **Custom fonts** — load user fonts, font families
- [ ] **LaTeX rendering** — math text with TeX-like syntax (`$\alpha$`, `$\sum$`)
- [ ] **Unicode support** — full Unicode text rendering
- [ ] **Dark mode** — built-in dark themes
- [ ] **Spine customization** — `spines['top'].set_visible(False)`, spine positioning
- [ ] **Tick parameters** — `tick_params()` with direction, length, width, colors
- [ ] **Grid customization** — major/minor grid, grid styles per axis
- [ ] **Cycle customization** — `cycler()` for custom property cycles
- [ ] **Figure facecolor/edgecolor** — background customization
- [ ] **Axes facecolor** — per-axes background
- [ ] **Savefig options** — `transparent`, `pad_inches`, `bbox_inches='tight'`

## Phase 4: 3D Plotting (v0.5.0)

Full 3D visualization support.

- [ ] **3D axes** — `add_subplot(projection='3d')`
- [ ] **3D line plots** — `plot3D(x, y, z)`
- [ ] **3D scatter** — `scatter3D(x, y, z)`
- [ ] **3D surface** — `plot_surface(X, Y, Z)` with colormaps
- [ ] **3D wireframe** — `plot_wireframe(X, Y, Z)`
- [ ] **3D bar charts** — `bar3d()`
- [ ] **3D contour** — `contour3D()`, `contourf3D()`
- [ ] **Camera control** — `view_init(elev, azim)`, mouse rotation
- [ ] **Z-axis** — full z-axis with ticks, labels, limits
- [ ] **Projection types** — perspective, orthographic
- [ ] **3D text** — text positioned in 3D space
- [ ] **Trisurf** — `plot_trisurf()` for triangulated surfaces

## Phase 5: Interactivity and Animation (v0.6.0)

Interactive features and animation support.

- [ ] **Event handling** — mouse click, motion, key press, scroll
- [ ] **Zoom and pan** — interactive navigation toolbar
- [ ] **Widgets** — `Slider`, `Button`, `CheckButtons`, `RadioButtons`, `TextBox`
- [ ] **Cursor** — crosshair cursor, snap to data
- [ ] **Lasso selector** — freeform selection
- [ ] **Animation** — `FuncAnimation`, `ArtistAnimation`
- [ ] **Blitting** — fast animation via partial redraw
- [ ] **GIF/MP4 export** — save animations to file
- [ ] **Real-time plotting** — live data updates

## Phase 6: Backend System (v0.7.0)

Multiple rendering backends.

- [ ] **Qt backend** — `QApplication` integration for Qt5/Qt6
- [ ] **GTK backend** — GTK3/GTK4 integration
- [ ] **Tk backend** — Tkinter integration
- [ ] **Web/HTML backend** — render to HTML5 Canvas or WebGL
- [ ] **Jupyter backend** — inline display in Jupyter notebooks (`%matplotlib inline`)
- [ ] **Cairo backend** — vector rendering via Cairo
- [ ] **macOS native backend** — Cocoa/Metal integration
- [ ] **Headless backend** — pure raster, no display (for servers)
- [ ] **Backend selection** — `matplotlib.use('Agg')` equivalent

## Phase 7: Data Integration (v0.8.0)

Integration with the Python data ecosystem.

- [ ] **Pandas integration** — plot directly from DataFrame/Series
- [ ] **NumPy optimization** — zero-copy array passing via PyO3
- [ ] **xarray support** — labeled multi-dimensional arrays
- [ ] **Date handling** — `plot_date()`, date locators, date formatters
- [ ] **Categorical axes** — string-based categorical data
- [ ] **Units support** — pint, astropy.units
- [ ] **NaN handling** — proper gaps in line plots for NaN values

## Phase 8: Advanced Features (v0.9.0)

Specialized and power-user features.

- [ ] **Sankey diagrams** — `Sankey()` for flow diagrams
- [ ] **Dendrograms** — hierarchical clustering visualization
- [ ] **Geographic projections** — `Basemap`-like functionality
- [ ] **Smith charts** — for RF engineering
- [ ] **Radar/spider charts** — polar bar charts
- [ ] **Waterfall charts** — financial/cumulative
- [ ] **Gantt charts** — project timeline visualization
- [ ] **Subplots mosaic** — `subplot_mosaic()` for named subplot layouts
- [ ] **Inset axes** — `inset_axes()`, `zoomed_inset_axes()`
- [ ] **Broken axes** — axes with discontinuities
- [ ] **Multipage PDF** — `PdfPages` for multi-page documents
- [ ] **Pickle support** — serialize/deserialize figures
- [ ] **Copy to clipboard** — system clipboard integration

## Phase 9: Full API Compatibility (v1.0.0)

Complete drop-in replacement goal.

- [ ] **100% pyplot API coverage** — all matplotlib.pyplot functions
- [ ] **Axes API parity** — all Axes methods
- [ ] **Figure API parity** — all Figure methods
- [ ] **Artist hierarchy** — full artist tree (Figure → Axes → Artist)
- [ ] **Property cycle parity** — all default property cycling
- [ ] **Default style parity** — match matplotlib's default appearance pixel-for-pixel
- [ ] **Test suite** — image comparison tests against matplotlib output
- [ ] **Documentation** — full API docs with examples
- [ ] **Type stubs** — `.pyi` files for IDE autocompletion
- [ ] **Backward compatibility** — support for deprecated matplotlib APIs

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Priority areas:**
1. Phase 1 features (most impact for users)
2. Bug fixes and edge cases
3. Performance optimizations
4. Documentation and examples

## How to Help

- Pick any unchecked item from a phase
- Open an issue to discuss the approach
- Submit a PR with tests
- Report bugs or missing edge cases
