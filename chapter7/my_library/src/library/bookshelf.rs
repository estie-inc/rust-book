use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Default for Bookshelf {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher,
        }
    }

    // 本を追加するメソッド
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // タイトルで本を検索するメソッド
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        let mut found_books = vec![];
        for book in &self.books {
            let match_result = self.matcher.fuzzy_match(&book.title, title_query);
            if let Some(score) = match_result {
                if score > 0 {
                    found_books.push(book);
                }
            }
        }
        found_books
    }

    // タイトル名の完全一致で本を検索するメソッド
    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title == title_query)
            .collect()
    }
    // タイトル名の部分一致で本を検索するメソッド
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title_query))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};
    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("すごいぞChatGPT!AIを使って学ぼうRust!", "山田太郎");
        let book2 = Book::new("Pythonプログラミング入門", "山田花子");
        shelf.add_book(book1);
        shelf.add_book(book2);
        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}
