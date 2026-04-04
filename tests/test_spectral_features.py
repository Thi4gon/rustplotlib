"""Tests for spectral/signal features: specgram, acorr, xcorr, psd, spectrum plots,
hist2d, semilogx, semilogy, loglog, and subplot_mosaic string support."""

import pytest
import numpy as np
import os
import tempfile

from rustplotlib import pyplot as plt


class TestSpecgram:
    """Test ax.specgram() and plt.specgram()."""

    def setup_method(self):
        plt.close()

    def test_specgram_basic(self):
        np.random.seed(42)
        x = np.sin(2 * np.pi * 50 * np.linspace(0, 1, 1024)) + np.random.randn(1024) * 0.5
        fig, ax = plt.subplots()
        spec, freqs, times = ax.specgram(x, NFFT=256, Fs=1024)
        assert spec.shape[0] == 129  # NFFT//2 + 1
        assert len(freqs) == 129
        assert len(times) > 0
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_specgram_module_level(self):
        np.random.seed(42)
        x = np.random.randn(512)
        plt.figure()
        spec, freqs, times = plt.specgram(x, NFFT=128, Fs=256, noverlap=64)
        assert spec.shape[0] == 65
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestAcorr:
    """Test ax.acorr() and plt.acorr()."""

    def setup_method(self):
        plt.close()

    def test_acorr_basic(self):
        np.random.seed(42)
        x = np.random.randn(100)
        fig, ax = plt.subplots()
        lags, acorr_vals = ax.acorr(x, maxlags=20)
        assert len(lags) == 41  # -20 to +20
        assert len(acorr_vals) == 41
        # Autocorrelation at lag 0 should be 1.0
        mid = len(lags) // 2
        assert abs(acorr_vals[mid] - 1.0) < 1e-6
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_acorr_default_maxlags(self):
        np.random.seed(42)
        x = np.random.randn(50)
        fig, ax = plt.subplots()
        lags, acorr_vals = ax.acorr(x)
        assert len(lags) == 2 * 49 + 1  # maxlags = len(x) - 1

    def test_acorr_module_level(self):
        np.random.seed(42)
        x = np.random.randn(100)
        plt.figure()
        lags, acorr_vals = plt.acorr(x, maxlags=10)
        assert len(lags) == 21


class TestXcorr:
    """Test ax.xcorr() and plt.xcorr()."""

    def setup_method(self):
        plt.close()

    def test_xcorr_basic(self):
        np.random.seed(42)
        x = np.random.randn(100)
        y = np.random.randn(100)
        fig, ax = plt.subplots()
        lags, xcorr_vals = ax.xcorr(x, y, maxlags=20)
        assert len(lags) == 41
        assert len(xcorr_vals) == 41
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_xcorr_self_correlation(self):
        np.random.seed(42)
        x = np.random.randn(100)
        fig, ax = plt.subplots()
        lags, xcorr_vals = ax.xcorr(x, x, maxlags=20)
        # Cross-correlation with self at lag 0 should be ~1.0
        mid = len(lags) // 2
        assert abs(xcorr_vals[mid] - 1.0) < 1e-6

    def test_xcorr_module_level(self):
        np.random.seed(42)
        x = np.random.randn(100)
        y = np.random.randn(100)
        plt.figure()
        lags, xcorr_vals = plt.xcorr(x, y, maxlags=15)
        assert len(lags) == 31


class TestPsd:
    """Test ax.psd() and plt.psd()."""

    def setup_method(self):
        plt.close()

    def test_psd_basic(self):
        np.random.seed(42)
        x = np.random.randn(1024)
        fig, ax = plt.subplots()
        psd_vals, freqs = ax.psd(x, NFFT=256, Fs=1024)
        assert len(psd_vals) == 129
        assert len(freqs) == 129
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_psd_module_level(self):
        np.random.seed(42)
        x = np.random.randn(512)
        plt.figure()
        psd_vals, freqs = plt.psd(x, NFFT=128, Fs=256)
        assert len(psd_vals) == 65


class TestMagnitudeSpectrum:
    """Test ax.magnitude_spectrum() and plt.magnitude_spectrum()."""

    def setup_method(self):
        plt.close()

    def test_magnitude_spectrum_basic(self):
        np.random.seed(42)
        x = np.sin(2 * np.pi * 10 * np.linspace(0, 1, 256))
        fig, ax = plt.subplots()
        spectrum, freqs = ax.magnitude_spectrum(x, Fs=256)
        assert len(spectrum) == len(freqs)
        assert len(freqs) == len(x) // 2 + 1
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_magnitude_spectrum_module_level(self):
        np.random.seed(42)
        x = np.random.randn(128)
        plt.figure()
        spectrum, freqs = plt.magnitude_spectrum(x, Fs=100)
        assert len(spectrum) == 65


class TestAngleSpectrum:
    """Test ax.angle_spectrum() and plt.angle_spectrum()."""

    def setup_method(self):
        plt.close()

    def test_angle_spectrum_basic(self):
        np.random.seed(42)
        x = np.random.randn(128)
        fig, ax = plt.subplots()
        spectrum, freqs = ax.angle_spectrum(x, Fs=100)
        assert len(spectrum) == len(freqs)
        # Angles should be in [-pi, pi]
        assert np.all(spectrum >= -np.pi - 1e-10)
        assert np.all(spectrum <= np.pi + 1e-10)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_angle_spectrum_module_level(self):
        np.random.seed(42)
        x = np.random.randn(64)
        plt.figure()
        spectrum, freqs = plt.angle_spectrum(x)
        assert len(spectrum) == 33


class TestPhaseSpectrum:
    """Test ax.phase_spectrum() and plt.phase_spectrum()."""

    def setup_method(self):
        plt.close()

    def test_phase_spectrum_is_alias(self):
        np.random.seed(42)
        x = np.random.randn(128)
        fig, ax = plt.subplots()
        spec_angle, freqs_angle = ax.angle_spectrum(x, Fs=100)

        plt.close()
        fig2, ax2 = plt.subplots()
        spec_phase, freqs_phase = ax2.phase_spectrum(x, Fs=100)

        np.testing.assert_array_almost_equal(spec_angle, spec_phase)
        np.testing.assert_array_almost_equal(freqs_angle, freqs_phase)

    def test_phase_spectrum_module_level(self):
        np.random.seed(42)
        x = np.random.randn(64)
        plt.figure()
        spectrum, freqs = plt.phase_spectrum(x)
        assert len(spectrum) == 33


class TestCohere:
    """Test ax.cohere() and plt.cohere()."""

    def setup_method(self):
        plt.close()

    def test_cohere_basic(self):
        np.random.seed(42)
        x = np.random.randn(512)
        y = np.random.randn(512)
        fig, ax = plt.subplots()
        coh, freqs = ax.cohere(x, y, NFFT=256, Fs=512)
        assert len(coh) == 129
        assert len(freqs) == 129
        # Coherence values should be non-negative
        assert np.all(coh >= -1e-10)
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_cohere_self(self):
        np.random.seed(42)
        x = np.random.randn(256)
        fig, ax = plt.subplots()
        coh, freqs = ax.cohere(x, x, NFFT=256, Fs=256)
        # Self-coherence should be ~1.0 everywhere
        assert np.all(coh > 0.99)

    def test_cohere_module_level(self):
        np.random.seed(42)
        x = np.random.randn(512)
        y = np.random.randn(512)
        plt.figure()
        coh, freqs = plt.cohere(x, y, NFFT=256)
        assert len(coh) == 129


class TestCsd:
    """Test ax.csd() and plt.csd()."""

    def setup_method(self):
        plt.close()

    def test_csd_basic(self):
        np.random.seed(42)
        x = np.random.randn(512)
        y = np.random.randn(512)
        fig, ax = plt.subplots()
        Pxy, freqs = ax.csd(x, y, NFFT=256, Fs=512)
        assert len(Pxy) == 129
        assert len(freqs) == 129
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_csd_module_level(self):
        np.random.seed(42)
        x = np.random.randn(512)
        y = np.random.randn(512)
        plt.figure()
        Pxy, freqs = plt.csd(x, y, NFFT=256)
        assert len(Pxy) == 129


class TestHist2d:
    """Test ax.hist2d() and plt.hist2d()."""

    def setup_method(self):
        plt.close()

    def test_hist2d_basic(self):
        np.random.seed(42)
        x = np.random.randn(1000)
        y = np.random.randn(1000)
        fig, ax = plt.subplots()
        H, xedges, yedges = ax.hist2d(x, y, bins=20)
        assert H.shape == (20, 20)
        assert len(xedges) == 21
        assert len(yedges) == 21
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_hist2d_tuple_bins(self):
        np.random.seed(42)
        x = np.random.randn(500)
        y = np.random.randn(500)
        fig, ax = plt.subplots()
        H, xedges, yedges = ax.hist2d(x, y, bins=(15, 25))
        assert H.shape == (15, 25)
        assert len(xedges) == 16
        assert len(yedges) == 26

    def test_hist2d_module_level(self):
        np.random.seed(42)
        x = np.random.randn(500)
        y = np.random.randn(500)
        plt.figure()
        H, xedges, yedges = plt.hist2d(x, y, bins=10)
        assert H.shape == (10, 10)


class TestSemilogx:
    """Test ax.semilogx() and plt.semilogx()."""

    def setup_method(self):
        plt.close()

    def test_semilogx_basic(self):
        fig, ax = plt.subplots()
        x = [1, 10, 100, 1000]
        y = [1, 2, 3, 4]
        result = ax.semilogx(x, y)
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_semilogx_module_level(self):
        plt.figure()
        result = plt.semilogx([1, 10, 100], [1, 2, 3])
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestSemilogy:
    """Test ax.semilogy() and plt.semilogy()."""

    def setup_method(self):
        plt.close()

    def test_semilogy_basic(self):
        fig, ax = plt.subplots()
        x = [1, 2, 3, 4]
        y = [1, 10, 100, 1000]
        result = ax.semilogy(x, y)
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_semilogy_module_level(self):
        plt.figure()
        result = plt.semilogy([1, 2, 3], [10, 100, 1000])
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestLoglog:
    """Test ax.loglog() and plt.loglog()."""

    def setup_method(self):
        plt.close()

    def test_loglog_basic(self):
        fig, ax = plt.subplots()
        x = [1, 10, 100, 1000]
        y = [1, 10, 100, 1000]
        result = ax.loglog(x, y)
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            fig.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)

    def test_loglog_module_level(self):
        plt.figure()
        result = plt.loglog([1, 10, 100], [10, 100, 1000])
        assert result is not None
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
            plt.savefig(f.name)
            assert os.path.getsize(f.name) > 0
            os.unlink(f.name)


class TestSubplotMosaicString:
    """Test subplot_mosaic with string layout."""

    def setup_method(self):
        plt.close()

    def test_mosaic_string_basic(self):
        fig, axes = plt.subplot_mosaic("AB\nCC")
        assert 'A' in axes
        assert 'B' in axes
        assert 'C' in axes
        assert len(axes) == 3

    def test_mosaic_string_single_row(self):
        fig, axes = plt.subplot_mosaic("ABC")
        assert len(axes) == 3
        assert 'A' in axes
        assert 'B' in axes
        assert 'C' in axes

    def test_mosaic_string_with_dot(self):
        fig, axes = plt.subplot_mosaic("A.\n.B")
        assert 'A' in axes
        assert 'B' in axes
        assert '.' not in axes
        assert len(axes) == 2

    def test_mosaic_list_still_works(self):
        fig, axes = plt.subplot_mosaic([['A', 'B'], ['C', 'C']])
        assert 'A' in axes
        assert 'B' in axes
        assert 'C' in axes
        assert len(axes) == 3
