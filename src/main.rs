fn main() {
    study_enumeration();
}

fn study_enumeration() {
    // 1. create instance of Enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 2. Enum with field
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 3. implementation on enumeration
    let m = Message::Write(String::from("hello"));
    m.call();

    // 4. Study enum with Option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의함
    }
}