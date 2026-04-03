use std::time::{SystemTime, UNIX_EPOCH};
use tiny_skia::Pixmap;

/// Show a plot in an interactive window.
///
/// On macOS, winit requires the event loop on the main thread, which conflicts
/// with Python's runtime. We use a fallback approach: save to a temp file and
/// open with the system viewer.
///
/// In the future, a native window (winit + softbuffer) could be used for
/// non-Python contexts or with proper main-thread integration.
pub fn show_pixmap(pixmap: &Pixmap) -> Result<(), String> {
    // Try native window first; fall back to system viewer on failure.
    #[cfg(not(target_os = "linux"))]
    {
        // On macOS (and Windows), opening a winit event loop from a non-main
        // thread (as Python calls us) typically panics. Use fallback.
        show_with_system_viewer(pixmap)
    }

    #[cfg(target_os = "linux")]
    {
        match show_native_window(pixmap) {
            Ok(()) => Ok(()),
            Err(_) => show_with_system_viewer(pixmap),
        }
    }
}

/// Save the pixmap to a temp file and open it with the OS default viewer.
fn show_with_system_viewer(pixmap: &Pixmap) -> Result<(), String> {
    // Generate unique temp filename to avoid race conditions
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    let path = format!("{}/rustplotlib_{}_{}.png", std::env::temp_dir().display(), pid, timestamp);

    pixmap
        .save_png(&path)
        .map_err(|e| format!("Failed to save temp PNG: {}", e))?;

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open viewer: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open viewer: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &path])
            .spawn()
            .map_err(|e| format!("Failed to open viewer: {}", e))?;
    }

    // Clean up after a short delay (spawn a thread to delete after 5 seconds)
    let path_clone = path.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        let _ = std::fs::remove_file(&path_clone);
    });

    Ok(())
}

/// Show the pixmap in a native winit window with softbuffer.
/// This only works when called from the main thread.
#[cfg(target_os = "linux")]
fn show_native_window(pixmap: &Pixmap) -> Result<(), String> {
    use std::num::NonZeroU32;
    use std::sync::Arc;
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoop};
    use winit::keyboard::{Key, NamedKey};
    use winit::window::{Window, WindowAttributes, WindowId};

    struct PlotWindow {
        window: Option<Arc<Window>>,
        surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
        pixmap_w: u32,
        pixmap_h: u32,
        pixel_data: Vec<u32>,
    }

    impl ApplicationHandler for PlotWindow {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_none() {
                let attrs = WindowAttributes::default()
                    .with_title("RustPlot")
                    .with_inner_size(winit::dpi::PhysicalSize::new(
                        self.pixmap_w,
                        self.pixmap_h,
                    ));
                let window = Arc::new(
                    event_loop
                        .create_window(attrs)
                        .expect("Failed to create window"),
                );
                let context = softbuffer::Context::new(window.clone())
                    .expect("Failed to create softbuffer context");
                let surface = softbuffer::Surface::new(&context, window.clone())
                    .expect("Failed to create softbuffer surface");
                self.window = Some(window.clone());
                self.surface = Some(surface);
                window.request_redraw();
            }
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _id: WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.logical_key == Key::Named(NamedKey::Escape) {
                        event_loop.exit();
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(surface) = self.surface.as_mut() {
                        let w = match NonZeroU32::new(self.pixmap_w) {
                            Some(w) => w,
                            None => return,  // skip rendering if dimensions are 0
                        };
                        let h = match NonZeroU32::new(self.pixmap_h) {
                            Some(h) => h,
                            None => return,
                        };
                        if let Err(_) = surface.resize(w, h) {
                            return;
                        }
                        let mut buf = match surface.buffer_mut() {
                            Ok(b) => b,
                            Err(_) => return,
                        };
                        buf.copy_from_slice(&self.pixel_data);
                        let _ = buf.present();
                    }
                }
                _ => {}
            }
        }
    }

    // Pre-convert pixels from tiny-skia premultiplied RGBA to softbuffer 0x00RRGGBB
    let pixel_data: Vec<u32> = pixmap
        .pixels()
        .iter()
        .map(|px| {
            let a = px.alpha() as u32;
            if a > 0 {
                let r = (px.red() as u32 * 255 / a).min(255);
                let g = (px.green() as u32 * 255 / a).min(255);
                let b = (px.blue() as u32 * 255 / a).min(255);
                (r << 16) | (g << 8) | b
            } else {
                0xFFFFFF
            }
        })
        .collect();

    let event_loop = EventLoop::new().map_err(|e| format!("EventLoop error: {}", e))?;
    let mut app = PlotWindow {
        window: None,
        surface: None,
        pixmap_w: pixmap.width(),
        pixmap_h: pixmap.height(),
        pixel_data,
    };
    event_loop
        .run_app(&mut app)
        .map_err(|e| format!("Event loop error: {}", e))?;
    Ok(())
}
