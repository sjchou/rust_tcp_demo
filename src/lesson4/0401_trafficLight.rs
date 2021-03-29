
pub trait TrafficLightTrait {
    fn logTime(&self);
}
enum TrafficLight {
    Red,
    Green,
    Yellow
}
impl TrafficLightTrait for TrafficLight{
    fn logTime(&self) {
        match self {
            Red => println!("wait for 40s"),
            Green => println!("wait for 60s"),
            Yellow => println!("wait for 5s"),
        }
    }
}
fn main() {
    println!("Hello, world!");
    let yellow : TrafficLight = TrafficLight::Yellow;
    yellow.logTime();
}
