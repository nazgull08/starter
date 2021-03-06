mod terrain;

extern crate sdl2;
extern crate nalgebra as na;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect,Point};
use std::time::Duration;
use sdl2::video::{Window, WindowPos};
use sdl2::render::TextureQuery;
use sdl2::image::{InitFlag, LoadTexture};
use rand::Rng;
use std::path::Path;


pub fn main() {
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

    let bgPath : &Path = (Path::new("./res/img/twin.png"));
    let marinePath : &Path = (Path::new("./res/img/smallmarine.png"));
    let marinebPath : &Path = (Path::new("./res/img/smallmarineb.png"));
    let floorPath : &Path = (Path::new("./res/img/floor/isoTest1.png"));
    let wall1Path : &Path = (Path::new("./res/img/floor/isoWall1.png"));
    let bgPath: &Path = (Path::new("./res/img/twin.png"));
    let marinePath: &Path = (Path::new("./res/img/smallmarine.png"));

    let texture_creator = canvas.texture_creator();

    let bg = texture_creator.load_texture(bgPath).unwrap();
    let marine = texture_creator.load_texture(marinePath).unwrap();
    let marineb = texture_creator.load_texture(marinebPath).unwrap();
    let floor = texture_creator.load_texture(floorPath).unwrap();
    let wall1 = texture_creator.load_texture(wall1Path).unwrap();


//=====================================================================

    let hColor = Color::RGB(255, 255, 255);
    let mColor = Color::RGB(0, 150, 0);



    let mut  click:(i32,i32,sdl2::mouse::MouseButton);


    let mut h:Hero = Hero{name:String::from("Dog"), color:hColor,positionX:200,positionY:200};
    let mut m:Hero = Hero{name:String::from("Mouse"), color:mColor,positionX:100,positionY:100};
    let mut heroRect = Rect::new(h.positionX,h.positionY,100,100);
    let mut mousRect = Rect::new(m.positionX,m.positionY,20,20);
    let mut movement:(Movement,Movement) = (Movement::Stop,Movement::Stop);
    let mut heroTexture = &marine;  


    let ship = [Rect::new(400,400,150,100),Rect::new(550,400,150,100),Rect::new(700,400,150,100),Rect::new(850,400,150,100),
                Rect::new(475,445,150,100),Rect::new(625,445,150,100),Rect::new(775,445,150,100),Rect::new(925,445,150,100),
                Rect::new(550,490,150,100),Rect::new(700,490,150,100),Rect::new(850,490,150,100),Rect::new(1000,490,150,100),
                Rect::new(625,535,150,100),Rect::new(775,535,150,100),Rect::new(925,535,150,100),Rect::new(1075,535,150,100)
    ];



    'running: loop {
        match movement.0 {
            Movement::Forward => {h.positionX+=5;}
            Movement::Back => {h.positionX-=5;}
            _ => {}
        }
        match movement.1 {
            Movement::Forward => {h.positionY+=5;}
            Movement::Back => {h.positionY-=5;}
            _ => {}
        }
        heroRect = Rect::new(h.positionX,h.positionY,100,100);
        mousRect = Rect::new(m.positionX,m.positionY,20,20);
        canvas.set_draw_color(Color::RGB(255, 59, 50));
        canvas.clear();
        canvas.copy(&bg, None, None);
        canvas.set_draw_color(h.color);
        for fl in ship{
            canvas.copy(&floor,None, fl);
        };
        if marineCounter<=15 {
            heroTexture = &marine;
        }
        else{
            heroTexture = &marineb;
        };

        canvas.copy(heroTexture, None, heroRect);
        canvas.set_draw_color(m.color);
        canvas.fill_rect(mousRect);

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
                    let mut rC = rng.gen_range(0..255);
                    let mut gC = rng.gen_range(0..255);
                    let mut bC = rng.gen_range(0..255);
                    h.color = Color::RGB(rC,gC,bC);
                },
                Event::MouseMotion {x,y,..} =>{
                    m.positionX = x;
                    m.positionY = y;
                }
                Event::MouseButtonDown {x,y,mouse_btn,..} =>{
                    click = (x,y,mouse_btn);
                    if (mouse_btn == sdl2::mouse::MouseButton::Left){
                        h.positionX = x-50;
                        h.positionY = y-50;
                    }
                    if (mouse_btn == sdl2::mouse::MouseButton::Right){
                        h.color = Color::RGB(255,0,0);
                    }
                }
                _ => {}
            }
       }
        drawTerrain();
        canvas.present();
        marineCounter+=1;
        if marineCounter >= 30 {
            marineCounter = 0;
        }
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
