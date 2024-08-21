/// Represents a book in a library.
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    /// Generates a summary of the book.
    ///
    /// # Returns
    ///
    /// A string containing the book's title, author, and year.
    pub fn summary(&self) -> String {
        format!("{} von {} ({})", self.title, self.author, self.year)
    }
}

/// Represents a library containing a collection of books.
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    /// Creates a new instance of the library.
    ///
    /// # Returns
    ///
    /// A new `Library` instance.
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    /// Adds a book to the library.
    ///
    /// # Arguments
    ///
    /// * `book` - The book to be added.
    /// 
    /// # Example
    /// 
    /// ```
    /// use library::{Library, Book};
    ///
    /// let mut library = Library::new();
    /// library.add_book(Book {
    ///    title: "Der Herr der Ringe".to_string(),
    ///    author: "J.R.R. Tolkien".to_string(),
    ///    year: 1954,
    /// });
    /// ```
    /// 
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// Removes a book from the library.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the book to be removed.
    ///
    /// # Returns
    ///
    /// 
    /// An `Ok` result if the book was successfully removed, or an `Err` containing an error message if the book was not found.
    pub fn remove_book(&mut self, title: &str) -> Result<(), String> {
        let index = self.books.iter().position(|b| b.title == title);

        match index {
            Some(i) => {
                self.books.remove(i);
                Ok(())
            },
            None => Err(format!("Buch mit dem Titel '{}' nicht gefunden.", title)),
        }
    }

    /// Retrieves all books written by a specific author.
    ///
    /// # Arguments
    ///
    /// * `author` - The name of the author.
    ///
    /// # Returns
    ///
    /// A vector containing references to all books written by the specified author.
    pub fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    /// Retrieves a book by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the book.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the book if found, or `None` if the book was not found.
    pub fn get_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.title == title)
    }
}

// Unit tests
// See: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// Also: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book() {
        let mut library = Library::new();
        let book = Book {
            title: String::from("Rust Programming"),
            author: String::from("Steve Klabnik"),
            year: 2018,
        };
        library.add_book(book);
        assert_eq!(library.books.len(), 1);
    }

    #[test]
    fn test_remove_book() {
        let mut library = Library::new();
        let book = Book {
            title: String::from("Rust Programming"),
            author: String::from("Steve Klabnik"),
            year: 2018,
        };
        library.add_book(book);
        let result = library.remove_book("Rust Programming");
        assert!(result.is_ok());
        assert_eq!(library.books.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_book() {
        let mut library = Library::new();
        let result = library.remove_book("Nonexistent Book");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_books_by_author() {
        let mut library = Library::new();
        let book1 = Book {
            title: String::from("Rust Programming"),
            author: String::from("Steve Klabnik"),
            year: 2018,
        };
        let book2 = Book {
            title: String::from("Rust Advanced"),
            author: String::from("Steve Klabnik"),
            year: 2020,
        };
        let book3 = Book {
            title: String::from("Another Book"),
            author: String::from("Another Author"),
            year: 2021,
        };
        library.add_book(book1);
        library.add_book(book2);
        library.add_book(book3);
        let books_by_author = library.get_books_by_author("Steve Klabnik");
        assert_eq!(books_by_author.len(), 2);
    }

    #[test]
    fn test_get_book_by_title() {
        let mut library = Library::new();
        let book = Book {
            title: String::from("Rust Programming"),
            author: String::from("Steve Klabnik"),
            year: 2018,
        };
        library.add_book(book);
        let found_book = library.get_book_by_title("Rust Programming");
        assert!(found_book.is_some());
        assert_eq!(found_book.unwrap().title, "Rust Programming");
    }

    #[test]
    fn test_get_nonexistent_book_by_title() {
        let library = Library::new();
        let found_book = library.get_book_by_title("Nonexistent Book");
        assert!(found_book.is_none());
    }

    #[test]
    fn test_book_summary() {
        let book = Book {
            title: String::from("Rust Programming"),
            author: String::from("Steve Klabnik"),
            year: 2018,
        };
        let summary = book.summary();
        assert_eq!(summary, "Rust Programming von Steve Klabnik (2018)");
    }
}
