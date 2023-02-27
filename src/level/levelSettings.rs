use sdl2::pixels::Color;
pub struct SpriteWithFloat {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
    pub color: Color,
}
pub struct LevelSettings {
    pub player: SpriteWithFloat,
    pub bounce: f64,
    pub max_gravity: f64,
    pub max_force_x: f64,
    pub min_force: f64,
    pub speed: f64,
    pub jump_force: f64,
    pub friction: f64,
    pub gravity: f64,
    pub blockColor: Color,
    pub goalColor : Color,
    pub enemyColor : Color,
}
