fn main() {
    let mut count = 1;

    loop {
        println!("Cont: {count}");

        if count % 3 == 0 {
            println!("{count} is divisible by 3");
        }

        if count == 10 {
            break;
        }

        count += 1;
    }
}
