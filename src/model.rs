#[derive(Clone)]
pub struct Process {
    pub id: i8,
    pub cpu: i32,
    pub io: i32,
    pub tc: i32,
    pub priority: i8,
}

impl Process{
    pub fn new() -> Process{
        Process { id: -1, cpu: -1, io: -1, tc: -1, priority: -1 }
    }

    pub fn is_empty(&self) -> bool{
        self.id < 0 || self.cpu <= 0 || self.io < 0
        || self.tc < 0 || self.priority < 0
    }
}