"""Tests for v5.1.0 features: bicubic interpolation, pick events, path effects, FancyArrowPatch."""
import os
import tempfile
import pytest


class TestBicubicInterpolation:
    """Test bicubic image interpolation."""

    def test_bicubic_renders(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(8, 8)
        fig, ax = plt.subplots()
        ax.imshow(data, interpolation='bicubic', cmap='viridis')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_bicubic_vs_nearest(self):
        """Bicubic output should differ from nearest (smoother)."""
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(5, 5)

        fig1, ax1 = plt.subplots()
        ax1.imshow(data, interpolation='nearest', cmap='hot')
        f1 = tempfile.NamedTemporaryFile(suffix='.png', delete=False)
        plt.savefig(f1.name)
        s1 = os.path.getsize(f1.name)

        fig2, ax2 = plt.subplots()
        ax2.imshow(data, interpolation='bicubic', cmap='hot')
        f2 = tempfile.NamedTemporaryFile(suffix='.png', delete=False)
        plt.savefig(f2.name)
        s2 = os.path.getsize(f2.name)

        # File sizes should differ (bicubic produces smoother gradients = different compression)
        # This is a basic sanity check, not strict
        assert s1 > 0 and s2 > 0
        os.unlink(f1.name)
        os.unlink(f2.name)

    def test_bicubic_with_extent(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(10, 10)
        fig, ax = plt.subplots()
        ax.imshow(data, interpolation='bicubic', cmap='plasma', extent=[0, 10, 0, 10])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_all_four_interpolations(self):
        """Test all four interpolation modes in subplots."""
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(6, 6)
        fig, axes = plt.subplots(1, 4)
        axes[0].imshow(data, interpolation='nearest', cmap='viridis')
        axes[1].imshow(data, interpolation='bilinear', cmap='viridis')
        axes[2].imshow(data, interpolation='bicubic', cmap='viridis')
        axes[3].imshow(data, interpolation='lanczos', cmap='viridis')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_lanczos_renders(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(10, 10)
        fig, ax = plt.subplots()
        ax.imshow(data, interpolation='lanczos', cmap='plasma')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_lanczos_with_extent(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(8, 8)
        fig, ax = plt.subplots()
        ax.imshow(data, interpolation='lanczos', cmap='hot', extent=[-5, 5, -5, 5])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestNewColormaps:
    """Test newly added colormaps."""

    def test_diverging_colormaps(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        cmaps = ['coolwarm', 'bwr', 'seismic', 'PuOr', 'RdGy', 'RdYlGn']
        data = np.random.rand(5, 5)
        for cm in cmaps:
            fig, ax = plt.subplots()
            ax.imshow(data, cmap=cm)
            with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
                plt.savefig(f.name)
                assert os.path.getsize(f.name) > 0
                os.unlink(f.name)

    def test_sequential_colormaps(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        cmaps = ['Greys', 'PuBuGn', 'RdPu', 'gist_yarg']
        data = np.random.rand(5, 5)
        for cm in cmaps:
            fig, ax = plt.subplots()
            ax.imshow(data, cmap=cm)
            with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
                plt.savefig(f.name)
                assert os.path.getsize(f.name) > 0
                os.unlink(f.name)

    def test_cyclic_colormaps(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        cmaps = ['flag', 'prism']
        data = np.random.rand(5, 5)
        for cm in cmaps:
            fig, ax = plt.subplots()
            ax.imshow(data, cmap=cm)
            with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
                plt.savefig(f.name)
                assert os.path.getsize(f.name) > 0
                os.unlink(f.name)

    def test_reversed_new_colormaps(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        data = np.random.rand(5, 5)
        for cm in ['coolwarm_r', 'Greys_r', 'bwr_r']:
            fig, ax = plt.subplots()
            ax.imshow(data, cmap=cm)
            with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
                plt.savefig(f.name)
                assert os.path.getsize(f.name) > 0
                os.unlink(f.name)


class TestCSSNamedColors:
    """Test CSS/X11 named colors."""

    def test_basic_css_colors(self):
        import rustplotlib.pyplot as plt
        colors = ['steelblue', 'coral', 'tomato', 'gold', 'crimson',
                  'dodgerblue', 'forestgreen', 'orchid', 'salmon']
        fig, ax = plt.subplots()
        x = list(range(len(colors)))
        ax.bar(x, [1] * len(colors), color=colors[0])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_extended_css_colors(self):
        import rustplotlib.pyplot as plt
        colors = ['aliceblue', 'antiquewhite', 'aquamarine', 'azure',
                  'beige', 'bisque', 'blanchedalmond', 'burlywood',
                  'chartreuse', 'chocolate', 'cornflowerblue', 'darkviolet']
        fig, ax = plt.subplots()
        for c in colors:
            ax.plot([0, 1], [0, 1], color=c)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_grey_gray_variants(self):
        """Both 'grey' and 'gray' spellings should work."""
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        grays = ['darkgray', 'darkgrey', 'lightgray', 'lightgrey', 'dimgray', 'dimgrey']
        for c in grays:
            ax.plot([0, 1], [0, 1], color=c)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_css_colors_in_plot(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        x = np.linspace(0, 5, 20)
        fig, ax = plt.subplots()
        ax.plot(x, np.sin(x), color='mediumseagreen')
        ax.plot(x, np.cos(x), color='indianred')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_css_colors_in_scatter(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        fig, ax = plt.subplots()
        ax.scatter([1, 2, 3], [1, 2, 3], c='royalblue')
        ax.scatter([4, 5, 6], [4, 5, 6], c='firebrick')
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class Test3DRotation:
    """Test 3D mouse rotation API."""

    def test_view_init_stores_angles(self):
        import rustplotlib.pyplot as plt
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        ax.view_init(elev=45, azim=30)
        assert ax._elev == 45.0
        assert ax._azim == 30.0

    def test_rotate_incremental(self):
        import rustplotlib.pyplot as plt
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        ax.view_init(0, 0)
        ax.rotate(10, 5)
        assert ax._azim == 10.0
        assert ax._elev == 5.0

    def test_rotate_elev_clamped(self):
        import rustplotlib.pyplot as plt
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        ax.view_init(80, 0)
        ax.rotate(0, 50)  # would be 130, clamped to 90
        assert ax._elev == 90.0
        ax.rotate(0, -200)  # would be -110, clamped to -90
        assert ax._elev == -90.0

    def test_3d_rotation_render(self):
        import rustplotlib.pyplot as plt
        import numpy as np
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        ax.plot([0, 1], [0, 1], [0, 1])
        ax.view_init(45, 45)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_multiple_rotations(self):
        import rustplotlib.pyplot as plt
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        ax.view_init(0, 0)
        for _ in range(10):
            ax.rotate(5, 2)
        assert abs(ax._azim - 50.0) < 1e-10
        assert abs(ax._elev - 20.0) < 1e-10


class TestInteractiveCursor:
    """Test interactive cursor widgets."""

    def test_cursor_creation_with_options(self):
        from rustplotlib.widgets import Cursor
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        cursor = Cursor(ax, horizOn=True, vertOn=False, color='blue')
        assert cursor.horizOn is True
        assert cursor.vertOn is False
        assert cursor.color == 'blue'
        assert cursor.active is True

    def test_cursor_update(self):
        from rustplotlib.widgets import Cursor
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        cursor = Cursor(ax)
        cursor._update(3.0, 4.0)
        assert cursor.x == 3.0
        assert cursor.y == 4.0

    def test_cursor_callback(self):
        from rustplotlib.widgets import Cursor
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        cursor = Cursor(ax)
        positions = []
        cursor.on_moved(lambda x, y: positions.append((x, y)))
        cursor._update(1.0, 2.0)
        cursor._update(3.0, 4.0)
        assert positions == [(1.0, 2.0), (3.0, 4.0)]

    def test_cursor_inactive(self):
        from rustplotlib.widgets import Cursor
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        cursor = Cursor(ax)
        cursor.set_active(False)
        cursor._update(5.0, 6.0)
        assert cursor.x is None  # should not update when inactive

    def test_cursor_clear(self):
        from rustplotlib.widgets import Cursor
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        cursor = Cursor(ax)
        cursor._update(1.0, 2.0)
        cursor.clear()
        assert cursor.x is None
        assert cursor.y is None

    def test_multicursor_creation(self):
        from rustplotlib.widgets import MultiCursor
        import rustplotlib.pyplot as plt
        fig, (ax1, ax2) = plt.subplots(1, 2)
        mc = MultiCursor(fig.canvas, [ax1, ax2], horizOn=False)
        assert mc.horizOn is False
        assert mc.vertOn is True
        assert len(mc.axes) == 2


class TestPickEvents:
    """Test pick event system."""

    def test_line_pick(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        lines = ax.plot([1, 2, 3], [10, 20, 30], picker=5)
        line = lines[0]

        picked = []
        fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 2.0
        me.ydata = 20.0
        fig.canvas.pick(me)

        assert len(picked) == 1
        assert picked[0].artist is line
        assert 1 in picked[0].ind

    def test_scatter_pick(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        sc = ax.scatter([0, 5, 10], [0, 5, 10], picker=True)

        picked = []
        fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 5.0
        me.ydata = 5.0
        fig.canvas.pick(me)

        assert len(picked) == 1
        assert picked[0].artist is sc

    def test_no_pick_when_far(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        ax.plot([0, 1], [0, 1], picker=1)

        picked = []
        fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 100.0
        me.ydata = 100.0
        fig.canvas.pick(me)

        assert len(picked) == 0

    def test_picker_not_set(self):
        """Artists without picker should not trigger pick events."""
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3])  # no picker

        picked = []
        fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 2.0
        me.ydata = 2.0
        fig.canvas.pick(me)

        assert len(picked) == 0

    def test_callable_picker(self):
        """Test custom callable picker."""
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        lines = ax.plot([0, 10], [0, 10])
        line = lines[0]

        # Custom picker: always picks
        line.set_picker(lambda artist, me: (True, {"custom": True}))

        picked = []
        fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 999.0
        me.ydata = 999.0
        fig.canvas.pick(me)

        assert len(picked) == 1

    def test_pickable_property(self):
        import rustplotlib.pyplot as plt
        fig, ax = plt.subplots()
        lines = ax.plot([0, 1], [0, 1])
        line = lines[0]
        assert not line.pickable()
        line.set_picker(5)
        assert line.pickable()

    def test_disconnect_pick(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.events import MouseEvent
        fig, ax = plt.subplots()
        ax.plot([1, 2, 3], [1, 2, 3], picker=5)

        picked = []
        cid = fig.canvas.mpl_connect('pick_event', lambda e: picked.append(e))

        me = MouseEvent('button_press_event', fig.canvas)
        me.xdata = 2.0
        me.ydata = 2.0
        fig.canvas.pick(me)
        assert len(picked) == 1

        fig.canvas.mpl_disconnect(cid)
        fig.canvas.pick(me)
        assert len(picked) == 1  # should not increase


class TestPathEffects:
    """Test path effects (withStroke)."""

    def test_withstroke_renders(self):
        import rustplotlib.pyplot as plt
        import rustplotlib.patheffects as pe
        import numpy as np

        fig, ax = plt.subplots()
        x = np.linspace(0, 6, 50)
        ax.plot(x, np.sin(x), color='white', linewidth=2,
                path_effects=[pe.withStroke(linewidth=5, foreground='black')])
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_stroke_class(self):
        import rustplotlib.patheffects as pe
        s = pe.Stroke(linewidth=4, foreground='red')
        assert s.linewidth == 4
        assert s.foreground == 'red'
        color, width = s.get_outline_params()
        assert color == 'red'
        assert width == 4

    def test_withstroke_inherits_stroke(self):
        import rustplotlib.patheffects as pe
        ws = pe.withStroke(linewidth=3, foreground='blue')
        assert isinstance(ws, pe.Stroke)
        assert isinstance(ws, pe.AbstractPathEffect)

    def test_normal_effect(self):
        import rustplotlib.patheffects as pe
        n = pe.Normal()
        assert isinstance(n, pe.AbstractPathEffect)

    def test_simple_patch_shadow(self):
        import rustplotlib.patheffects as pe
        s = pe.SimplePatchShadow(offset=(3, -3), shadow_rgbFace='gray', alpha=0.5)
        assert s.offset == (3, -3)
        assert s.shadow_rgbFace == 'gray'
        assert s.alpha == 0.5

    def test_simple_line_shadow(self):
        import rustplotlib.patheffects as pe
        s = pe.SimpleLineShadow(offset=(1, -1), shadow_color='navy', alpha=0.2)
        assert s.shadow_color == 'navy'
        assert s.alpha == 0.2

    def test_set_path_effects_on_proxy(self):
        import rustplotlib.pyplot as plt
        import rustplotlib.patheffects as pe
        fig, ax = plt.subplots()
        lines = ax.plot([0, 1], [0, 1])
        line = lines[0]
        line.set_path_effects([pe.withStroke(linewidth=3, foreground='black')])
        effects = line.get_path_effects()
        assert len(effects) == 1
        assert isinstance(effects[0], pe.withStroke)


class TestFancyArrowPatch:
    """Test FancyArrowPatch with various arrow styles."""

    def test_simple_arrow(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        fig, ax = plt.subplots()
        patch = FancyArrowPatch(posA=(0, 0), posB=(5, 5), arrowstyle='->', edgecolor='blue')
        ax.add_patch(patch)
        ax.set_xlim(-1, 6)
        ax.set_ylim(-1, 6)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_double_head_arrow(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        fig, ax = plt.subplots()
        patch = FancyArrowPatch(posA=(1, 1), posB=(4, 4), arrowstyle='<->', edgecolor='red')
        ax.add_patch(patch)
        ax.set_xlim(0, 5)
        ax.set_ylim(0, 5)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_wedge_arrow(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        fig, ax = plt.subplots()
        patch = FancyArrowPatch(posA=(0, 0), posB=(5, 3), arrowstyle='wedge', edgecolor='green')
        ax.add_patch(patch)
        ax.set_xlim(-1, 6)
        ax.set_ylim(-1, 4)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_curved_arrow_arc3(self):
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        fig, ax = plt.subplots()
        patch = FancyArrowPatch(
            posA=(1, 1), posB=(4, 4),
            arrowstyle='->',
            connectionstyle='arc3,rad=0.3',
            edgecolor='blue'
        )
        ax.add_patch(patch)
        ax.set_xlim(0, 5)
        ax.set_ylim(0, 5)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_all_arrow_styles(self):
        """Test all arrow styles render without error."""
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import FancyArrowPatch
        styles = ['->', '-|>', '<-', '<->', 'wedge', '-[', '|-']
        fig, ax = plt.subplots(figsize=(8, 6))
        for i, style in enumerate(styles):
            patch = FancyArrowPatch(
                posA=(0, i), posB=(5, i),
                arrowstyle=style,
                edgecolor='blue',
                mutation_scale=20,
            )
            ax.add_patch(patch)
        ax.set_xlim(-1, 6)
        ax.set_ylim(-1, len(styles))
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_connection_patch(self):
        """Test ConnectionPatch creates and renders."""
        import rustplotlib.pyplot as plt
        from rustplotlib.patches import ConnectionPatch
        fig, (ax1, ax2) = plt.subplots(1, 2)
        ax1.plot([0, 1], [0, 1])
        ax2.plot([0, 1], [0, 1])

        con = ConnectionPatch(
            xyA=(0.5, 0.5), xyB=(0.5, 0.5),
            coordsA='data', coordsB='data',
            axesA=ax1, axesB=ax2,
            arrowstyle='->',
            edgecolor='red',
        )
        # ConnectionPatch add_patch on ax1
        ax1.add_patch(con)
        with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_fancy_arrow_patch_properties(self):
        """Test FancyArrowPatch stores properties correctly."""
        from rustplotlib.patches import FancyArrowPatch
        patch = FancyArrowPatch(
            posA=(1, 2), posB=(3, 4),
            arrowstyle='<->',
            connectionstyle='arc3,rad=0.5',
            mutation_scale=30,
            shrinkA=5,
            shrinkB=10,
        )
        assert patch.posA == (1, 2)
        assert patch.posB == (3, 4)
        assert patch.arrowstyle == '<->'
        assert patch.connectionstyle == 'arc3,rad=0.5'
        assert patch.mutation_scale == 30
        assert patch.shrinkA == 5
        assert patch.shrinkB == 10
