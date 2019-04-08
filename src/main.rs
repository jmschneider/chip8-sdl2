extern crate sdl2;
extern crate chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use chip8::cpu::Cpu;

const PIXEL_SIZE: u32 = 15;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut cpu = Cpu::new();

    let rom = include_bytes!("roms/INVADERS");
    cpu.load(rom);
 
    let height =  PIXEL_SIZE * chip8::display::WIDTH as u32;
    let width =  PIXEL_SIZE * chip8::display::HEIGHT as u32;
    let window = video_subsystem.window("Chip 8 Emulator", height, width)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for _ in 0..9 { cpu.execute_cycle(); }
        cpu.decrement_timers();

        for y in 0..(chip8::display::HEIGHT as u32) {
            for x in 0..(chip8::display::WIDTH as u32) {
                let idx = y * chip8::display::WIDTH as u32 + x;
                canvas.set_draw_color(color(cpu.display.memory[idx as usize]));
                
                let _ = canvas.fill_rect(
                    Rect::new((PIXEL_SIZE * x) as i32 - 1, (PIXEL_SIZE * y) as i32 - 1, PIXEL_SIZE, PIXEL_SIZE)
                );
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn color(value: bool) -> Color {
    if value {
        Color::RGBA(0, 204, 204)
    } else {
        Color::RGBA(0, 0, 0, 51)
    }
}
