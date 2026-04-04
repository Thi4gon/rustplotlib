"""PdfPages — Save multiple figures to a real multi-page PDF."""

import os
import zlib


class PdfPages:
    """Save multiple figures to a single multi-page PDF file.

    Usage:
        with PdfPages('output.pdf') as pdf:
            fig, ax = plt.subplots()
            ax.plot([1, 2, 3])
            pdf.savefig(fig)

            fig2, ax2 = plt.subplots()
            ax2.bar([1, 2], [3, 4])
            pdf.savefig(fig2)
    """

    def __init__(self, filename, keep_empty=True, metadata=None):
        self.filename = filename
        self.keep_empty = keep_empty
        self.metadata = metadata or {}
        self._pages = []  # list of (width, height, rgb_bytes)

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()

    def savefig(self, figure=None, dpi=None, **kwargs):
        """Save a figure as a page in the PDF."""
        from rustplotlib import pyplot
        if figure is None:
            fig_proxy = pyplot.gcf()
        elif hasattr(figure, '_fig'):
            fig_proxy = figure
        else:
            # Assume it's a raw RustFigure
            class _Wrapper:
                def __init__(self, f): self._fig = f
            fig_proxy = _Wrapper(figure)

        # Render to RGBA buffer
        result = fig_proxy._fig.render_to_rgba_buffer()
        rgba_data, w, h = result

        # Convert RGBA to RGB (drop alpha, unpremultiply)
        rgb = bytearray(w * h * 3)
        for i in range(w * h):
            off = i * 4
            a = rgba_data[off + 3]
            if a > 0:
                rgb[i * 3] = min(255, rgba_data[off] * 255 // a)
                rgb[i * 3 + 1] = min(255, rgba_data[off + 1] * 255 // a)
                rgb[i * 3 + 2] = min(255, rgba_data[off + 2] * 255 // a)
            else:
                rgb[i * 3] = 255
                rgb[i * 3 + 1] = 255
                rgb[i * 3 + 2] = 255

        self._pages.append((w, h, bytes(rgb)))

    def close(self):
        """Write all pages to the multi-page PDF file."""
        if not self._pages and not self.keep_empty:
            return

        pdf = self._build_pdf()
        with open(self.filename, 'wb') as f:
            f.write(pdf)

    def _build_pdf(self):
        """Build a real multi-page PDF with embedded images."""
        if not self._pages:
            return b"%PDF-1.4\n%%EOF\n"

        n = len(self._pages)
        buf = bytearray()
        offsets = []

        buf.extend(b"%PDF-1.4\n")

        # Obj 1: Catalog
        offsets.append(len(buf))
        buf.extend(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n")

        # Obj 2: Pages
        offsets.append(len(buf))
        kids = " ".join(f"{3 + i * 3} 0 R" for i in range(n))
        buf.extend(f"2 0 obj\n<< /Type /Pages /Kids [{kids}] /Count {n} >>\nendobj\n".encode())

        # Each page: 3 objects (Page, Image, Content)
        obj_num = 3
        for w, h, rgb in self._pages:
            # Compress RGB data
            compressed = zlib.compress(rgb, 6)

            # Page object
            offsets.append(len(buf))
            img_obj = obj_num + 1
            content_obj = obj_num + 2
            buf.extend(
                f"{obj_num} 0 obj\n<< /Type /Page /Parent 2 0 R "
                f"/MediaBox [0 0 {w} {h}] "
                f"/Contents {content_obj} 0 R "
                f"/Resources << /XObject << /Img {img_obj} 0 R >> >> "
                f">>\nendobj\n".encode()
            )

            # Image XObject
            offsets.append(len(buf))
            buf.extend(
                f"{img_obj} 0 obj\n<< /Type /XObject /Subtype /Image "
                f"/Width {w} /Height {h} /ColorSpace /DeviceRGB "
                f"/BitsPerComponent 8 /Filter /FlateDecode "
                f"/Length {len(compressed)} >>\nstream\n".encode()
            )
            buf.extend(compressed)
            buf.extend(b"\nendstream\nendobj\n")

            # Content stream
            offsets.append(len(buf))
            content = f"q {w} 0 0 {h} 0 0 cm /Img Do Q".encode()
            buf.extend(
                f"{content_obj} 0 obj\n<< /Length {len(content)} >>\nstream\n".encode()
            )
            buf.extend(content)
            buf.extend(b"\nendstream\nendobj\n")

            obj_num += 3

        # Xref
        xref_off = len(buf)
        total_objs = len(offsets) + 1
        buf.extend(f"xref\n0 {total_objs}\n".encode())
        buf.extend(b"0000000000 65535 f \n")
        for off in offsets:
            buf.extend(f"{off:010d} 00000 n \n".encode())

        buf.extend(
            f"trailer\n<< /Size {total_objs} /Root 1 0 R >>\n"
            f"startxref\n{xref_off}\n%%EOF\n".encode()
        )

        return bytes(buf)

    @property
    def page_count(self):
        return len(self._pages)
