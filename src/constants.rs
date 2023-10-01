pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

pub const CONTAINER_WIDTH: f32 = 300.0;
pub const CONTAINER_HEIGHT: f32 = 400.0;
pub const CONTAINER_THICKNESS: f32 = 20.0;
pub const CONTAINER_BASE_OFFSET: f32 = 25.0;
pub const SPAWN_HEIGHT: f32 =
    -SCREEN_HEIGHT / 2.0 + CONTAINER_BASE_OFFSET + CONTAINER_HEIGHT + 20.0 + KNOWN_TYPES[5].0 / 2.0;
pub const NEXT_PREVIEW_OFFSET: f32 = 100.0;
pub const NEXT_PREVIEW_LABEL_SIZE: f32 = 40.0;

pub const GRAVITY: f32 = 3.0;
pub const RESTITUATION: f32 = 0.05;
pub const MASS: f32 = 5.0;
pub const MAX_SPEED: f32 = 100.0;
pub const MAX_Y_VELOCITY_BEFORE_CLAMP: f32 = 50.0;
pub const MAX_X_VELOCITY_BEFORE_CLAMP: f32 = 50.0;

pub const CLICK_DELAY: f32 = 0.8;
pub const KNOWN_TYPES: [(f32, &str); 11] = [
    (26.0, "gura.png"),
    (40.0, "aqua.png"),
    (54.0, "hakos.png"),
    (60.0, "towa.png"),
    (77.0, "kobo.png"),
    (92.0, "ayame.png"),
    (97.0, "koyori.png"),
    (129.0, "fubuki.png"),
    (154.0, "mio.png"),
    (174.0, "ollie.png"),
    (204.0, "sana.png"),
];
