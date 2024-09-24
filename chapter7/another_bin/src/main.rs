use my_library::library::{book::Book, bookshelf::Bookshelf};
fn main() {
    let mut shelf = Bookshelf::new();
    let book1 = Book::new("すごいぞChatGPT!AIを使って学ぼうRust!", "山田太郎");
    let book2 = Book::new("Pythonプログラミング入門", "山田花子");
    shelf.add_book(book1);
    shelf.add_book(book2);
    let found_books = shelf.search_books("chatgpt");
    println!("{:?}", found_books);
}
