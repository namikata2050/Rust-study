fn main() {
    let mut str = String::from("Hello, world!");
    // Stringは可変で所有権をもつ文字列。strがStringの所有権をもっている
    let str2 = &str;
    // 所有権はstrのまま、str2は参照権限だけ借用
    println!("{}", str2);
    // str2が最後に使われ、ライフタイムが終了し、参照権限が解放
    str.push_str("!!");
    println!("{}", str);
}