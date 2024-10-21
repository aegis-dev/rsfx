
// Determines
pub enum RenderResolution {
    W427h240,
    W640h360,
    W854h480,
    Native
}

pub struct GraphicsSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub render_resolution: RenderResolution,
}

impl GraphicsSettings {
    pub fn new() -> GraphicsSettings {
        GraphicsSettings { vsync: false, fullscreen: true, render_resolution: RenderResolution::W854h480 }
    }
}