"""WebAgg backend — serves interactive figures via HTTP in the browser.

Uses Python's built-in http.server + base64 PNG encoding.
The browser displays the figure as an image and sends mouse/keyboard events
back via fetch() requests.

Usage:
    import rustplotlib.pyplot as plt
    plt.switch_backend('webagg')
    fig, ax = plt.subplots()
    ax.plot([1, 2, 3], [4, 5, 6])
    plt.show()  # Opens browser at http://localhost:8988
"""

import base64
import json
import threading
import webbrowser
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import parse_qs, urlparse

from rustplotlib.backends.backend_base import FigureCanvasBase, FigureManagerBase
from rustplotlib.events import MouseEvent, KeyEvent


_current_figure = None
_server_port = 8988


class FigureCanvasWebAgg(FigureCanvasBase):
    """Canvas that renders to PNG and serves via HTTP."""

    def __init__(self, figure):
        super().__init__(figure)
        self._png_data = None

    def draw(self):
        """Render figure to PNG bytes."""
        rust_fig = self.figure._fig
        self._png_data = rust_fig.render_to_png_bytes()
        self.callbacks.process("draw_event")

    def get_png_base64(self):
        """Return the current figure as base64-encoded PNG."""
        if self._png_data is None:
            self.draw()
        return base64.b64encode(self._png_data).decode('ascii')

    def get_png_bytes(self):
        if self._png_data is None:
            self.draw()
        return self._png_data


class WebAggHandler(BaseHTTPRequestHandler):
    """HTTP handler for WebAgg backend."""

    def log_message(self, format, *args):
        pass  # Suppress default logging

    def do_GET(self):
        parsed = urlparse(self.path)

        if parsed.path == '/' or parsed.path == '/index.html':
            self._serve_html()
        elif parsed.path == '/figure.png':
            self._serve_png()
        elif parsed.path == '/api/status':
            self._json_response({"status": "ok"})
        elif parsed.path.startswith('/api/event'):
            self._handle_event(parsed)
        else:
            self.send_error(404)

    def do_POST(self):
        if self.path == '/api/event':
            length = int(self.headers.get('Content-Length', 0))
            body = self.rfile.read(length).decode('utf-8')
            try:
                data = json.loads(body)
                self._process_event(data)
            except Exception:
                pass
            self._json_response({"ok": True})
        else:
            self.send_error(404)

    def _serve_html(self):
        global _current_figure
        html = _generate_html()
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Content-Length', len(html))
        self.end_headers()
        self.wfile.write(html.encode('utf-8'))

    def _serve_png(self):
        global _current_figure
        if _current_figure is None:
            self.send_error(404)
            return
        canvas = _current_figure._canvas
        if not isinstance(canvas, FigureCanvasWebAgg):
            canvas = FigureCanvasWebAgg(_current_figure)
        png = canvas.get_png_bytes()
        self.send_response(200)
        self.send_header('Content-Type', 'image/png')
        self.send_header('Content-Length', len(png))
        self.send_header('Cache-Control', 'no-cache')
        self.end_headers()
        self.wfile.write(png)

    def _json_response(self, data):
        body = json.dumps(data).encode('utf-8')
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', len(body))
        self.end_headers()
        self.wfile.write(body)

    def _handle_event(self, parsed):
        params = parse_qs(parsed.query)
        data = {k: v[0] for k, v in params.items()}
        self._process_event(data)
        self._json_response({"ok": True})

    def _process_event(self, data):
        global _current_figure
        if _current_figure is None:
            return
        canvas = _current_figure._canvas
        event_type = data.get('type', '')
        x = float(data.get('x', 0))
        y = float(data.get('y', 0))

        if event_type in ('button_press_event', 'button_release_event'):
            button = int(data.get('button', 1))
            me = MouseEvent(event_type, canvas, x=x, y=y, button=button)
            canvas.callbacks.process(event_type, me)
        elif event_type in ('key_press_event', 'key_release_event'):
            key = data.get('key', '')
            ke = KeyEvent(event_type, canvas, x=x, y=y, key=key)
            canvas.callbacks.process(event_type, ke)
        elif event_type == 'motion_notify_event':
            me = MouseEvent(event_type, canvas, x=x, y=y)
            canvas.callbacks.process(event_type, me)


class FigureManagerWebAgg(FigureManagerBase):
    """Manager that starts the HTTP server and opens a browser."""

    def __init__(self, canvas, num):
        super().__init__(canvas, num)

    def show(self):
        global _current_figure, _server_port
        _current_figure = self.canvas.figure

        # Start server in background thread
        server = HTTPServer(('127.0.0.1', _server_port), WebAggHandler)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()

        url = f'http://127.0.0.1:{_server_port}/'
        print(f'WebAgg serving at {url}')
        webbrowser.open(url)

        # Block until user closes (Ctrl+C)
        try:
            import time
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            server.shutdown()


def _generate_html():
    """Generate the HTML page for the WebAgg viewer."""
    return """<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>RustPlotLib — WebAgg</title>
    <style>
        body { margin: 0; background: #1a1a2e; display: flex; justify-content: center;
               align-items: center; min-height: 100vh; font-family: system-ui; }
        #container { background: white; border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.3);
                     padding: 8px; }
        #figure { cursor: crosshair; }
        #status { color: #888; font-size: 12px; padding: 4px 8px; }
    </style>
</head>
<body>
    <div id="container">
        <img id="figure" src="/figure.png" draggable="false">
        <div id="status">Ready — click to interact</div>
    </div>
    <script>
        const img = document.getElementById('figure');
        const status = document.getElementById('status');

        function sendEvent(type, data) {
            fetch('/api/event', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({type, ...data})
            }).then(() => {
                // Refresh image after event
                img.src = '/figure.png?' + Date.now();
            });
        }

        img.addEventListener('mousedown', e => {
            const rect = img.getBoundingClientRect();
            sendEvent('button_press_event', {
                x: e.clientX - rect.left, y: e.clientY - rect.top,
                button: e.button + 1
            });
        });

        img.addEventListener('mouseup', e => {
            const rect = img.getBoundingClientRect();
            sendEvent('button_release_event', {
                x: e.clientX - rect.left, y: e.clientY - rect.top,
                button: e.button + 1
            });
        });

        img.addEventListener('mousemove', e => {
            const rect = img.getBoundingClientRect();
            const x = (e.clientX - rect.left).toFixed(0);
            const y = (e.clientY - rect.top).toFixed(0);
            status.textContent = `x=${x}  y=${y}`;
        });

        document.addEventListener('keydown', e => {
            sendEvent('key_press_event', {key: e.key, x: 0, y: 0});
        });
    </script>
</body>
</html>"""
