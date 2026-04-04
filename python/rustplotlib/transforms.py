"""Transform module — backed by Rust Affine2D for composable transforms."""

from rustplotlib._rustplotlib import Affine2D as _RustAffine2D


class Bbox:
    """Bounding box defined by two points."""

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

    def __repr__(self):
        return f"Bbox([[{self.x0}, {self.y0}], [{self.x1}, {self.y1}]])"


class Affine2D:
    """2D affine transform backed by Rust.

    Supports rotate, scale, translate, compose, and inverse.
    """

    def __init__(self, matrix=None):
        if matrix is not None:
            self._rust = _RustAffine2D()
            # Set from matrix if provided (not yet supported in Rust constructor)
        else:
            self._rust = _RustAffine2D()

    @staticmethod
    def identity():
        return Affine2D()

    def rotate(self, angle):
        """Rotate by angle (radians). Returns new Affine2D."""
        r = Affine2D.__new__(Affine2D)
        r._rust = _RustAffine2D.rotate(angle)
        result = Affine2D.__new__(Affine2D)
        result._rust = self._rust.compose(r._rust)
        return result

    def rotate_deg(self, degrees):
        """Rotate by angle (degrees). Returns new Affine2D."""
        r = Affine2D.__new__(Affine2D)
        r._rust = _RustAffine2D.rotate_deg(degrees)
        result = Affine2D.__new__(Affine2D)
        result._rust = self._rust.compose(r._rust)
        return result

    def scale(self, sx, sy=None):
        """Scale. Returns new Affine2D."""
        if sy is None:
            sy = sx
        s = Affine2D.__new__(Affine2D)
        s._rust = _RustAffine2D.scale(sx, sy)
        result = Affine2D.__new__(Affine2D)
        result._rust = self._rust.compose(s._rust)
        return result

    def translate(self, tx, ty):
        """Translate. Returns new Affine2D."""
        t = Affine2D.__new__(Affine2D)
        t._rust = _RustAffine2D.translate(tx, ty)
        result = Affine2D.__new__(Affine2D)
        result._rust = self._rust.compose(t._rust)
        return result

    def transform(self, points):
        """Transform points. Accepts (x,y) or list of (x,y)."""
        if isinstance(points, (list, tuple)) and len(points) == 2 and isinstance(points[0], (int, float)):
            return self._rust.transform_point(float(points[0]), float(points[1]))
        return self._rust.transform_points([(float(p[0]), float(p[1])) for p in points])

    def inverted(self):
        """Return the inverse transform."""
        result = Affine2D.__new__(Affine2D)
        result._rust = self._rust.inverted()
        return result

    def is_identity(self):
        return self._rust.is_identity()

    def get_matrix(self):
        return self._rust.get_matrix()

    def __add__(self, other):
        """Compose transforms: self + other."""
        if isinstance(other, Affine2D):
            result = Affine2D.__new__(Affine2D)
            result._rust = self._rust.compose(other._rust)
            return result
        return self

    def __radd__(self, other):
        return self.__add__(other)


class BboxTransform:
    """Transform between two bounding boxes."""

    def __init__(self, boxin, boxout):
        self.boxin = boxin
        self.boxout = boxout

    def transform(self, points):
        """Transform points from boxin to boxout coordinates."""
        if isinstance(points, (list, tuple)) and len(points) == 2 and isinstance(points[0], (int, float)):
            x, y = float(points[0]), float(points[1])
            nx = self.boxout.x0 + (x - self.boxin.x0) / self.boxin.width * self.boxout.width
            ny = self.boxout.y0 + (y - self.boxin.y0) / self.boxin.height * self.boxout.height
            return (nx, ny)
        return [(self.transform(p)) for p in points]


class BlendedGenericTransform:
    """Blended transform using different x and y transforms."""

    def __init__(self, x_transform, y_transform):
        self.x_transform = x_transform
        self.y_transform = y_transform


class ScaledTranslation:
    """Translation in scaled coordinates."""

    def __init__(self, xt, yt, scale_trans):
        self.xt = xt
        self.yt = yt
        self.scale_trans = scale_trans


class IdentityTransform:
    """Identity transform (no-op)."""

    def transform(self, points):
        return points

    def inverted(self):
        return self
