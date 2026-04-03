"""Tests for 3D plotting features."""

import os
import numpy as np
import rustplotlib.pyplot as plt


def test_3d_line_plot():
    """Test 3D line plot via subplots with projection='3d'."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    t = np.linspace(0, 4 * np.pi, 100)
    ax.plot(np.cos(t), np.sin(t), t, label='helix')
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')
    ax.set_title('3D Helix')
    ax.legend()
    fig.savefig('/tmp/test_3d_line.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_line.png')
    assert os.path.getsize('/tmp/test_3d_line.png') > 0
    os.remove('/tmp/test_3d_line.png')


def test_3d_scatter():
    """Test 3D scatter plot."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    np.random.seed(42)
    n = 50
    ax.scatter(np.random.rand(n), np.random.rand(n), np.random.rand(n),
               c='red', label='pts', alpha=0.8)
    ax.set_title('3D Scatter')
    ax.legend()
    fig.savefig('/tmp/test_3d_scatter.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_scatter.png')
    assert os.path.getsize('/tmp/test_3d_scatter.png') > 0
    os.remove('/tmp/test_3d_scatter.png')


def test_3d_surface():
    """Test 3D surface plot."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    X = np.linspace(-3, 3, 30)
    Y = np.linspace(-3, 3, 30)
    X, Y = np.meshgrid(X, Y)
    Z = np.sin(np.sqrt(X ** 2 + Y ** 2))
    ax.plot_surface(X, Y, Z, cmap='viridis', alpha=0.8)
    ax.set_title('3D Surface')
    fig.savefig('/tmp/test_3d_surface.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_surface.png')
    assert os.path.getsize('/tmp/test_3d_surface.png') > 0
    os.remove('/tmp/test_3d_surface.png')


def test_3d_wireframe():
    """Test 3D wireframe plot."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    X = np.linspace(-2, 2, 20)
    Y = np.linspace(-2, 2, 20)
    X, Y = np.meshgrid(X, Y)
    Z = X ** 2 - Y ** 2
    ax.plot_wireframe(X, Y, Z, color='blue', linewidth=0.5)
    ax.set_title('3D Wireframe')
    fig.savefig('/tmp/test_3d_wireframe.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_wireframe.png')
    assert os.path.getsize('/tmp/test_3d_wireframe.png') > 0
    os.remove('/tmp/test_3d_wireframe.png')


def test_3d_bar():
    """Test 3D bar chart."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    x = [0, 1, 2, 3]
    y = [0, 0, 0, 0]
    z = [0, 0, 0, 0]
    dx = [0.8, 0.8, 0.8, 0.8]
    dy = [0.8, 0.8, 0.8, 0.8]
    dz = [3, 5, 2, 7]
    ax.bar3d(x, y, z, dx, dy, dz, color='teal', alpha=0.8)
    ax.set_title('3D Bar')
    fig.savefig('/tmp/test_3d_bar.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_bar.png')
    assert os.path.getsize('/tmp/test_3d_bar.png') > 0
    os.remove('/tmp/test_3d_bar.png')


def test_3d_view_init():
    """Test custom camera angles."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    ax.plot([0, 1, 2], [0, 1, 0], [0, 1, 2])
    ax.view_init(elev=45, azim=-120)
    fig.savefig('/tmp/test_3d_viewinit.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_viewinit.png')
    os.remove('/tmp/test_3d_viewinit.png')


def test_3d_axis_limits():
    """Test setting axis limits on 3D axes."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    ax.plot([0, 1, 2], [0, 1, 0], [0, 1, 2])
    ax.set_xlim(left=-1.0, right=3.0)
    ax.set_ylim(bottom=-1.0, top=2.0)
    ax.set_zlim(bottom=-1.0, top=3.0)
    fig.savefig('/tmp/test_3d_lims.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_lims.png')
    os.remove('/tmp/test_3d_lims.png')


def test_3d_add_subplot():
    """Test fig.add_subplot(111, projection='3d') API."""
    fig = plt.figure(figsize=(8, 6))
    ax = fig.add_subplot(111, projection='3d')
    ax.plot([0, 1, 2], [0, 1, 0], [0, 1, 2], label='line')
    ax.set_title('add_subplot 3D')
    ax.legend()
    fig.savefig('/tmp/test_3d_addsub.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_addsub.png')
    os.remove('/tmp/test_3d_addsub.png')


def test_3d_mpl_toolkits_import():
    """Test that the mpl_toolkits compatibility stub can be imported."""
    from rustplotlib.mpl_toolkits.mplot3d import Axes3D
    assert Axes3D is not None


def test_3d_surface_colormaps():
    """Test surface plot with different colormaps."""
    X = np.linspace(-2, 2, 15)
    Y = np.linspace(-2, 2, 15)
    X, Y = np.meshgrid(X, Y)
    Z = np.exp(-(X ** 2 + Y ** 2))

    for cmap in ['viridis', 'coolwarm', 'plasma', 'hot']:
        fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
        ax.plot_surface(X, Y, Z, cmap=cmap)
        ax.set_title(f'Surface ({cmap})')
        fig.savefig(f'/tmp/test_3d_{cmap}.png')
        plt.close()
        assert os.path.exists(f'/tmp/test_3d_{cmap}.png')
        os.remove(f'/tmp/test_3d_{cmap}.png')


def test_3d_multiple_lines():
    """Test multiple line plots on the same 3D axes."""
    fig, ax = plt.subplots(subplot_kw={'projection': '3d'})
    t = np.linspace(0, 2 * np.pi, 50)
    ax.plot(np.cos(t), np.sin(t), t, label='helix 1', color='red')
    ax.plot(np.cos(t) * 0.5, np.sin(t) * 0.5, t, label='helix 2', color='blue')
    ax.legend()
    fig.savefig('/tmp/test_3d_multi.png')
    plt.close()
    assert os.path.exists('/tmp/test_3d_multi.png')
    os.remove('/tmp/test_3d_multi.png')
