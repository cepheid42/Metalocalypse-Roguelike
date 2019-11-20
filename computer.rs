pub struct Computer {
    level: i32,
    difficulty: i32,
    pub health: i32
}

pub trait Enemy {
    fn new(level: i32, difficulty: i32) -> Self;
    fn action(&self) -> (i32, i32);
    fn damage(&mut self, dmg: i32);
    fn stats(&self) -> String;
}

impl Enemy for Computer {
    fn new(level: i32, difficulty: i32) -> Self {
        Computer {
            level: level,
            difficulty: difficulty,
            health: difficulty + level
        }
    }
    fn action(&self) -> (i32, i32){
        (self.level, self.difficulty)
    }
    fn damage(&mut self, dmg: i32){
        self.health -= dmg;
    }
    fn stats(&self) -> String{
        format!("level: {} difficulty: {}", self.level, self.difficulty)
    }
}