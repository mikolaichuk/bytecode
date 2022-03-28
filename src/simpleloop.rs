#[derive(Debug)]
pub struct SimpleLoop {
    start_pointer: Option<usize>,
    counter: Option<i32>,
}

impl SimpleLoop {
    pub fn new() -> Self {
        SimpleLoop {
            start_pointer: None,
            counter: None,
        }
    }

    pub fn get_start_pointer(&self) -> Option<usize> {
        self.start_pointer
    }

    pub fn activate(&mut self, pointer: usize, counter: i32) {
        self.start_pointer = Some(pointer);
        self.counter = Some(counter);
    }

    pub fn desactivate(&mut self) {
        self.start_pointer = None;
        self.counter = None;
    }

    pub fn dec_counter(&mut self) {
        match self.counter.as_mut() {
            Some(counter) => *counter -= 1,
            None => (),
        }
    }

    pub fn is_counter_eq_zero(&self) -> bool {
        match self.counter {
            Some(counter) => counter == 0,
            None => false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.counter.is_some() && self.start_pointer.is_some()
    }
}
