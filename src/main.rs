extern crate sdl2;

use sdl2::{ Sdl, SdlResult, VideoSubsystem };
use sdl2::video::{ Window, FullscreenType };
use sdl2::mouse::{ Cursor };
use sdl2::event::{ Event };

#[allow(dead_code)]
struct CipherClock {
  context: Sdl,
  subsystem: VideoSubsystem,
  window: Window,
  cursor: Cursor
}

impl CipherClock {

  fn new() -> CipherClock {

    let title = "The Cipher of The Monks";
    let width: u32 = 1000;
    let height: u32 = 1000;

    let context = sdl2::init().unwrap();
    let subsystem = context.video().unwrap();

    let window_builder = subsystem.window( title, width, height );

    let mut window = window_builder.build().unwrap();

    window.set_title( title );
    window.hide();

    return CipherClock {
      context: context,
      subsystem: subsystem,
      window: window,
      cursor: CipherClock::create_cursor(),
    };

  }

  fn create_cursor() -> Cursor {

    let data: &[u8] = &[];
    let mask: &[u8] = &[];

    return Cursor::new( data, mask, 1i32, 1i32, 0i32, 0i32 ).unwrap();

  }

  fn show( &mut self ) -> SdlResult<()> {
    // self.window.maximize();
    self.window.show();
    return self.window.set_fullscreen( FullscreenType::True );
  }

  fn hide_cursor( &mut self ) {
    self.cursor.set();
  }

  fn quit( &mut self ) {
    // self.window.set_fullscreen( FullscreenType::Off );
    self.window.hide();
  }

}

fn main() {

  let title = "The Cipher of The Monks";
  let mut clock = CipherClock::new();
  let mut event_loop = clock.context.event_pump().unwrap();

  clock.hide_cursor();

  match clock.show() {
    Ok(result) => println!( "{0} {1:?}", title, result ),
    Err(message) => panic!( "The Monks went to the Catacombs: {:?}", message ),
  }

  for event in event_loop.wait_iter() {
    match event {
      Event::KeyDown {..} => { clock.quit(); break },
      Event::MouseButtonDown {..} => { clock.quit(); break },
      Event::Window { win_event_id, .. } => {
        println!( "Window::{:?}", win_event_id );
      },
      _ => (),
    }
  }

}
