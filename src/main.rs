fn main() {
    let red = TrafficLight::Red;
    println!("red {}", red.time());
    let green = TrafficLight::Green;
    println!("green {}", green.time());
    let yellow = TrafficLight::Yellow;
    println!("yellow {}", yellow.time());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 80,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 3
        }
    }
}
