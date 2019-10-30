
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use std::fs::File;
use std::io::{self, Write, Read};


// To make things easier to read, we'll create a constant which will be the texture's size.
const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    let content = format!("{}\n{}\n", s_highscores, s_number_of_lines);
    write_into_file(&content, "scores.txt").is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ").filter_map(|nb| nb.parse::<u32>().ok()).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content.splitn(2, "\n").map(|line|
            line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_of_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, number_of_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
                           texture_creator: &'a TextureCreator<WindowContext>,
                           color: TextureColor,
                           size: u32) -> Option<Texture<'a>> {
    // We'll want to handle failures outside of this function.
    if let Ok(mut square_texture) =
        texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                // For now, TextureColor only handles two colors.
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        // An error occured so we return nothing and let the function caller handle the error.
        None
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    sdl2::image::init(INIT_JPG | INIT_PNG).expect("Couldn't initialize image context");

    // Parameters are: title, width, height
    let window = video_subsystem.window("Tetris", 800, 600)
                                .position_centered() // to put it in the middle of the screen
                                .build() // to create the window
                                .expect("Failed to create window");

    let mut canvas = window.into_canvas()
                           .target_texture()
                           .present_vsync() // To enable v-sync.
                           .build()
                           .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let image_texture = texture_creator.load_texture("assets/OceanBeach.jpg").expect("Couldn't load image");

    // We create a texture with a 32x32 size.
    let green_square = create_texture_rect(&mut canvas,
                                           &texture_creator,
                                           TextureColor::Green,
                                           TEXTURE_SIZE).expect("Failed to create a texture");
    let blue_square = create_texture_rect(&mut canvas,
                                          &texture_creator,
                                          TextureColor::Blue,
                                          TEXTURE_SIZE).expect("Failed to create a texture");

let timer = SystemTime::now();

    // First we get the event handler:
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    // Then we create an infinite loop to loop over events:
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // If we receive a 'quit' event or if the user press the 'ESC' key, we quit.
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running // We "break" the infinite loop.
                },
                _ => {}
            }
        }

        // We fill our window with red.
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // We draw it.
        canvas.clear();

        // The rectangle switch happens here:
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                // In case of error, we do nothing...
                true
            }
        };
        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        canvas.copy(&image_texture, None, None).expect("Render failed");
        // Copy our texture into the window.
        canvas.copy(square_texture,
                    None,
                    // We copy it at the top-left of the window with a 32x32 size.
                    Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
              .expect("Couldn't copy texture into window");
        // We update window's display.


        canvas.present();

        // We sleep enough to get ~60 fps. If we don't call this, the program will take
        // 100% of a CPU time.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

