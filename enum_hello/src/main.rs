#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Message {
    StartGame,
    WInPoint {
        who:String
    },
    ChangePlayerName(String)
}

fn main() {

}

fn enum1() {
    let red = Color::Red;
    let green = Color::Green;
    println!("red = {:?}", red);

    // enum 비교 가능
    println!("red == greed => {}",red == green);
    println!("red == greed => {}",red == Color::Red);
}

fn enum2() {
    let m1 = Message::StartGame;
    let m2 = Message::WInPoint { who: String::from("홍길동") };
    let m3 = Message::ChangePlayerName(
        String::from("길동")
    );
}

fn enumOption() {
    let some_number = Some(2);
    let absent_number:Option<i32> = None;
}