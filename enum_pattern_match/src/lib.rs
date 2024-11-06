// This is a library for enum pattern match
// ```
// use enum_pattern_match::TrafficLight;
// enum_pattern_match::print_light_color(TrafficLight::Red);
// ```
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub fn print_light_color(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Red"),
        TrafficLight::Green => println!("Green"),
        TrafficLight::Yellow => println!("Yellow"),
    }
}
