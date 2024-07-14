pub fn print() {
    let a = String::from("abc");
    let b = a;
    // println!("{}", a); // bに所有権が移動するのでコンパイルエラー

    let c: u32 = 1;
    let d: u32 = c;
    println!("c = {}, d = {}", c, d); // 基本データ型はCopyトレイトを実装しているのでコンパイル可能
}
