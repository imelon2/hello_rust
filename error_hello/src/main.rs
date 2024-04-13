fn main() {
    // result_fn();
    // result_recover_fn();
    // result_recover_unwrap_fn();
    // result_recover_expect_fn();
    // let err = read_username_long();
    // let err = read_username_short();
    // let err = read_username_shorter();
    let err = read_username_even_shorter();
    match err {
        Result::Ok(_str) => println!("{}",_str),
        Result::Err(_e) => println!("{}",_e)
    }
}

fn panic_fn() {
        // panic!("강제 종료 !!")

        let v = vec![1,2,3];

        v[99];
}

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

use std::{fs::File, io::ErrorKind};

fn result_fn() {
    let file = File::open("hello.txt");
    match file {
        Ok(f) => println!("Success opne file"),
        Err(e) => panic!("Fail open file")
    }
}

fn result_recover_fn() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(f) => Ok(f),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("hello.txt"),
            _ => panic!("파일 접근 실패")
        }
    };
}

fn result_recover_unwrap_fn() {
    let file = File::open("hello.txt").unwrap();
}

fn result_recover_expect_fn() {
    let file = File::open("hello.txt").expect("파일을 열 수 없음");
}

use std::io::Error;
use std::io::Read;

/* 에러 전파를 일일이 다루었을 경우 코드가 길어집니다. */
fn read_username_long() -> Result<String, Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* ?축약 표현을 쓰면 간결하게 연쇄적인 에러를 편히 다룰 수 있습니다. */
fn read_username_short() -> Result<String, Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

/* ?축약표현을 연이어 쓸 수도 있습니다. */
fn read_username_shorter() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

/* fs::read_to_string은 내부적으로 ?축약표현이 이미 들어있습니다. */
fn read_username_even_shorter() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}