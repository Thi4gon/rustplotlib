"""Text module compatibility."""


class Text:
    def __init__(self, x=0, y=0, text='', **kwargs):
        self.x = x
        self.y = y
        self.text = text

    def set_text(self, s):
        self.text = s

    def get_text(self):
        return self.text

    def set_fontsize(self, size):
        pass

    def set_color(self, c):
        pass

    def set_visible(self, b):
        pass

    def set_position(self, xy):
        self.x, self.y = xy


class Annotation(Text):
    pass
