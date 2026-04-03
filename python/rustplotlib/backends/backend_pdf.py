"""PdfPages — Save multiple figures to a multi-page PDF (or per-page PNGs)."""

import os


class PdfPages:
    """Save multiple figures to a single PDF file.

    Usage:
        with PdfPages('output.pdf') as pdf:
            fig, ax = plt.subplots()
            ax.plot([1, 2, 3])
            pdf.savefig(fig)

            fig2, ax2 = plt.subplots()
            ax2.bar([1, 2], [3, 4])
            pdf.savefig(fig2)
    """

    def __init__(self, filename):
        self.filename = filename
        self.pages = []

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()

    def savefig(self, figure=None, **kwargs):
        """Save a figure as a page."""
        from rustplotlib import pyplot
        if figure is None:
            fig = pyplot._gcf()
        elif hasattr(figure, '_fig'):
            fig = figure._fig
        else:
            fig = figure

        png_bytes = fig.render_to_png_bytes()
        self.pages.append(png_bytes)

    def close(self):
        """Write all pages to the PDF file."""
        if not self.pages:
            return

        if len(self.pages) == 1:
            base, ext = os.path.splitext(self.filename)
            with open(base + '.png', 'wb') as f:
                f.write(self.pages[0])
        else:
            for i, page in enumerate(self.pages):
                base, ext = os.path.splitext(self.filename)
                with open(f"{base}_page{i + 1}.png", 'wb') as f:
                    f.write(page)
