fn main() {
    println!("Hello, world!");
}

/** 참조한 값의 수명이 더 짧은 경우 */
fn short_lifetime() {
    return;
    let x;

    // x가 참조한 y값이 범위(수명)을 벗어나, 수명 검사기에 의해 거부됨
    // Error : `y` dropped here while still borrowed
    {
        let y = 5;
        x = &y;
    }

    println!("x = {}",x);
}

/** 유효한 수명 */
fn ok_lifetime() {
    let y = 5;
    let x = &y;

    println!("x = {}",x);
}

/** ------------------------------------------------------------------------------------ */

/** 반환 값 수명 */
fn str_lifetime() {
    let s1 = String::from("가나다");
    let s2 = "하나둘셋";

    let res = longest(s1.as_str(), s2);
    println!("더 긴 문자열은 {}", res);
}

/* 수명 표기를 잘 한 경우 */
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/** ------------------------------------------------------------------------------------ */

/* 구조체에도 수명 표기를 합니다. */
struct ImportantPart<'a> {
    part: &'a str,
}

/* 구조체 ImportantPart의 part라는 문자열 슬라이스의 수명과 같게 유지합니다. */
fn lifetime_in_struct() {
    let sentences = String::from("안녕하세요. 러스트의 참조 수명에 대해 알아볼게요.");
    let first_sentence = sentences
        .split('.')
        .next()
        .expect("마침표 '.'를 찾을 수 없어요");
    let i = ImportantPart {
        part: first_sentence,
    };
}
