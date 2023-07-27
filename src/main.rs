use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("DINO in Rust")
        .build();

    let i = Image::load_image("assets/sprite.png")
        .expect("could not load sprite");

    let t = rl
        .load_texture_from_image(&thread, &i)
        .expect("could not load texture from image");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_texture_rec(
            &t, 
            Rectangle::new(76.0, 0.0, 88.0, 96.0), 
            Vector2::new(0.0, 0.0),
            Color::WHITE,
            );
    }
}
