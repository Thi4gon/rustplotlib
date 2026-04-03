"""Widget stubs for matplotlib compatibility."""


class Slider:
    def __init__(self, ax, label, valmin, valmax, valinit=None, **kwargs):
        self.ax = ax
        self.label = label
        self.val = valinit if valinit is not None else valmin
        self._observers = []

    def on_changed(self, func):
        self._observers.append(func)

    def set_val(self, val):
        self.val = val


class Button:
    def __init__(self, ax, label, **kwargs):
        self.ax = ax
        self.label = label
        self._observers = []

    def on_clicked(self, func):
        self._observers.append(func)


class CheckButtons:
    def __init__(self, ax, labels, actives=None, **kwargs):
        self.ax = ax
        self.labels = labels
        self.actives = actives or [False] * len(labels)

    def on_clicked(self, func):
        pass


class RadioButtons:
    def __init__(self, ax, labels, active=0, **kwargs):
        self.ax = ax
        self.labels = labels
        self.active = active

    def on_clicked(self, func):
        pass


class TextBox:
    def __init__(self, ax, label, initial='', **kwargs):
        self.ax = ax
        self.label = label
        self.text = initial

    def on_submit(self, func):
        pass


class Cursor:
    def __init__(self, ax, **kwargs):
        self.ax = ax


class SpanSelector:
    def __init__(self, ax, onselect, direction, **kwargs):
        self.ax = ax


class RectangleSelector:
    def __init__(self, ax, onselect, **kwargs):
        self.ax = ax


class LassoSelector:
    def __init__(self, ax, onselect, **kwargs):
        self.ax = ax
