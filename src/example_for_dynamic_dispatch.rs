pub trait Animal {
    fn speak(&self);
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

pub fn make_animal_speak(animal: &dyn Animal) {
    animal.speak();
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dynamic_dispatch() {
        let dog = Dog;
        make_animal_speak(&dog);
    }

    #[test]
    fn sizeof() {
        println!("size of i32::{}", std::mem::size_of::<f32>());
        let points = vec![Point { x: 1.0, y: 2.0, z: 3.0 }; 1_000_000];
        println!("size of i32::{}", std::mem::size_of::<Point>());

    }
}
