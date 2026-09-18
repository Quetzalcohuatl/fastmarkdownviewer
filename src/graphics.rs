//! Native renderer startup, including Windows machines without usable OpenGL.

/// Start the viewer, retrying graphics initialization before creating the app.
///
/// On Windows, OpenGL startup failures fall back to Direct3D 12, then Windows'
/// software rasterizer (WARP). The app factory runs at most once, so retrying
/// cannot reload or overwrite a user's session after the viewer has started.
///
/// # Errors
/// Returns the final renderer error, or any application/window-system error.
pub fn run_native(
    app_name: &str,
    mut options: eframe::NativeOptions,
    creator: eframe::AppCreator<'_>,
) -> eframe::Result {
    options.renderer = default_renderer();
    #[cfg(target_os = "windows")]
    {
        windows::run(app_name, options, creator)
    }
    #[cfg(not(target_os = "windows"))]
    {
        eframe::run_native(app_name, options, creator)
    }
}

#[cfg(feature = "renderer-glow")]
const fn default_renderer() -> eframe::Renderer {
    eframe::Renderer::Glow
}

#[cfg(all(feature = "renderer-wgpu", not(feature = "renderer-glow")))]
const fn default_renderer() -> eframe::Renderer {
    eframe::Renderer::Wgpu
}

#[cfg(not(any(feature = "renderer-glow", feature = "renderer-wgpu")))]
compile_error!("enable renderer-glow or renderer-wgpu");

#[cfg(target_os = "windows")]
mod windows {
    use std::{cell::RefCell, sync::Arc};

    #[derive(Clone, Copy, Debug)]
    enum Backend {
        Primary,
        Direct3d,
        Software,
    }

    pub(super) fn run(
        name: &str,
        mut options: eframe::NativeOptions,
        creator: eframe::AppCreator<'_>,
    ) -> eframe::Result {
        // Diagnostic override: normal users always get automatic fallback.
        let backends: &[Backend] = match std::env::var("FMV_GRAPHICS").as_deref() {
            Ok("opengl") => &[Backend::Primary],
            Ok("direct3d") => &[Backend::Direct3d, Backend::Software],
            Ok("software") => &[Backend::Software],
            _ => &[Backend::Primary, Backend::Direct3d, Backend::Software],
        };
        let creator = RefCell::new(Some(creator));
        for (index, &backend) in backends.iter().enumerate() {
            let mut attempt = options.clone();
            // NativeOptions::clone intentionally omits these one-shot hooks.
            attempt.event_loop_builder = options.event_loop_builder.take();
            attempt.window_builder = options.window_builder.take();
            if !matches!(backend, Backend::Primary)
                || matches!(attempt.renderer, eframe::Renderer::Wgpu)
            {
                configure_direct3d(&mut attempt, matches!(backend, Backend::Software));
            }
            let result = eframe::run_native(
                name,
                attempt,
                Box::new(|context| {
                    if let Some(state) = &context.wgpu_render_state {
                        let info = state.adapter.get_info();
                        eprintln!(
                            "FMV_RENDERER={:?} adapter={} device={:?}",
                            info.backend, info.name, info.device_type
                        );
                    } else {
                        eprintln!("FMV_RENDERER=OpenGL");
                    }
                    creator
                        .borrow_mut()
                        .take()
                        .expect("app factory called once")(context)
                }),
            );
            let retry = result.as_ref().is_err_and(|error| {
                can_retry(
                    error,
                    creator.borrow().is_none(),
                    index + 1 < backends.len(),
                )
            });
            if !retry {
                return result;
            }
            if let Err(error) = result {
                eprintln!(
                    "Graphics startup ({backend:?}) failed: {error}. Trying the next renderer."
                );
            }
        }
        unreachable!("the final attempt returns its result")
    }

    fn can_retry(error: &eframe::Error, app_created: bool, has_next: bool) -> bool {
        if app_created || !has_next {
            return false;
        }
        match error {
            #[cfg(feature = "renderer-glow")]
            eframe::Error::OpenGL(_)
            | eframe::Error::Glutin(_)
            | eframe::Error::NoGlutinConfigs(..) => true,
            eframe::Error::Wgpu(_) => true,
            _ => false,
        }
    }

    fn configure_direct3d(options: &mut eframe::NativeOptions, software: bool) {
        use eframe::egui_wgpu::{WgpuSetup, WgpuSetupCreateNew};

        options.renderer = eframe::Renderer::Wgpu;
        let mut setup = WgpuSetupCreateNew::without_display_handle();
        setup.instance_descriptor.backends = wgpu::Backends::DX12;
        // FXC is supplied by Windows; no extra shader-compiler DLL is required.
        setup
            .instance_descriptor
            .backend_options
            .dx12
            .shader_compiler = wgpu::Dx12Compiler::Fxc;
        if software {
            setup.native_adapter_selector = Some(Arc::new(|adapters, surface| {
                let adapter = adapters.iter().find(|adapter| {
                    adapter.get_info().device_type == wgpu::DeviceType::Cpu
                        && surface.is_none_or(|surface| adapter.is_surface_supported(surface))
                });
                adapter
                    .cloned()
                    .ok_or_else(|| "Windows software rendering is unavailable".into())
            }));
        }
        options.wgpu_options.wgpu_setup = WgpuSetup::CreateNew(setup);
    }

    #[cfg(all(test, feature = "renderer-glow"))]
    mod tests {
        use super::*;

        #[test]
        fn fallback_is_only_for_graphics_failure_before_app_creation() {
            let error = eframe::Error::OpenGL("OpenGL unavailable".to_owned().into());
            assert!(can_retry(&error, false, true));
            assert!(!can_retry(&error, true, true));
            assert!(!can_retry(&error, false, false));
            let app_error =
                eframe::Error::AppCreation(std::io::Error::other("document error").into());
            assert!(!can_retry(&app_error, false, true));
        }
    }
}
