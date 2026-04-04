"""Spines module compatibility."""


class Spine:
    def __init__(self, axes, spine_type):
        self.axes = axes
        self.spine_type = spine_type

    def set_visible(self, b):
        pass

    def set_color(self, c):
        pass

    def set_linewidth(self, lw):
        pass

    def set_position(self, position):
        pass

    def set_bounds(self, low, high):
        pass
