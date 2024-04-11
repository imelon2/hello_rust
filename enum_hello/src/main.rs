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
    let m1 = MEss
}