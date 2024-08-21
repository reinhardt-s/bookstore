use library::{Book, Library};

#[test]
fn test_add_book() {
    let mut library = Library::new();
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        year: 2018,
    };
    library.add_book(book);
    let retrieved_book = library.get_book_by_title("The Rust Programming Language").expect("Book not found");
    assert_eq!(retrieved_book.author, "Steve Klabnik and Carol Nichols");
}


#[test]
fn test_remove_book() {
    let mut library = Library::new();
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        year: 2018,
    };
    library.add_book(book);
    let _ = library.remove_book("The Rust Programming Language");
    assert!(library.get_book_by_title("The Rust Programming Language").is_none());
}

#[test]
fn test_get_book_by_author() {
    let mut library = Library::new();
    let book1 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        year: 2018,
    };
    let book2 = Book {
        title: String::from("Programming Rust"),
        author: String::from("Jim Blandy and Jason Orendorff"),
        year: 2017,
    };
    library.add_book(book1);
    library.add_book(book2);
    let books_by_author = library.get_books_by_author("Steve Klabnik and Carol Nichols");
    assert_eq!(books_by_author.len(), 1);
    assert_eq!(books_by_author[0].title, "The Rust Programming Language");
}

