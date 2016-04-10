extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
use std::cell::Cell;
use std::sync::mpsc::{Receiver};

macro_rules! glcheck {
    ($e: expr) => (
      {
        $e;
        assert_eq!(unsafe{gl::GetError()},0);
      }
    )
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    glcheck!(unsafe {gl::FrontFace(gl::CCW)});
    glcheck!(unsafe {gl::Enable(gl::DEPTH_TEST)});
    glcheck!(unsafe {gl::Enable(gl::SCISSOR_TEST)});
    glcheck!(unsafe {gl::DepthFunc(gl::LEQUAL)});
    glcheck!(unsafe {gl::FrontFace(gl::CCW)});
    glcheck!(unsafe {gl::Enable(gl::CULL_FACE)});
    glcheck!(unsafe {gl::CullFace(gl::BACK)});
}

pub struct Window
{
  pub width: i32,
  pub height: i32,
  pub framebuffer_width: i32,
  pub framebuffer_height: i32,
  pub pixel_ratio: f32,
  pub delta_time: f64,
  pub last_time: f64,
  pub nvg: nanovg::Context,
  glfw: glfw::Glfw,
  window: glfw::Window,
  events: Receiver<(f64,glfw::WindowEvent)>
}

impl Window
{
  pub fn new (width: u32, height: u32, title: &str) -> Window
  {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.set_error_callback(Some(
      glfw::Callback {
          f: error_callback as fn(glfw::Error, String, &Cell<usize>),
          data: Cell::new(0),
      }
    ));
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    
    let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    window.set_key_polling(true);
    window.make_current();
    
    glcheck!(gl::load_with(|name| window.get_proc_address(name) as *const _));
    init_gl();
    
    glfw.set_swap_interval(0);
    glfw.set_time(0.0);
    
    let (width, height) = window.get_size();
    let (framebuffer_width, framebuffer_height) = window.get_framebuffer_size();
    
    Window {
      width: width,
      height: height,
      framebuffer_width: framebuffer_width,
      framebuffer_height: framebuffer_height,
      pixel_ratio: framebuffer_width as f32 / width as f32,
      delta_time: 0.0,
      last_time: glfw.get_time(),
      nvg: nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES),
      glfw: glfw,
      window: window,
      events: events,
    }
  }
  
  pub fn should_close (&self) -> bool
  {
    self.window.should_close()
  }
  
  pub fn idle (&mut self)
  {
    self.glfw.wait_events();
    /*
    for (_, event) in glfw::flush_messages(&self.events) {
        //handle_window_event(&mut window, event);
    }
    */
  }
  
  pub fn update (&mut self)
  {
    let (width, height) = self.window.get_size();
    self.width = width;
    self.height = height;
    let (framebuffer_width, framebuffer_height) = self.window.get_framebuffer_size();
    self.framebuffer_width = framebuffer_width;
    self.framebuffer_height = framebuffer_height;
    self.pixel_ratio = self.framebuffer_width as f32 / self.width as f32;
    
    let current_time = self.glfw.get_time();
    self.delta_time = current_time - self.last_time;
    self.last_time = current_time;
  }
  
  pub fn begin_frame (&mut self)
  {
    glcheck!(unsafe {gl::Viewport(0, 0, self.framebuffer_width, self.framebuffer_height)});
    glcheck!(unsafe {gl::ClearColor(1.0, 1.0, 1.0, 1.0)});
    glcheck!(unsafe {gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT)});
    glcheck!(unsafe {gl::Enable(gl::BLEND)});
    glcheck!(unsafe {gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)});
    glcheck!(unsafe {gl::Enable(gl::CULL_FACE)});
    glcheck!(unsafe {gl::Disable(gl::DEPTH_TEST)});
    
    self.nvg.begin_frame(self.width as u32, self.height as u32, self.pixel_ratio);
  }
  
  pub fn end_frame (&mut self)
  {
    self.nvg.end_frame();
    unsafe {gl::Enable(gl::DEPTH_TEST);}
    self.window.swap_buffers();
  }
}