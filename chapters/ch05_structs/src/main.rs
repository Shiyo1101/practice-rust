// Chapter 5: Structs (TODO)

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 引数selfは、Rectangle構造体のインスタンスを指す参照です。
    // 実際は self: &Self と書くこともできますが、慣習的にselfとだけ書かれます。
    // Rust処理系は、selfが構造体のインスタンスを指すことを理解しているためコンパイルエラーにはなりません。
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
