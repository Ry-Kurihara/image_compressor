fn func(input_str: String) -> String {
    let func_str = input_str + "add_suffix"; // 文字列を結合
    println!("完成系がこれ：{}", func_str); // マクロのビックリマークは必須
    func_str // 結合後の文字列を返す
}

fn main() {
    let origin_str = String::from("original"); // 所有型文字列を生成
    let result = func(origin_str); // 所有権が `func` に移動

    // この時点で `origin_str` は無効になっている
    println!("mainで出力：{}", origin_str); // 結合された文字列を出力
}