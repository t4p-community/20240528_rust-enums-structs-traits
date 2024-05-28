trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

fn draw_shape(shape: Box<dyn Drawable>) {
    shape.draw();
}

fn main() {
    let circle = Circle;
    let square = Square;

    draw_shape(Box::new(circle));
    draw_shape(Box::new(square));
}
