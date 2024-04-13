fn main() {
    // let numbers = vec![3,4,1,6,8,10];
    // let result = smallest_i32(&numbers);
    // println!("가장 작은 수는 {}",result);

    // let chars  = vec!['홍','길','동'];
    // let result = smallest_char(&chars);
    // println!("가장 먼저 오는 글자는 {}",result);

    // let result = smallest(&numbers);
    // println!("가장 작은 수는 {}",result);

    // let result = smallest(&chars);
    // println!("가장 먼저 오는 글자는 {}",result);

    // struct_generics_fn1();
    // struct_generics_fn2();

    let p1 = Point3 {x:1,y:2};
    println!("p1.x() = {}",p1.x());
}

/** ------------------------------------------------------------------------------------ */
fn smallest_i32(list:&[i32]) -> &i32 {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest<T: std::cmp::PartialOrd>(list:&[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest 
}

/** ------------------------------------------------------------------------------------ */

/** Struct 제너릭 선언 */
#[derive(Debug)]
struct Point1<T> {
    x:T,
    y:T
}

fn struct_generics_fn1() {
    let p1 = Point1 {x:2, y:3};
    let p2 = Point1 {x:'이',y:'삼'};
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}

#[derive(Debug)]
struct Point2<T,U> {
    x:T,
    y:U
}

fn struct_generics_fn2() {
    let p1 = Point2 {x:2, y:'삼'};
    println!("p1 = {:?}", p1);
}

/** ------------------------------------------------------------------------------------ */

#[derive(Debug)]
struct Point3<T> {
    x:T,
    y:T
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
