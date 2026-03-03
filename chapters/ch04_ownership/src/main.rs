// Chapter 4: Ownership
fn main() {
    ownership_concept();
    borrowing_concept();
    slice_concept();
}

fn ownership_concept() {
    println!("--- Ownership ---");
    // String型はヒープに確保されるため、所有権の移動（ムーブ）が発生します。
    // 文字列リテラルはスタックに格納されるため、所有権の移動は発生しません。
    let s1 = String::from("hello");

    // s1からs2へ所有権が移動します。
    // ムーブ後はs1を使用できません。
    let s2 = s1;
    // println!("{}, world!", s1); // コンパイルエラーになります
    println!("s2: {}", s2);

    // clone()を使うとヒープデータのディープコピーが行えます。
    // ディープコピーは、元のデータと同じ内容を持つ新しいデータを作成します。
    // これにより、s2とs3は別々の所有権を持ち、両方とも有効になります。
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}

fn borrowing_concept() {
    println!("\n--- Borrowing ---");
    let mut s = String::from("hello");

    // 不変参照を渡す（借用）
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // 可変参照を渡す（ミュータブルな借用）
    change(&mut s);
    println!("Changed string: {}", s);
}

fn calculate_length(s: &String) -> usize {
    // 参照（&）を受け取っているため、所有権は移動しません
    s.len()
}

fn change(s: &mut String) {
    // 可変参照（&mut）を受け取ると、元の値を変更できます
    s.push_str(", world");
}

fn slice_concept() {
    println!("\n--- Slices ---");
    let s = String::from("hello world");

    // 文字列スライス（一部分の参照）
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice1: {}, slice2: {}", hello, world);

    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // 全体をスライスとして返す
}
