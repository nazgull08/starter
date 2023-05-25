mod terrain;

extern crate sdl2;
extern crate nalgebra as na;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::image::{InitFlag, LoadTexture};
use std::path::Path;


pub fn main() {
    let mut cube : terrain::Terrain;
    let mut end:bool = false;
    let mut marineCounter : u8 = 0;

    let mut rng = rand::thread_rng();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init();
    let image_context = sdl2::image::init(InitFlag::PNG);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut ss = match video_subsystem.current_display_mode(0) {
            Ok(dm) => dm,
            Err(e) => return ()
    };

    let mut sW: u32 = ss.w as u32;
    let mut sH: u32 = ss.h as u32;

    let mut window = video_subsystem.window("Starter",sW,sH).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

//======================Images=========================================

    let bgPath : &Path = Path::new("./res/img/twin.png");
    let marinePath : &Path = Path::new("./res/img/smallmarine.png");
    let marinebPath : &Path = Path::new("./res/img/smallmarineb.png");
    let floorPath : &Path = Path::new("./res/img/floor/isoTest1.png");
    let wall1Path : &Path = Path::new("./res/img/floor/isoWall1.png");
    let bgPath: &Path = Path::new("./res/img/twin.png");
    let marinePath: &Path = Path::new("./res/img/smallmarine.png");

    let texture_creator = canvas.texture_creator();

    let bg = texture_creator.load_texture(bgPath).unwrap();
    let marine = texture_creator.load_texture(marinePath).unwrap();
    let marineb = texture_creator.load_texture(marinebPath).unwrap();
    let floor = texture_creator.load_texture(floorPath).unwrap();
    let wall1 = texture_creator.load_texture(wall1Path).unwrap();


//=====================================================================




    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 59, 50));
        canvas.clear();
        canvas.copy(&bg, None, None);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }|
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                },
                  Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                },
                Event::MouseMotion {x,y,..} =>{
                }
                Event::MouseButtonDown {x,y,mouse_btn,..} =>{
                    if mouse_btn == sdl2::mouse::MouseButton::Left{
                    }
                    if mouse_btn == sdl2::mouse::MouseButton::Right{
                    }
                }
                _ => {}
            }
       }
        drawTerrain();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 500));
    }

}

struct World {

}

struct Hero {
    name: String,
    color: Color,
    positionX: i32,
    positionY: i32
}

enum Movement {
    Forward,
    Back,
    Stop
}

fn drawTerrain(){

}
