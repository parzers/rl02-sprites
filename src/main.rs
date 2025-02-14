use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const PLAYER_SPEED: f32 = 400.0;

struct Game
{
    player_x: f32,
    player_y: f32,
    tex_pumuckl: Texture2D,
    tex_bg: Texture2D,
}

impl Game
{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self
    {
        let tex_pumuckl = rl.load_texture(&thread, "assets/pumuckl.png").unwrap();
        let tex_bg = rl.load_texture(&thread, "assets/background.png").unwrap();
        Self { player_x: 0.0, player_y: 0.0, tex_pumuckl, tex_bg }
    }
}

fn update(game: &mut Game, rl: &mut RaylibHandle, delta_time: f32)
{
    use raylib::consts::KeyboardKey::*;
    if rl.is_key_down(KEY_LEFT) {
        game.player_x -= delta_time * PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_RIGHT) {
        game.player_x += delta_time * PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_UP) {
        game.player_y -= delta_time * PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_DOWN) {
        game.player_y += delta_time * PLAYER_SPEED;
    }
}

fn draw(game: &Game, mut d: RaylibDrawHandle) {
    d.clear_background(Color::PINK);
    d.draw_texture(&game.tex_bg, 0, 0, Color::WHITE);
    d.draw_texture(&game.tex_pumuckl, game.player_x as i32, game.player_y as i32, Color::WHITE);
    d.draw_fps(SCREEN_WIDTH - 110, 10);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, Raylib!")
        .build();
    
    let mut game = Game::new(&mut rl, &thread);
    
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        update(&mut game, &mut rl, delta_time);
        draw(&game, rl.begin_drawing(&thread));
    }
}
