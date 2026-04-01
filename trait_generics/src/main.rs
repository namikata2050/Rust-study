// // Greetableをもつ型にはgreetという関数が定義される
// trait Greetable {
//     fn greet(&self) -> String;
// }

// // i32型にGreetableをもたせ、greet関数の中身を定義する
// impl Greetable for i32 {
//     fn greet(&self) -> String {
//         format!("Hello, {}!", self.to_string())
//     }
// }

// fn main() {
//     let greetable = test_trait1();
//     println!("{}", test_trait2(greetable));
// }

// // 無からi32型である5を返すが、これはGreetable型ももつ
// fn test_trait1() -> impl Greetable {
//     5
// }

// // Greetable型の変数を受け取り、greet関数を適用する
// fn test_trait2(greetable: impl Greetable) -> String {
//     greetable.greet()
// }

// Display traitをもってくる
use std::fmt::Display;

// 任意の型をもつジェネリック型TがDisplay型ももつとする
// trait境界なしでは、任意の型を引数にとる関数しか定義できず不十分
fn test<T: Display>(item: T) {
    println!("{}", item);
}

fn main() {
    test(42);
    test("hoge");
    test(3.14);
}