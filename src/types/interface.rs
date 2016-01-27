//! types/interface
//! Contains all structs used for initializing wlc.
//! You will only need this module for invoking
//! `rustwlc::init`.

#![feature(libc)]
extern crate libc;

use std::ffi;
use std::ffi::{CString};

use super::*;
use super::super::handle::WlcHandle;

/// Function signature of some standard Wwlc callbacks
pub type InterfaceHandler = Option<extern "C" fn(WlcHandle) -> ()>;

/// Represents the wlc callback interface.
/// wlc initialization involves registering
/// a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
#[no_mangle]
pub struct WlcInterface {
    pub output: OutputInterface,
    pub view: ViewInterface,
    pub keyboard: KeyboardInterface,
    pub pointer: PointerInterface,
    pub touch: TouchInterface,
    pub compositor: CompositorInterface,
    pub input: InputInterface
}

/// Represents window callbacks
#[repr(C)]
pub struct OutputInterface {
    /// Output was created
    pub created: Option<extern "C" fn(handle: WlcHandle) -> bool>,
    /// Output lost or destroyed
    pub destroyed: InterfaceHandler,
    pub focus: Option<extern "C" fn(handle: WlcHandle, focused: bool) -> ()>,
    /// Output resolution changed
    pub resolution: Option<extern "C" fn(handle: WlcHandle, old_size: Size, new_size: Size) -> ()>,
    pub render: RenderInterface,
}

/// Represents global rendering callbacks
#[repr(C)]
pub struct RenderInterface {
    /// Pre render hook
    pub pre: InterfaceHandler,
    /// Post render hook
    pub post: InterfaceHandler,
}

/// Represents window viewing callbacks
#[repr(C)]
pub struct ViewInterface {
    /// View was created. Return false if you want to destroy the view
    /// (e.g. failed to allocated data related to the view)
    pub created: Option<extern "C" fn(handle: WlcHandle) -> bool>,
    /// View was destroyed
    pub destroyed: InterfaceHandler,
    /// View lost or got focus
    pub focus: Option<extern "C" fn(handle: WlcHandle, focused: bool) -> ()>,
    /// View was moved to to output
    pub move_to_output: Option<extern "C" fn(current: WlcHandle, from_output: WlcHandle, to_output: WlcHandle) -> ()>,
    pub request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
pub struct RequestInterface {
    /// Request to set given geometry to view. Apply using
    /// wlc_view_set_geometry (handle.set_geometry) to agree.
    pub geometry: Option<extern "C" fn(handle: WlcHandle, geometry: Geometry) -> ()>,
    /// Request to disable or enable the given state for a view.
    /// Apply using wlc_view_set_state to agree.
    pub state: Option<extern "C" fn(current: WlcHandle, state: ViewState, handled: bool) -> ()>,
    /// View requests to move itself. Start an interactive move to agree.
    pub move_: Option<extern "C" fn(handle: WlcHandle, destination: Point) -> ()>,
    /// Request to resize itself with the given edges.
    /// Start and interactive move to agree
    pub resize: Option<extern "C" fn(handle: WlcHandle, edge: ResizeEdge, location: Point) -> ()>,

    /// View rendering hooks
    pub render: RenderInterface,
}

/// Represents keyboard press callbacks
#[repr(C)]
pub struct KeyboardInterface {
    /// Key event was triggered, handle.0 will be zero if there was no focus
    /// Return true to prevent sending the event to clients.
    pub key: Option<extern "C" fn(handle: WlcHandle, time: u32, mods: KeyboardModifiers, key: u32, state: KeyState) -> bool>,
}

/// Represents mouse input callbacks
#[repr(C)]
pub struct PointerInterface {
    /// Button event was triggered, handle.0 will be zero if there was no
    /// focus. Return true to prevent sending the event to clients.
    pub button: Option<extern "C" fn(hande: WlcHandle, button: libc::c_uint, mods: KeyboardModifiers, time: u32, state: ButtonState, point: Point) -> bool>,
    /// Scroll event was triggered, view handle will be zero if there was
    /// no focus. Return true to prevent sending the event to clients.
    pub scroll: Option<extern "C" fn(handle: WlcHandle, button: u32, mods: KeyboardModifiers, axis: ScrollAxis, amount: [u64; 2]) -> bool>,
    /// Mouse was moved, handle.0 will be zero if there was no focus.
    /// Use wlc_pointer_set_position to agree. Return true to prevent
    /// sending event to clients.
    pub motion: Option<extern "C" fn(heights: WlcHandle, dist: u32, point: Point)>,
}

/// Represents touchscreen callbacks
#[repr(C)]
pub struct TouchInterface {
    /// Screen was touched, handle.0 will be zero if there was no focus.
    /// Return true to prevent sending the event to clients.
    pub touch: Option<extern "C" fn(handle: WlcHandle, time: libc::c_uint, mods: KeyboardModifiers, touch: TouchType, key: libc::c_int, point: Point) -> bool>,
}

/// Represents a callback for initializing the callback
#[repr(C)]
pub struct CompositorInterface {
    /// Compositor is ready to accept clients.
    pub ready: Option<extern "C" fn() -> ()>
}

/// Represents experimental callbacks for libinput events
#[repr(C)]
pub struct InputInterface {
    pub created: Option<extern "C" fn(device: LibinputDevice) -> bool>,
    pub destroyed: Option<extern "C" fn(device: LibinputDevice) -> ()>
}
