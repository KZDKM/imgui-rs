use std::{
    cell::RefCell,
    ffi::{c_void, CStr},
    os::raw::{c_char, c_int},
};

use crate::{PlatformIo, Viewport};

thread_local!(
    pub(crate) static PLATFORM_VIEWPORT_CONTEXT: RefCell<Option<crate::PlatformViewportContext>> = const { RefCell::new(None) });

thread_local!(
    pub(crate) static RENDERER_VIEWPORT_CONTEXT: RefCell<Option<crate::RendererViewportContext>> = const { RefCell::new(None) });

/// Trait holding functions needed when the platform integration supports viewports.
///
/// Register it via [`Context::set_platform_backend()`](crate::context::Context::set_platform_backend())
pub trait PlatformViewportBackend: 'static {
    /// Called by imgui when a new [`Viewport`] is created.
    ///
    /// # Notes
    /// This function should initiate the creation of a platform window.
    /// The window should be invisible.
    fn create_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui when a [`Viewport`] is about to be destroyed.
    ///
    /// # Notes
    /// This function should initiate the destruction of the platform window.
    fn destroy_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui to make a [`Viewport`] visible.
    fn show_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui to reposition a [`Viewport`].
    ///
    /// # Notes
    /// `pos` specifies the position of the windows content area (excluding title bar etc.)
    fn set_window_pos(&mut self, viewport: &mut Viewport, pos: [f32; 2]);
    /// Called by imgui to get the position of a [`Viewport`].
    ///
    /// # Notes
    /// You should return the position of the window's content area (excluding title bar etc.)
    fn get_window_pos(&mut self, viewport: &mut Viewport) -> [f32; 2];
    /// Called by imgui to set the size of a [`Viewport`].
    ///
    /// # Notes
    /// `size` specifies the size of the window's content area (excluding title bar etc.)
    fn set_window_size(&mut self, viewport: &mut Viewport, size: [f32; 2]);
    /// Called by imgui to get the size of a [`Viewport`].
    ///
    /// # Notes
    /// you should return the size of the window's content area (excluding title bar etc.)
    fn get_window_size(&mut self, viewport: &mut Viewport) -> [f32; 2];
    /// Called by imgui to make a [`Viewport`] steal the focus.
    fn set_window_focus(&mut self, viewport: &mut Viewport);
    /// Called by imgui to query whether a [`Viewport`] is in focus.
    fn get_window_focus(&mut self, viewport: &mut Viewport) -> bool;
    /// Called by imgui to query whether a [`Viewport`] is minimized.
    fn get_window_minimized(&mut self, viewport: &mut Viewport) -> bool;
    /// Called by imgui to set a [`Viewport`] title.
    fn set_window_title(&mut self, viewport: &mut Viewport, title: &str);
    /// Called by imgui to set the opacity of an entire [`Viewport`].
    ///
    /// If your backend does not support opactiy, it is safe to just do nothing in this function.
    fn set_window_alpha(&mut self, viewport: &mut Viewport, alpha: f32);
    fn update_window(&mut self, viewport: &mut Viewport);
    fn render_window(&mut self, viewport: &mut Viewport);
    fn swap_buffers(&mut self, viewport: &mut Viewport);
    fn create_vk_surface(
        &mut self,
        viewport: &mut Viewport,
        instance: u64,
        out_surface: &mut u64,
    ) -> i32;
}

#[inline]
fn get_platform(
    ctx: &mut Option<PlatformViewportContext>,
) -> &mut Box<dyn PlatformViewportBackend> {
    &mut ctx.as_mut().unwrap().backend
}

#[inline]
fn get_renderer(
    ctx: &mut Option<RendererViewportContext>,
) -> &mut Box<dyn RendererViewportBackend> {
    &mut ctx.as_mut().unwrap().backend
}

pub(crate) extern "C" fn platform_create_window(viewport: *mut Viewport) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx: &mut Option<crate::PlatformViewportContext>| {
        get_platform(ctx).create_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_destroy_window(viewport: *mut Viewport) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).destroy_window(unsafe { &mut *viewport });
    })
}

pub(crate) extern "C" fn platform_show_window(viewport: *mut Viewport) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).show_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_set_window_pos(viewport: *mut Viewport, pos: sys::ImVec2) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).set_window_pos(unsafe { &mut *viewport }, [pos.x, pos.y]);
    })
}
pub(crate) extern "C" fn platform_get_window_pos(
    viewport: *mut Viewport,
    out_pos: *mut sys::ImVec2,
) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        let pos = get_platform(ctx).get_window_pos(unsafe { &mut *viewport });
        unsafe {
            *out_pos = sys::ImVec2::new(pos[0], pos[1]);
        }
    })
}
pub(crate) extern "C" fn platform_set_window_size(viewport: *mut Viewport, size: sys::ImVec2) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).set_window_size(unsafe { &mut *viewport }, [size.x, size.y]);
    })
}
pub(crate) extern "C" fn platform_get_window_size(
    viewport: *mut Viewport,
    out_size: *mut sys::ImVec2,
) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx: &mut Option<crate::PlatformViewportContext>| {
        let size = get_platform(ctx).get_window_size(unsafe { &mut *viewport });
        unsafe {
            *out_size = sys::ImVec2::new(size[0], size[1]);
        }
    })
}
pub(crate) extern "C" fn platform_set_window_focus(viewport: *mut Viewport) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx: &mut Option<crate::PlatformViewportContext>| {
        get_platform(ctx).set_window_focus(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_get_window_focus(viewport: *mut Viewport) -> bool {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx: &mut Option<crate::PlatformViewportContext>| {
        get_platform(ctx).get_window_focus(unsafe { &mut *viewport })
    })
}
pub(crate) extern "C" fn platform_get_window_minimized(viewport: *mut Viewport) -> bool {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx: &mut Option<crate::PlatformViewportContext>| {
        get_platform(ctx).get_window_minimized(unsafe { &mut *viewport })
    })
}
pub(crate) extern "C" fn platform_set_window_title(viewport: *mut Viewport, title: *const c_char) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        let title = unsafe { CStr::from_ptr(title).to_str().unwrap() };
        get_platform(ctx).set_window_title(unsafe { &mut *viewport }, title);
    })
}
pub(crate) extern "C" fn platform_set_window_alpha(viewport: *mut Viewport, alpha: f32) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).set_window_alpha(unsafe { &mut *viewport }, alpha);
    })
}
pub(crate) extern "C" fn platform_update_window(viewport: *mut Viewport) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).update_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_render_window(viewport: *mut Viewport, _arg: *mut c_void) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).render_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_swap_buffers(viewport: *mut Viewport, _arg: *mut c_void) {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).swap_buffers(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn platform_create_vk_surface(
    viewport: *mut Viewport,
    instance: u64,
    _arg: *const c_void,
    out_surface: *mut u64,
) -> c_int {
    PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_platform(ctx).create_vk_surface(unsafe { &mut *viewport }, instance, unsafe {
            &mut *out_surface
        })
    })
}

/// Just holds a [`PlatformViewportBackend`].
pub(crate) struct PlatformViewportContext {
    pub(crate) backend: Box<dyn PlatformViewportBackend>,
}

/// Trait that holds optional functions for a rendering backend to support multiple viewports.
///
/// It is completely fine to not use this Backend at all, as all functions are optional.
pub trait RendererViewportBackend: 'static {
    /// Called after [`PlatformViewportBackend::create_window()`].
    fn create_window(&mut self, viewport: &mut Viewport);
    /// Called before [`PlatformViewportBackend::destroy_window()`].
    fn destroy_window(&mut self, viewport: &mut Viewport);
    /// Called after [`PlatformViewportBackend::set_window_size()`].
    fn set_window_size(&mut self, viewport: &mut Viewport, size: [f32; 2]);
    fn render_window(&mut self, viewport: &mut Viewport);
    fn swap_buffers(&mut self, viewport: &mut Viewport);
}

pub(crate) extern "C" fn renderer_create_window(viewport: *mut Viewport) {
    RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_renderer(ctx).create_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn renderer_destroy_window(viewport: *mut Viewport) {
    RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_renderer(ctx).destroy_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn renderer_set_window_size(viewport: *mut Viewport, size: sys::ImVec2) {
    RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_renderer(ctx).set_window_size(unsafe { &mut *viewport }, [size.x, size.y]);
    })
}
pub(crate) extern "C" fn renderer_render_window(viewport: *mut Viewport, _arg: *mut c_void) {
    RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_renderer(ctx).render_window(unsafe { &mut *viewport });
    })
}
pub(crate) extern "C" fn renderer_swap_buffers(viewport: *mut Viewport, _arg: *mut c_void) {
    RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|ctx| {
        get_renderer(ctx).swap_buffers(unsafe { &mut *viewport });
    })
}

/// Just holds a [`RendererViewportBackend`].
pub(crate) struct RendererViewportContext {
    pub(crate) backend: Box<dyn RendererViewportBackend>,
}

/// Describes a monitor that can be used by ImGui.
#[repr(C)]
pub struct PlatformMonitor {
    /// Position of the monitor on the virtual desktop.
    pub main_pos: [f32; 2],
    /// Size of the monitor on the virtual desktop.
    pub main_size: [f32; 2],
    /// Working position of the monitor, should exclude task bar etc.
    ///
    /// Set to `main_pos` if not known.
    pub work_pos: [f32; 2],
    /// Working size of the monitor, should exclude task bar etc.
    ///
    /// Set to `work_size` if not known.
    pub work_size: [f32; 2],
    pub dpi_scale: f32,

    pub platform_handle: *mut c_void,
}

#[test]
#[cfg(test)]
fn test_platform_monitor_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<PlatformMonitor>(),
        mem::size_of::<sys::ImGuiPlatformMonitor>()
    );
    assert_eq!(
        mem::align_of::<PlatformMonitor>(),
        mem::align_of::<sys::ImGuiPlatformMonitor>()
    );
    use sys::ImGuiPlatformMonitor;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(PlatformMonitor, $l),
                memoffset::offset_of!(ImGuiPlatformMonitor, $r)
            );
        };
    }

    assert_field_offset!(main_pos, MainPos);
    assert_field_offset!(main_size, MainSize);
    assert_field_offset!(work_pos, WorkPos);
    assert_field_offset!(work_size, WorkSize);
    assert_field_offset!(dpi_scale, DpiScale);
    assert_field_offset!(platform_handle, PlatformHandle);
}

extern "C" {
    pub(crate) fn ImGuiPlatformIO_Set_Platform_GetWindowPos(
        pio: *mut PlatformIo,
        func: extern "C" fn(*mut Viewport, *mut sys::ImVec2),
    );
    pub(crate) fn ImGuiPlatformIO_Set_Platform_GetWindowSize(
        pio: *mut PlatformIo,
        func: extern "C" fn(*mut Viewport, *mut sys::ImVec2),
    );
}
