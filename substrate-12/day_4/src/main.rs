fn main() {
    test1();
    test2();
}

enum TrafficLight {
    Red,
    Green,
    Yello,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 30,
            TrafficLight::Yello => 3,
        }
    }
}

fn test1() {
    let light = TrafficLight::Red;
    println!("{}", light.time());
    println!("{}", TrafficLight::Yello.time());
    println!("{}", TrafficLight::Green.time());
}

#[allow(dead_code)]
fn sum(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in numbers {
        sum = sum.checked_add(num)?;
    }
    Some(sum)
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.area());
}

fn test2() {
    let circle = Circle { radius: 2.0 };
    print_area(circle);

    let triangle = Triangle { base: 3.0, height: 4.0 };
    print_area(triangle);

    let square = Square { side: 5.0 };
    print_area(square);
}

