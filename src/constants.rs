use bevy::prelude::Color;

pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

pub const CONTAINER_WIDTH: f32 = 420.0;
pub const CONTAINER_HEIGHT: f32 = 500.0;
pub const CONTAINER_THICKNESS: f32 = 20.0;
pub const CONTAINER_BASE_OFFSET: f32 = 25.0;
pub const SPAWN_OFFSET: f32 =
    CONTAINER_BASE_OFFSET + CONTAINER_HEIGHT + CONTAINER_THICKNESS + KNOWN_TYPES[10].0 / 2.0;
pub const SPAWN_HEIGHT: f32 = -SCREEN_HEIGHT / 2.0 + SPAWN_OFFSET;
pub const GAME_OVER_HEIGHT: f32 =
    -SCREEN_HEIGHT / 2.0 + CONTAINER_BASE_OFFSET + CONTAINER_HEIGHT + CONTAINER_THICKNESS;
pub const NEXT_PREVIEW_OFFSET: f32 = 180.0;
pub const NEXT_PREVIEW_LABEL_SIZE: f32 = 40.0;

pub const GRAVITY: f32 = 3.5;
pub const RESTITUATION: f32 = 0.005;
pub const MASS: f32 = 5.0;
pub const MAX_SPEED: f32 = 100.0;
pub const MAX_Y_VELOCITY_BEFORE_CLAMP: f32 = 50.0;
pub const MAX_X_VELOCITY_BEFORE_CLAMP: f32 = 50.0;

pub const BG_COLOR: Color = Color::rgb(0.7922, 0.9412, 0.9725);
pub const NEXT_BG_COLOR: Color = Color::rgb(0.5647, 0.8784, 0.9373);
pub const CONTAINER_COLOR: Color = Color::rgb(0.0, 0.7059, 0.8471);
pub const SCORE_TEXT_COLOR: Color = Color::rgb(0.0, 0.4667, 0.7137);
pub const TEXT_COLOR: Color = Color::rgb(0.0118, 0.0157, 0.3686);
pub const PREVIEW_HINT_COLOR: Color = Color::rgba(0.0118, 0.0157, 0.3686, 0.25);
pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub const CLICK_DELAY: f32 = 0.8;
pub const KNOWN_TYPES: [(f32, &str, u32); 11] = [
    (31.2, "gura.png", 0),
    (48.0, "aqua.png", 1),
    (64.8, "hakos.png", 3),
    (72.0, "towa.png", 6),
    (92.4, "kobo.png", 10),
    (110.4, "ayame.png", 15),
    (116.4, "koyori.png", 21),
    (154.8, "fubuki.png", 28),
    (184.8, "mio.png", 36),
    (208.8, "ollie.png", 45),
    (244.8, "sana.png", 55),
];
