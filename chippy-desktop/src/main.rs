use std::env;

use chippy_core::Chip8;
use raylib::prelude::*;

const KEYS: [KeyboardKey; 16] = [
    KeyboardKey::KEY_X,     // 0
    KeyboardKey::KEY_ONE,   // 1
    KeyboardKey::KEY_TWO,   // 2
    KeyboardKey::KEY_THREE, // 3
    KeyboardKey::KEY_Q,     // 4
    KeyboardKey::KEY_W,     // 5
    KeyboardKey::KEY_E,     // 6
    KeyboardKey::KEY_A,     // 7
    KeyboardKey::KEY_S,     // 8
    KeyboardKey::KEY_D,     // 9
    KeyboardKey::KEY_Z,     // A
    KeyboardKey::KEY_C,     // B
    KeyboardKey::KEY_FOUR,  // C
    KeyboardKey::KEY_R,     // D
    KeyboardKey::KEY_F,     // E
    KeyboardKey::KEY_V,     // F
];

const SCALE: i32 = 12;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const DEFAULT_TICKS_PER_FRAME: u32 = 10; 
const TARGET_FPS: u32 = 30;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rom> [sound.wav] [ticks per frame, often 8-15]", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];
    let wav_path = args.get(2);
    let ticks_per_frame: u32 = args.get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_TICKS_PER_FRAME);
    let rom = std::fs::read(rom_path).unwrap_or_else(|e| {
        eprintln!("failed to read ROM '{}': {}", rom_path, e);
        std::process::exit(1);
    });

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32 * SCALE, HEIGHT as i32 * SCALE)
        .title("CHIP-8")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let audio = RaylibAudio::init_audio_device().unwrap();
    let sound = wav_path.map(|path| {
        audio.new_sound(path).unwrap_or_else(|e| {
            eprintln!("failed to load sound '{}': {}", path, e);
            std::process::exit(1);
        })
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);
    while !rl.window_should_close() {
        update_keypad(&mut rl, &mut chip8);

        for _ in 0..ticks_per_frame {
            chip8.tick();
        }


        chip8.tick_timers();
        if chip8.is_sound_playing() {
            if let Some(ref s) = sound {
                unsafe { raylib::ffi::PlaySound(**s) };
            }
        }


        if chip8.is_display_dirty() {
            draw_display(&mut rl, &thread, &chip8);
            chip8.clear_display_dirty();
        }
    }
}
fn draw_display(rl: &mut RaylibHandle, thread: &RaylibThread, chip8: &Chip8) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);

    // Draw every CHIP-8 pixel
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if chip8.get_display_buffer()[y * WIDTH + x] {
                d.draw_rectangle(
                    x as i32 * SCALE,
                    y as i32 * SCALE,
                    SCALE,
                    SCALE,
                    Color::GREEN,
                );
            }
        }
    }
}

fn update_keypad(rl: &mut RaylibHandle, chp8: &mut Chip8) {
    for index in 0..16 {
        if rl.is_key_down(KEYS[index as usize]) {
            chp8.key_down(index);
        } else {
            chp8.key_up(index);
        }
    }
}
