#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // rect_as_struct();
    // show();
    impl_fn();
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn impl_fn() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("이 사각형의 면적은 {}입니다.", rect.area());
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn associated_fn() {
    println!("정사각형 = {:?}", Rectangle::square(30));
}

fn rect_as_struct() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("해당 사각형의 면적은 {:?}.", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn show() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("해당 사각형은 {:?}.", rect);
    dbg!(rect);
}
