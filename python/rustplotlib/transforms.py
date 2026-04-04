"""Transform module compatibility."""


class Bbox:
    def __init__(self, points):
        self.points = points

    @staticmethod
    def from_bounds(x0, y0, width, height):
        return Bbox([[x0, y0], [x0 + width, y0 + height]])

    @property
    def x0(self):
        return self.points[0][0]

    @property
    def y0(self):
        return self.points[0][1]

    @property
    def x1(self):
        return self.points[1][0]

    @property
    def y1(self):
        return self.points[1][1]

    @property
    def width(self):
        return self.x1 - self.x0

    @property
    def height(self):
        return self.y1 - self.y0


class Affine2D:
    def __init__(self):
        self._matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]

    def rotate_deg(self, degrees):
        return self

    def scale(self, sx, sy=None):
        return self

    def translate(self, tx, ty):
        return self


class BboxTransform:
    def __init__(self, boxin, boxout):
        self.boxin = boxin
        self.boxout = boxout


class BlendedGenericTransform:
    def __init__(self, x_transform, y_transform):
        pass


class ScaledTranslation:
    def __init__(self, xt, yt, scale_trans):
        pass


class IdentityTransform:
    pass
