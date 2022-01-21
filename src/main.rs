use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect,Point};
use std::time::Duration;

pub fn main() {
    let mut end:bool = false;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window("starter", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut h:Hero = Hero{name:String::from("Dog"),positionX:200,positionY:200};
    let mut heroRect = Rect::new(h.positionX,h.positionY,30,30);
    let mut movement:(Movement,Movement) = (Movement::Stop,Movement::Stop);

    'running: loop {
        match movement.0 {
            Movement::Forward => {h.positionX+=1;}
            Movement::Back => {h.positionX-=1;}
            _ => {}
        }
        match movement.1 {
            Movement::Forward => {h.positionY+=1;}
            Movement::Back => {h.positionY-=1;}
            _ => {}
        }
        heroRect = Rect::new(h.positionX,h.positionY,30,30);
        canvas.set_draw_color(Color::RGB(255, 59, 50));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(heroRect);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }|
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    movement.0=Movement::Back;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    movement.0=Movement::Stop;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    movement.0=Movement::Forward;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    movement.0=Movement::Stop;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    movement.1=Movement::Back;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    movement.1=Movement::Stop;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    movement.1=Movement::Forward;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    movement.1=Movement::Stop;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                     let mut window2 = video_subsystem.window("starter", 1920, 1080)
                            .position_centered()
                            .build()
                            .unwrap();
                     canvas = window2.into_canvas().build().unwrap();
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }

    fn myfunc() -> (){
        println!("111");
    }
}


struct Hero {
    name: String,
    positionX: i32,
    positionY: i32
}

enum Movement {
    Forward,
    Back,
    Stop
}
