// fn main() {
//     match divide(10.0, 2.0) {
//         Ok(result) => println!("結果: {}", result),
//         // divideでOk(5.0)が返ってくるので、result=5.0となる
//         Err(error) => println!("エラー: {}", error),
//     }
// }

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b != 0.0 {
//         Ok(a / b)
//         // もしa=10.0,b=2.0ならOk(5.0)を返す
//     } else {
//         Err("ゼロで割ることはできません".to_string())
//     }
// }

// fn main() -> Result<(), String> {
//     let result1 = result1()?;
//     let result2 = result2()?;
//     // ?を付けると、result型でErrが返ってきたとき、mainの処理を中断してErrを返す
//     println!("結果: {}", result1 + result2);
//     Ok(())
// }

// fn result1() -> Result<f64, String> {
//     let result1 = divide(10.0, 2.0)?;
//     Ok(result1)
// }

// fn result2() -> Result<f64, String> {
//     let result2 = divide(30.0, 5.0)?;
//     Ok(result2)
// }

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b != 0.0 {
//         Ok(a / b)
//     } else {
//         Err("ゼロで割ることはできません".to_string())
//     }
// } 

fn find_word(text: String, word: String) -> Option<usize> {
    text.find(word.as_str())
    // .find()は部分文字列を検索しインデックスを返す
}

fn main() {
    match find_word(String::from("Hello, world"), String::from("world")) {
        Some(index) => println!("'world'は{}番目にあります", index),
        None => println!("'world'は見つかりませんでした"),
    }
}