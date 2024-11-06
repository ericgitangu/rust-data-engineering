use traits_generics::print_area;
use traits_generics::Circle;
use traits_generics::Rectangle;

fn main() {
    let circle = Circle { radius: 1.0 };
    print_area(circle);

    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };
    print_area(rectangle);
}
