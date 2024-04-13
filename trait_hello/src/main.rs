fn main() {
    let cat = Pet::Cat;
    let gildong = Person {
        name:String::from("워녁"),
        active:true
    };

    // meet(&cat, &gildong);
    // meet2(&cat, &cat);
    // meet3(&gildong, &cat);
    // meet4(&cat, &cat);
    meet5(&cat, &gildong);

}

trait Greet {
    fn greeting(&self) -> String;
}

#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
    Tiger
}

struct Person {
    name:String,
    active:bool
}

impl Greet for Pet {
    fn greeting(&self) -> String {
        match &self {
            Pet::Dog => String::from("멍멍"),
            Pet::Cat => String::from("냐옹"),
            Pet::Tiger => String::from("어흥"),
        }
    }
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕")
    }
}

/** ------------------------------------------------------------------------------------ */

fn meet(one:&impl Greet, another:&impl Greet) {
    println!("첫번째가 인사합니다 = {}",one.greeting());
    println!("두번째가 인사합니다 = {}",another.greeting());
}


fn meet2<T: Greet>(one:&T,another:&T) {
    println!("첫번째가 인사합니다 = {}",one.greeting());
    println!("두번째가 인사합니다 = {}",another.greeting());
}

fn meet3<T: Greet,U:Greet>(one:&T,another:&U) {
    println!("첫번째가 인사합니다 = {}",one.greeting());
    println!("두번째가 인사합니다 = {}",another.greeting());
}

use std::fmt::Debug;

fn meet4<T:Greet + Debug>(one:&T,another:&T) {
    println!("{:?} 첫번째가 인사합니다 = {}",one,one.greeting());
    println!("{:?} 두번째가 인사합니다 = {}",another,another.greeting());
}

use std::fmt::Display;

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.active.to_string())
    }
}

fn meet5<T:Greet + Debug,U:Greet + Display>(one:&T,another:&U) {
    println!("{:?} 첫번째가 인사합니다 = {}",one,one.greeting());
    println!("두번째가 인사합니다 = {}",another.greeting());
}

/** meet5()와 똑같은 문법 */
fn meet6<T,U>(one:&T,another:&U)
where
    T:Greet + Debug,
    U:Greet + Display,
    {
        println!("{:?} 첫번째가 인사합니다 = {}",one,one.greeting());
        println!("두번째가 인사합니다 = {}",another.greeting());
    }