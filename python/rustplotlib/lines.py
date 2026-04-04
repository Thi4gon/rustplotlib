"""Lines module compatibility."""


class Line2D:
    def __init__(self, xdata, ydata, **kwargs):
        self.xdata = xdata
        self.ydata = ydata

    def set_data(self, x, y):
        self.xdata = x
        self.ydata = y

    def get_xdata(self):
        return self.xdata

    def get_ydata(self):
        return self.ydata

    def set_color(self, c):
        pass

    def set_linewidth(self, lw):
        pass

    def set_linestyle(self, ls):
        pass

    def set_marker(self, m):
        pass

    def set_label(self, label):
        pass

    def get_color(self):
        return 'blue'

    def get_label(self):
        return ''

    def remove(self):
        pass

    def set_visible(self, b):
        pass

    def get_visible(self):
        return True
