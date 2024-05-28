struct Point {
    x: f64,
    y: f64,
}

enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point },
}

fn main() {
    // let my_shape = Shape::Rectangle {
    //     top_left: Point { x: 0.0, y: 5.0 },
    //     bottom_right: Point { x: 5.0, y: 0.0 },
    // };

    let my_shape = Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 5.0,
    };

    match my_shape {
        Shape::Circle { center, radius } =>
            println!(
                "Circle with center at ({}, {}) and radius: {}",
                center.x, center.y, radius),
        Shape::Rectangle { top_left, bottom_right } =>
            println!(
                "Rectangle with top left at ({}, {}) and bottom right at ({}, {})",
                top_left.x, top_left.y, bottom_right.x, bottom_right.y),
    }
}
