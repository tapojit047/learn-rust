use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

pub fn generate_pair<T>(x: T, y: T) -> Pair<T> {
    Pair { x, y }
}

impl<T> Pair<T> {
    pub fn new(self, x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}