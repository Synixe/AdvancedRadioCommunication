use std::time::SystemTime;

pub struct Player {
    discord: i64,
    position: Vector3,
    velocity: Vector3,
    orientation: Vector3,
    time: SystemTime
}

impl Player {
    pub fn new(discord: i64) -> Player {
        Player {
            discord: discord.clone(),
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            orientation: Vector3::new(0.0, 0.0, 0.0),
            time: SystemTime::now()
        }
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        let oldtime = self.time.clone();
        self.time = SystemTime::now();
        let dif = self.time.duration_since(oldtime).unwrap();
        let elapsed: f32 = (dif.as_secs() as f32) + (dif.subsec_nanos() as f32 / 1_000_000_000.0);

        self.velocity = Vector3::new(
            (position[0] - self.position.x) / elapsed,
            (position[1] - self.position.y) / elapsed,
            (position[2] - self.position.z) / elapsed
        );

        self.position.x = position[0];
        self.position.y = position[1];
        self.position.z = position[2];
    }
    pub fn get_position(&self) -> &Vector3 { &self.position }
    pub fn get_velocity(&self) -> &Vector3 { &self.velocity }
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }
}
