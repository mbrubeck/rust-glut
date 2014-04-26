// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/* automatically generated by rust-bindgen */

use libc::{c_int, c_uint, c_uchar, c_char};
use std::cast;
use std::local_data;
use std::ptr::null;

/* FIXME: global variable glutStrokeRoman */

/* FIXME: global variable glutStrokeMonoRoman */

/* FIXME: global variable glutBitmap9By15 */

/* FIXME: global variable glutBitmap8By13 */

/* FIXME: global variable glutBitmapTimesRoman10 */

/* FIXME: global variable glutBitmapTimesRoman24 */

/* FIXME: global variable glutBitmapHelvetica10 */

/* FIXME: global variable glutBitmapHelvetica12 */

/* FIXME: global variable glutBitmapHelvetica18 */

pub type GLenum = i32;
pub type GLint = i32;
pub type GLfloat = f32;
pub type GLdouble = f64;

pub struct Window(pub c_int);

pub static DOUBLE: c_uint = 2 as c_uint;

pub static ACTIVE_SHIFT: c_int = 1;
pub static ACTIVE_CTRL: c_int = 2;
pub static ACTIVE_ALT: c_int = 4;

// mouse buttons
pub static LEFT_BUTTON: c_int = 0;
pub static MIDDLE_BUTTON: c_int = 1;
pub static RIGHT_BUTTON: c_int = 2;

// mouse button callback state
pub static MOUSE_DOWN: c_int = 0;
pub static MOUSE_UP: c_int = 1;

static WINDOW_WIDTH: GLenum = 102;
static WINDOW_HEIGHT: GLenum = 103;

#[cfg(target_os="linux")]
#[cfg(target_os="android")]
pub static HAVE_PRECISE_MOUSE_WHEEL: bool = false;
#[cfg(target_os="macos")]
pub static HAVE_PRECISE_MOUSE_WHEEL: bool = true;

pub trait DisplayCallback { fn call(&self); }
pub trait KeyboardCallback { fn call(&self, key: c_uchar, x: c_int, y: c_int); }
pub trait MouseCallback { fn call(&self, button: c_int, state: c_int, x: c_int, y: c_int); }
pub trait MotionCallback { fn call(&self, x: c_int, y: c_int); }
pub trait PassiveMotionCallback { fn call(&self, x: c_int, y: c_int); }
pub trait TimerCallback { fn call(&self); }
pub trait ReshapeCallback { fn call(&self, x: c_int, y: c_int); }
pub trait IdleCallback { fn call(&self); }
pub trait MouseWheelCallback { fn call(&self, wheel: c_int, direction: c_int, x: c_int, y: c_int); }

local_data_key!(display_tls_key: ~DisplayCallback:'static)
local_data_key!(keyboard_tls_key: ~KeyboardCallback:'static)
local_data_key!(mouse_tls_key: ~MouseCallback:'static)
local_data_key!(motion_tls_key: ~MotionCallback:'static)
local_data_key!(passive_motion_tls_key: ~PassiveMotionCallback:'static)
local_data_key!(timer_tls_key: ~TimerCallback:'static)
local_data_key!(reshape_tls_key: ~ReshapeCallback:'static)
local_data_key!(idle_tls_key: ~IdleCallback:'static)
local_data_key!(mouse_wheel_tls_key: ~MouseWheelCallback:'static)

pub enum State {
    WindowWidth,
    WindowHeight
}

pub fn init() { 
    unsafe {
        let argc = 0 as c_int;
        let glut = ~"glut";
        glut.to_c_str().with_ref(|command| {
            let argv: (*u8, *u8) = (command as *u8, null());
            let argv_p = cast::transmute(&argv);
            glutInit(&argc, argv_p);
        });
    }
}

pub fn create_window(name: ~str) -> Window {
    unsafe {
        name.to_c_str().with_ref(|bytes| {
            Window(glutCreateWindow(bytes as *c_char))
        })
    }
}

pub fn destroy_window(window: Window) {
    unsafe {
        let Window(i) = window;
        glutDestroyWindow(i);
    }
}

pub fn set_window(window: Window) {
    unsafe {
        let Window(i) = window;
        glutSetWindow(i);
    }
}

pub fn set_window_title(_window: Window, title: &str) {
    unsafe {
        let title = title.to_owned();
        title.to_c_str().with_ref(|bytes| {
            glutSetWindowTitle(bytes as *c_char);
        });
    }
}

pub fn reshape_window(window: Window, width: c_int, height: c_int) {
    unsafe {
        let current_window = glutGetWindow();
        let Window(i) = window;
        glutSetWindow(i);
        glutReshapeWindow(width, height);
        glutSetWindow(current_window);
    }
}

pub extern "C" fn display_callback() {
    local_data::get(display_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call();
        });
    });
}

pub fn display_func(callback: ~DisplayCallback:'static) {
    local_data::set(display_tls_key, callback);
    unsafe {
        glutDisplayFunc(display_callback);
    }
}

pub extern "C" fn keyboard_callback(key: c_uchar, x: c_int, y: c_int) {
    local_data::get(keyboard_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(key, x, y);
        });
    });
}

pub fn keyboard_func(callback: ~KeyboardCallback:'static) {
    local_data::set(keyboard_tls_key, callback);
    unsafe {
        glutKeyboardFunc(keyboard_callback);
    }
}

pub extern "C" fn mouse_callback(button: c_int, state: c_int, x: c_int, y: c_int) {
    local_data::get(mouse_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(button, state, x, y);
        });
    });
}

pub fn mouse_func(callback: ~MouseCallback:'static) {
    local_data::set(mouse_tls_key, callback);
    unsafe {
        glutMouseFunc(mouse_callback);
    }
}

pub extern "C" fn motion_callback(x: c_int, y: c_int) {
    local_data::get(motion_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(x, y);
        });
    });
}

pub fn motion_func(callback: ~MotionCallback:'static) {
    local_data::set(motion_tls_key, callback);
    unsafe {
        glutMotionFunc(motion_callback);
    }
}

pub extern "C" fn passive_motion_callback(x: c_int, y: c_int) {
    local_data::get(passive_motion_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(x, y);
        });
    });
}

pub fn passive_motion_func(callback: ~PassiveMotionCallback:'static) {
    local_data::set(passive_motion_tls_key, callback);
    unsafe {
        glutPassiveMotionFunc(passive_motion_callback);
    }
}

pub extern "C" fn timer_callback(_index: int) {
    local_data::get(timer_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call();
        });
    });
}

pub fn timer_func(msecs: u32, callback: ~TimerCallback:'static) {
    local_data::set(timer_tls_key, callback);
    unsafe {
        glutTimerFunc(msecs, timer_callback, 0);
    }
}

pub extern "C" fn reshape_callback(width: c_int, height: c_int) {
    local_data::get(reshape_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(width, height);
        });
    });
}

pub fn reshape_func(_window: Window, callback: ~ReshapeCallback:'static) {
    local_data::set(reshape_tls_key, callback);
    unsafe {
        glutReshapeFunc(reshape_callback);
    }
}

pub extern "C" fn idle_callback() {
    local_data::get(idle_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call();
        });
    });
}

pub fn idle_func(callback: ~IdleCallback:'static) {
    local_data::set(idle_tls_key, callback);
    unsafe {
        glutIdleFunc(idle_callback);
    }
}

// Mouse wheel handling.
//
// This is not part of the standard, but it's supported by freeglut and our Mac hack.
#[cfg(target_os="linux")]
#[cfg(target_os="android")]
pub extern "C" fn mouse_wheel_callback(wheel: c_int, direction: c_int, x: c_int, y: c_int) {
    local_data::get(mouse_wheel_tls_key, |callback| {
        callback.as_ref().map(|&ref cb| {
            cb.call(wheel, direction, x, y);
        });
    });
}

#[cfg(target_os="linux")]
#[cfg(target_os="android")]
pub fn mouse_wheel_func(callback: ~MouseWheelCallback:'static) {
    local_data::set(mouse_wheel_tls_key, callback);
    unsafe {
        glutMouseWheelFunc(mouse_wheel_callback);
    }
}

#[cfg(target_os="macos")]
pub fn mouse_wheel_func(callback: ~MouseWheelCallback:'static) {
        local_data::set(mouse_wheel_tls_key, callback);
}

#[cfg(target_os="macos")]
pub fn check_loop() {
    unsafe {
        glutCheckLoop();
    }
}

#[cfg(target_os="linux")]
#[cfg(target_os="android")]
pub fn check_loop() {
    unsafe {
        glutMainLoopEvent();
    }
}

pub fn init_display_mode(mode: c_uint) {
    unsafe {
        glutInitDisplayMode(mode);
    }
}

pub fn swap_buffers() {
    unsafe {
        glutSwapBuffers();
    }
}

pub fn post_redisplay() {
    unsafe {
        glutPostRedisplay();
    }
}

pub fn get(state: State) -> c_int {
    unsafe {
        let glut_state;
        match state {
            WindowWidth => glut_state = WINDOW_WIDTH,
            WindowHeight => glut_state = WINDOW_HEIGHT
        }
        glutGet(glut_state)
    }
}

pub fn get_modifiers() -> c_int {
    unsafe {
        glutGetModifiers()
    }
}

pub fn init_window_size(width:uint, height:uint) {
    unsafe {
        glutInitWindowSize(width as c_int, height as c_int);
    }
}

#[cfg(target_os="android")]
#[link(name="android-glue")]
extern {}

#[cfg(target_os="macos")]
#[link(name="GLUT", kind="framework")]
extern {
    // Mac GLUT extension.
    fn glutCheckLoop();
}

#[cfg(target_os="linux")]
#[link(name="glut")]
extern {}

#[cfg(target_os="linux")]
#[cfg(target_os="android")]
extern {
    // freeglut extension.
    fn glutMainLoopEvent();
}

extern {

pub fn glutInit(argcp: *c_int, argv: **c_char);

pub fn glutInitDisplayMode(mode: c_uint);

#[cfg(not(target_os="android"))]
pub fn glutInitDisplayString(string: *c_char);

#[cfg(not(target_os="android"))]
pub fn glutInitWindowPosition(x: c_int, y: c_int);

pub fn glutInitWindowSize(width: c_int, height: c_int);

#[cfg(not(target_os="android"))]
pub fn glutMainLoop();

pub fn glutCreateWindow(title: *c_char) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutCreateSubWindow(win: c_int, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int;

pub fn glutDestroyWindow(win: c_int);

pub fn glutPostRedisplay();

#[cfg(not(target_os="android"))]
pub fn glutPostWindowRedisplay(win: c_int);

pub fn glutSwapBuffers();

pub fn glutGetWindow() -> c_int;

pub fn glutSetWindow(win: c_int);

pub fn glutSetWindowTitle(title: *c_char);

#[cfg(not(target_os="android"))]
pub fn glutSetIconTitle(title: *c_char);

#[cfg(not(target_os="android"))]
pub fn glutPositionWindow(x: c_int, y: c_int);

pub fn glutReshapeWindow(width: c_int, height: c_int);

#[cfg(not(target_os="android"))]
pub fn glutPopWindow();

#[cfg(not(target_os="android"))]
pub fn glutPushWindow();

#[cfg(not(target_os="android"))]
pub fn glutIconifyWindow();

#[cfg(not(target_os="android"))]
pub fn glutShowWindow();

#[cfg(not(target_os="android"))]
pub fn glutHideWindow();

#[cfg(not(target_os="android"))]
pub fn glutFullScreen();

#[cfg(not(target_os="android"))]
pub fn glutSetCursor(cursor: c_int);

#[cfg(not(target_os="android"))]
pub fn glutWarpPointer(x: c_int, y: c_int);

#[cfg(not(target_os="android"))]
pub fn glutEstablishOverlay();

#[cfg(not(target_os="android"))]
pub fn glutRemoveOverlay();

#[cfg(not(target_os="android"))]
pub fn glutUseLayer(layer: GLenum);

#[cfg(not(target_os="android"))]
pub fn glutPostOverlayRedisplay();

#[cfg(not(target_os="android"))]
pub fn glutPostWindowOverlayRedisplay(win: c_int);

#[cfg(not(target_os="android"))]
pub fn glutShowOverlay();

#[cfg(not(target_os="android"))]
pub fn glutHideOverlay();

#[cfg(not(target_os="android"))]
pub fn glutCreateMenu(arg1: *u8) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutDestroyMenu(menu: c_int);

#[cfg(not(target_os="android"))]
pub fn glutGetMenu() -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutSetMenu(menu: c_int);

#[cfg(not(target_os="android"))]
pub fn glutAddMenuEntry(label: *c_char, value: c_int);

#[cfg(not(target_os="android"))]
pub fn glutAddSubMenu(label: *c_char, submenu: c_int);

#[cfg(not(target_os="android"))]
pub fn glutChangeToMenuEntry(item: c_int, label: *c_char, value: c_int);

#[cfg(not(target_os="android"))]
pub fn glutChangeToSubMenu(item: c_int, label: *c_char, submenu: c_int);

#[cfg(not(target_os="android"))]
pub fn glutRemoveMenuItem(item: c_int);

#[cfg(not(target_os="android"))]
pub fn glutAttachMenu(button: c_int);

#[cfg(not(target_os="android"))]
pub fn glutDetachMenu(button: c_int);

pub fn glutDisplayFunc(func: extern "C" fn());

pub fn glutReshapeFunc(func: extern "C" fn(i32, i32));

pub fn glutKeyboardFunc(func: extern "C" fn(u8, i32, i32));

pub fn glutMouseFunc(func: extern "C" fn(i32, i32, i32, i32));

#[cfg(target_os="linux")]
#[cfg(target_os="android")]
pub fn glutMouseWheelFunc(func: extern "C" fn(i32, i32, i32, i32));

pub fn glutMotionFunc(func: extern "C" fn(i32, i32));

pub fn glutPassiveMotionFunc(func: extern "C" fn(i32, i32));

#[cfg(not(target_os="android"))]
pub fn glutEntryFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutVisibilityFunc(func: *u8);

pub fn glutIdleFunc(func: extern "C" fn());

pub fn glutTimerFunc(millis: c_uint, func: extern "C" fn(int), value: c_int);

#[cfg(not(target_os="android"))]
pub fn glutMenuStateFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutSpecialFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutSpaceballMotionFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutSpaceballRotateFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutSpaceballButtonFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutButtonBoxFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutDialsFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutTabletMotionFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutTabletButtonFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutMenuStatusFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutOverlayDisplayFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutWindowStatusFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutKeyboardUpFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutSpecialUpFunc(func: *u8);

#[cfg(not(target_os="android"))]
pub fn glutJoystickFunc(func: *u8, pollInterval: c_int);

#[cfg(not(target_os="android"))]
pub fn glutSetColor(arg1: c_int, red: GLfloat, green: GLfloat, blue: GLfloat);

#[cfg(not(target_os="android"))]
pub fn glutGetColor(ndx: c_int, component: c_int) -> GLfloat;

#[cfg(not(target_os="android"))]
pub fn glutCopyColormap(win: c_int);

pub fn glutGet(_type: GLenum) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutDeviceGet(_type: GLenum) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutExtensionSupported(name: *c_char) -> c_int;

pub fn glutGetModifiers() -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutLayerGet(_type: GLenum) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutGetProcAddress(procName: *c_char) -> *c_void;

#[cfg(not(target_os="android"))]
pub fn glutBitmapCharacter(font: *c_void, character: c_int);

#[cfg(not(target_os="android"))]
pub fn glutBitmapWidth(font: *c_void, character: c_int) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutStrokeCharacter(font: *c_void, character: c_int);

#[cfg(not(target_os="android"))]
pub fn glutStrokeWidth(font: *c_void, character: c_int) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutBitmapLength(font: *c_void, string: *c_uchar) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutStrokeLength(font: *c_void, string: *c_uchar) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutWireSphere(radius: GLdouble, slices: GLint, stacks: GLint);

#[cfg(not(target_os="android"))]
pub fn glutSolidSphere(radius: GLdouble, slices: GLint, stacks: GLint);

#[cfg(not(target_os="android"))]
pub fn glutWireCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);

#[cfg(not(target_os="android"))]
pub fn glutSolidCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);

#[cfg(not(target_os="android"))]
pub fn glutWireCube(size: GLdouble);

#[cfg(not(target_os="android"))]
pub fn glutSolidCube(size: GLdouble);

#[cfg(not(target_os="android"))]
pub fn glutWireTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);

#[cfg(not(target_os="android"))]
pub fn glutSolidTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);

#[cfg(not(target_os="android"))]
pub fn glutWireDodecahedron();

#[cfg(not(target_os="android"))]
pub fn glutSolidDodecahedron();

#[cfg(not(target_os="android"))]
pub fn glutWireTeapot(size: GLdouble);

#[cfg(not(target_os="android"))]
pub fn glutSolidTeapot(size: GLdouble);

#[cfg(not(target_os="android"))]
pub fn glutWireOctahedron();

#[cfg(not(target_os="android"))]
pub fn glutSolidOctahedron();

#[cfg(not(target_os="android"))]
pub fn glutWireTetrahedron();

#[cfg(not(target_os="android"))]
pub fn glutSolidTetrahedron();

#[cfg(not(target_os="android"))]
pub fn glutWireIcosahedron();

#[cfg(not(target_os="android"))]
pub fn glutSolidIcosahedron();

#[cfg(not(target_os="android"))]
pub fn glutVideoResizeGet(param: GLenum) -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutSetupVideoResizing();

#[cfg(not(target_os="android"))]
pub fn glutStopVideoResizing();

#[cfg(not(target_os="android"))]
pub fn glutVideoResize(x: c_int, y: c_int, width: c_int, height: c_int);

#[cfg(not(target_os="android"))]
pub fn glutVideoPan(x: c_int, y: c_int, width: c_int, height: c_int);

#[cfg(not(target_os="android"))]
pub fn glutReportErrors();

#[cfg(not(target_os="android"))]
pub fn glutIgnoreKeyRepeat(ignore: c_int);

#[cfg(not(target_os="android"))]
pub fn glutSetKeyRepeat(repeatMode: c_int);

#[cfg(not(target_os="android"))]
pub fn glutForceJoystickFunc();

#[cfg(not(target_os="android"))]
pub fn glutGameModeString(string: *c_char);

#[cfg(not(target_os="android"))]
pub fn glutEnterGameMode() -> c_int;

#[cfg(not(target_os="android"))]
pub fn glutLeaveGameMode();

#[cfg(not(target_os="android"))]
pub fn glutGameModeGet(mode: GLenum) -> c_int;

}
