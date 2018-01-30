pub struct Player {
    name: String,
    discord: i64,
    position: [f32; 3],
    velocity: [f32; 3],
    orientation: [f32; 3]
}

impl Player {
    pub fn new(name: &str, discord: i64) -> Player {
        Player {
            name: name.to_string(),
            discord: discord.clone(),
            position: [0.0,0.0,0.0],
            velocity: [0.0,0.0,0.0],
            orientation: [0.0,0.0,0.0]
        }
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position.clone();
    }
    pub fn get_position(&self) -> &[f32; 3] {
        &self.position
    }
}
