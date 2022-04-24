use beryllium::init::Sdl;
use beryllium::init::InitFlags;
use beryllium::gl_window::GlAttr;
use beryllium::gl_window::GlProfile;
use beryllium::gl_window::GlContextFlags;
use beryllium::window::WindowFlags;
use zstring::zstr;
use beryllium::event::Event;
use beryllium::{
    SdlResult,
};
use core::{ptr::null, str};
use glitz::{*};

pub fn show() {
    let sdl = Sdl::init(InitFlags::EVERYTHING).expect("couldn't start SDL");
    sdl.gl_set_attribute(GlAttr::RedSize, 16).unwrap();
    sdl.gl_set_attribute(GlAttr::GreenSize, 16).unwrap();
    sdl.gl_set_attribute(GlAttr::BlueSize, 16).unwrap();
    sdl.gl_set_attribute(GlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(GlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(GlAttr::Profile, GlProfile::Core as i32).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl
            .gl_set_attribute(GlAttr::Flags, GlContextFlags::FORWARD_COMPATIBLE.as_i32())
            .unwrap();
    }
    let gl_win = sdl
        .create_gl_window(
            zstr!("Hello Window"),
            None,
            (800, 600),
            WindowFlags::OPENGL,
        )
        .expect("couldn't make a window and context");
    let gl = unsafe { GlFns::from_loader(&|zs| gl_win.get_proc_address(zs)).unwrap() };
    gl.ClearColor(2.0, 0.6, 0.5, 1.0);
    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_event() {
            match event {
                Event::Quit => break 'main_loop,
                e => println!("{:?}", e),
            }
        }
        // now the events are clear

        // here's where we could change the world state and draw.
        gl.Clear(GL_COLOR_BUFFER_BIT);
        gl_win.swap_backbuffer();
    }
}