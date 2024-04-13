enum Color {
    Red, Green, Blue
}

struct RGB(u8,u8,u8);

enum Message {
    StartGame,
    WinPoint {who:String},
    ChangePlayerName(String)
}

fn main() {
    // handle_message_wildcard(&Message::ChangePlayerName(String::from("hello")));
    handle_message(&Message::ChangePlayerName(String::from("hello")));
}

/** ------------------------------------------------------------------------------------ */

fn color_to_rgb(color:Color) -> RGB {
    match color {
        Color::Red => RGB(255,0,0),
        Color::Green => RGB(0,255,0),
        Color::Blue => RGB(0,0,255)
    }
}

/** ------------------------------------------------------------------------------------ */

fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("게임 시작!!"),
        Message::WinPoint { who } => println!("{}의 득점",who),
        Message::ChangePlayerName(name) => println!("플레이어 이름 변경 => {}", name)
    };
}

/** ------------------------------------------------------------------------------------ */

fn increment(x:Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None // match는 모든 경우를 다 처리해야는데, Option은 None이 포함됨
    }
}

fn option() {
    let x = Some(2);
    println!("{:?}",increment(x));
    println!("{:?}",increment(None));
}

/** ------------------------------------------------------------------------------------ */

fn handle_message_wildcard(message: &Message) {
    match message {
        Message::StartGame => println!("게임 시작!!"),
        Message::WinPoint { who } => println!("{}의 득점",who),
        _ => println!("아직 구현되지 않은 메시지")
    }
}