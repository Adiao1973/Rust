fn main() {
    enum Book {
        Papery(u32),
        Electronic(String)
    }

    let book  = Book::Electronic(String::from("url"));

    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}