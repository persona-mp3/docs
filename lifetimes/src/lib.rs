#[allow(dead_code)]
pub enum TaskType {
    Async,
    IO,
}
pub struct Task {
    pub name: String,
    pub task_type: TaskType,
    pub function: fn() -> String,
}

pub trait Execute {
    fn execute(&self);
}

impl Execute for Task {
    fn execute(&self) {
        let res = (self.function)();
        let fmt = format!("Executed ::{}\n{}", self.name, res);
        println!("{}", fmt);
    }
}
