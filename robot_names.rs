use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: Option<String>,
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: None }
    }

    pub fn name(&mut self) -> &str {
        if self.name.is_none() {
            self.name = Some(Self::generate_unique_name());
        }
        self.name.as_ref().unwrap()
    }

    pub fn reset_name(&mut self) {
        if let Some(ref name) = self.name {
            USED_NAMES.lock().unwrap().remove(name);
        }
        self.name = None;
    }

    fn generate_unique_name() -> String {
        let mut rng = rand::thread_rng();
        let mut name;
        loop {
            name = format!(
                "{}{}{:03}",
                (rng.gen_range(b'A'..=b'Z') as char),
                (rng.gen_range(b'A'..=b'Z') as char),
                rng.gen_range(0..1000)
            );
            if USED_NAMES.lock().unwrap().insert(name.clone()) {
                break;
            }
        }
        name
    }
}

fn main() {
    let mut robot = Robot::new();
    println!("Robot name: {}", robot.name());
    robot.reset_name();
    println!("Robot name after reset: {}", robot.name());
}