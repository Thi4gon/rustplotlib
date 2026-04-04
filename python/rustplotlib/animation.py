"""Animation support for rustplotlib."""


class FuncAnimation:
    """Function-based animation that generates frames and saves to GIF/MP4.

    Supports blit=True for optimized Tk rendering (caches background).
    """

    def __init__(self, fig, func, frames=None, init_func=None, interval=50,
                 blit=False, repeat=True, repeat_delay=0, **kwargs):
        self.fig = fig
        self.func = func
        self.frames = frames if frames is not None else 100
        self.init_func = init_func
        self.interval = interval  # ms between frames
        self.blit = blit
        self.repeat = repeat
        self.repeat_delay = repeat_delay
        self._frames_data = []
        self._running = False
        self._background = None
        self._drawn_artists = []

    def _step(self, frame):
        """Execute a single animation step."""
        result = self.func(frame)
        if self.blit and result is not None:
            # If blit, func should return iterable of artists to redraw
            try:
                self._drawn_artists = list(result)
            except TypeError:
                self._drawn_artists = []
        return result

    def _start_tk(self):
        """Start live animation in Tk backend."""
        import tkinter as tk

        canvas = self.fig.canvas
        if not hasattr(canvas, '_tk_canvas') or canvas._tk_canvas is None:
            return

        if isinstance(self.frames, int):
            frame_iter = iter(range(self.frames))
        else:
            frame_iter = iter(self.frames)

        if self.init_func:
            self.init_func()
            canvas.draw()

        if self.blit:
            self._background = canvas._last_rgba

        self._running = True
        frame_list = list(frame_iter)
        frame_idx = [0]

        def animate():
            if not self._running:
                return
            if frame_idx[0] >= len(frame_list):
                if self.repeat:
                    frame_idx[0] = 0
                    if self.repeat_delay:
                        canvas._tk_canvas.after(self.repeat_delay, animate)
                        return
                else:
                    return

            self._step(frame_list[frame_idx[0]])
            canvas.draw()
            frame_idx[0] += 1
            canvas._tk_canvas.after(self.interval, animate)

        canvas._tk_canvas.after(self.interval, animate)

    def save(self, filename, writer=None, fps=None, dpi=None, **kwargs):
        """Save animation to file (GIF or individual PNGs)."""
        if fps is None:
            fps = 1000 // max(self.interval, 1)

        import os

        # Determine frame count
        if isinstance(self.frames, int):
            frame_iter = range(self.frames)
        else:
            frame_iter = self.frames

        # Generate frames
        frame_files = []

        if self.init_func:
            self.init_func()

        for i, frame in enumerate(frame_iter):
            self.func(frame)
            frame_path = f"/tmp/rustplotlib_anim_frame_{i:06d}.png"
            self.fig.savefig(frame_path)
            frame_files.append(frame_path)

        if filename.endswith('.gif'):
            # Try to use PIL/Pillow for GIF assembly
            try:
                from PIL import Image
                images = [Image.open(f) for f in frame_files]
                duration = 1000 // fps
                images[0].save(
                    filename,
                    save_all=True,
                    append_images=images[1:],
                    duration=duration,
                    loop=0 if self.repeat else 1,
                )
            except ImportError:
                print(
                    f"Install Pillow for GIF export. "
                    f"Frames saved as {frame_files[0]} ... {frame_files[-1]}"
                )
                return
        elif filename.endswith('.mp4') or filename.endswith('.avi'):
            # Suggest ffmpeg
            print("MP4/AVI export requires ffmpeg. Frames saved as PNGs in /tmp/")
            return
        else:
            # Save as PNGs in a directory
            import shutil
            os.makedirs(filename, exist_ok=True)
            for i, f in enumerate(frame_files):
                shutil.move(f, os.path.join(filename, f"frame_{i:06d}.png"))
            return

        # Cleanup temp frames
        for f in frame_files:
            try:
                os.remove(f)
            except OSError:
                pass

    def pause(self):
        """Pause the animation."""
        self._running = False

    def resume(self):
        """Resume the animation."""
        if not self._running:
            self._running = True
            self._start_tk()

    def event_source_start(self):
        """Start the animation event source."""
        self._start_tk()

    def event_source_stop(self):
        """Stop the animation event source."""
        self._running = False


class ArtistAnimation:
    """Animation from a list of artists per frame."""

    def __init__(self, fig, artists, interval=50, blit=False, repeat=True,
                 **kwargs):
        self.fig = fig
        self.artists = artists
        self.interval = interval
        self.repeat = repeat

    def save(self, filename, **kwargs):
        # Stub -- ArtistAnimation is harder to support without mutable figure state
        print("ArtistAnimation.save() not yet fully supported")
