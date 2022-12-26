struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn combine(rec1: &Rectangle, rec2: &Rectangle) -> Rectangle {
        Rectangle {
            width: rec1.width + rec2.width,
            height: rec1.height + rec2.height
        }
    }
}

fn main() {
    let billy = Rectangle {
        width: 32,
        height: 40
    };

    let maybe_sqr = Rectangle::square(5);

    println!("billy's area is {}", billy.area());
    println!("t/f, maybe_sqr is a square: {}", maybe_sqr.is_square());

    let billy_and_maybe = Rectangle::combine(&billy, &maybe_sqr);

    println!("the area of billy and maybe_sqr combined is {}", billy_and_maybe.area());
}