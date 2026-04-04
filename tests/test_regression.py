"""Image comparison regression tests.

Verifies that basic plots render to non-empty PNGs with expected properties.
These tests catch rendering regressions (broken output, wrong size, etc.).
"""
import os
import tempfile
import pytest
import numpy as np


class TestRenderingRegression:
    """Verify that all major plot types render correctly."""

    def _render(self, plot_func, **kwargs):
        """Helper: create figure, run plot_func, save PNG, verify."""
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots(figsize=kwargs.get('figsize', (6.4, 4.8)))
        plot_func(ax)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name, dpi=100)
            size = os.path.getsize(f.name)
            assert size > 1000, f"PNG too small: {size} bytes"
            os.unlink(f.name)

    def test_line_plot(self):
        self._render(lambda ax: ax.plot(np.linspace(0, 10, 100).tolist(),
                                         np.sin(np.linspace(0, 10, 100)).tolist()))

    def test_scatter_plot(self):
        self._render(lambda ax: ax.scatter(np.random.rand(50).tolist(),
                                            np.random.rand(50).tolist()))

    def test_bar_chart(self):
        self._render(lambda ax: ax.bar([1, 2, 3, 4], [10, 20, 15, 25]))

    def test_histogram(self):
        self._render(lambda ax: ax.hist(np.random.randn(1000).tolist(), bins=30))

    def test_imshow_scalar(self):
        self._render(lambda ax: ax.imshow(np.random.rand(20, 20).tolist(), cmap='viridis'))

    def test_imshow_bicubic(self):
        self._render(lambda ax: ax.imshow(np.random.rand(10, 10).tolist(),
                                           interpolation='bicubic', cmap='hot'))

    def test_imshow_lanczos(self):
        self._render(lambda ax: ax.imshow(np.random.rand(10, 10).tolist(),
                                           interpolation='lanczos', cmap='plasma'))

    def test_contour(self):
        x = np.linspace(-3, 3, 50)
        X, Y = np.meshgrid(x, x)
        Z = np.exp(-(X**2 + Y**2))
        self._render(lambda ax: ax.contourf(X.tolist(), Y.tolist(), Z.tolist()))

    def test_pie_chart(self):
        self._render(lambda ax: ax.pie([30, 25, 20, 15, 10]))

    def test_boxplot(self):
        data = [np.random.randn(50).tolist() for _ in range(4)]
        self._render(lambda ax: ax.boxplot(data))

    def test_errorbar(self):
        self._render(lambda ax: ax.errorbar([1, 2, 3], [4, 5, 6],
                                             yerr=[0.5, 0.3, 0.7]))

    def test_fill_between(self):
        x = np.linspace(0, 5, 50).tolist()
        y = np.sin(np.linspace(0, 5, 50)).tolist()
        self._render(lambda ax: ax.fill_between(x, y))

    def test_step_plot(self):
        self._render(lambda ax: ax.step([1, 2, 3, 4], [1, 4, 2, 5]))

    def test_stem_plot(self):
        self._render(lambda ax: ax.stem([1, 2, 3], [4, 5, 3]))

    def test_log_scale(self):
        def plot(ax):
            ax.semilogy([1, 2, 3], [10, 100, 1000])
        self._render(plot)

    def test_twin_axes(self):
        def plot(ax):
            ax.plot([1, 2, 3], [4, 5, 6])
            ax2 = ax.twinx()
            ax2.plot([1, 2, 3], [100, 200, 300], color='red')
        self._render(plot)

    def test_colorbar(self):
        def plot(ax):
            ax.imshow(np.random.rand(10, 10).tolist(), cmap='viridis')
            ax.colorbar()
        self._render(plot)

    def test_legend(self):
        def plot(ax):
            ax.plot([1, 2, 3], [1, 2, 3], label='A')
            ax.plot([1, 2, 3], [3, 2, 1], label='B')
            ax.legend()
        self._render(plot)

    def test_annotations(self):
        def plot(ax):
            ax.plot([1, 2, 3], [4, 5, 6])
            ax.annotate('Peak', xy=(2, 5), xytext=(2.5, 5.5))
            ax.text(1.5, 4.5, 'Hello')
        self._render(plot)

    def test_subplots_2x2(self):
        import rustplotlib.pyplot as plt
        fig, axes = plt.subplots(2, 2, figsize=(10, 8))
        axes[0][0].plot([1, 2, 3], [4, 5, 6])
        axes[0][1].bar([1, 2], [3, 4])
        axes[1][0].scatter([1, 2, 3], [1, 2, 3])
        axes[1][1].hist(np.random.randn(200).tolist())
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 2000
            os.unlink(f.name)

    def test_svg_output(self):
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [4, 5, 6])
        with tempfile.NamedTemporaryFile(suffix='.svg', delete=False) as f:
            plt.savefig(f.name)
            content = open(f.name).read()
            assert '<svg' in content
            os.unlink(f.name)

    def test_pdf_output(self):
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [4, 5, 6])
        with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
            plt.savefig(f.name)
            with open(f.name, 'rb') as pf:
                header = pf.read(5)
            assert header == b'%PDF-'
            os.unlink(f.name)

    def test_eps_output(self):
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [4, 5, 6])
        with tempfile.NamedTemporaryFile(suffix='.eps', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_3d_surface(self):
        import rustplotlib.pyplot as plt
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        u = np.linspace(0, 2*np.pi, 15)
        v = np.linspace(0, np.pi, 15)
        X = np.outer(np.cos(u), np.sin(v))
        Y = np.outer(np.sin(u), np.sin(v))
        Z = np.outer(np.ones_like(u), np.cos(v))
        ax.plot_surface(X, Y, Z, cmap='viridis')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 1000
            os.unlink(f.name)

    def test_css_colors(self):
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        colors = ['steelblue', 'coral', 'tomato', 'gold', 'crimson', 'forestgreen']
        for i, c in enumerate(colors):
            ax.plot([0, 1], [i, i+1], color=c)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 1000
            os.unlink(f.name)

    def test_fancy_arrow_patch(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        fig, ax = plt.subplots()
        for i, style in enumerate(['->', '<->', 'wedge']):
            patch = FancyArrowPatch(posA=(0, i), posB=(5, i),
                                     arrowstyle=style, edgecolor='blue')
            ax.add_patch(patch)
        ax.set_xlim(-1, 6)
        ax.set_ylim(-1, 3)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 1000
            os.unlink(f.name)

    def test_path_effects(self):
        import rustplotlib.pyplot as plt
        import rustplotlib.patheffects as pe
        fig, ax = plt.subplots()
        ax.plot([0, 1, 2], [0, 1, 0], color='white', linewidth=2,
                path_effects=[pe.withStroke(linewidth=5, foreground='black')])
        ax.set_facecolor('#333333')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 1000
            os.unlink(f.name)
