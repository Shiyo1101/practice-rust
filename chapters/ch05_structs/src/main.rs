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

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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

    println!("The width of the rectangle is {} pixels.", rect.get_width());
    println!(
        "The height of the rectangle is {} pixels.",
        rect.get_height()
    );

    let reac1 = Rectangle {
        width: 30,
        height: 50,
    };

    let reac2 = Rectangle {
        width: 10,
        height: 40,
    };

    let reac3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can reac1 hold reac2? {}", reac1.can_hold(&reac2)); // -> true
    println!("Can reac1 hold reac3? {}", reac1.can_hold(&reac3)); // -> false

    let square = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", square.area());
}
