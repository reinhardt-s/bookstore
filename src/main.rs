use bookstore::string_utils::is_empty_string;
use library::Book;
use library::Library;
use std::io::{self, Write};

fn main() {
    let mut library = Library::new();

    loop {
        println!("What action would you like to perform?");
        println!("1. Get all books");
        println!("2. Add a book");
        println!("3. Remove a book");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => get_all_books(&library),
                    2 => add_book(&mut library),
                    3 => remove_book(&mut library),
                    4 => {
                        println!("Quitting...");
                        return;
                    }
                    _ => {
                        println!("Invalid choice. Please try again.");
                    }
                }
            }
            Err(_) => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

fn get_all_books(library: &Library) {
    println!("From which author would you like to see the books?");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read input");
    let books = library.get_books_by_author(&author);
    println!("All books:");
    for book in books {
        println!("{}", book.summary());
    }
}

fn add_book(library: &mut Library) {
    println!("Enter the book details:");
    print!("Title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");

    print!("Author: ");
    io::stdout().flush().unwrap();
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read input");

    print!("Year: ");
    io::stdout().flush().unwrap();
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("Failed to read input");
    let year: u32 = year.trim().parse().expect("Invalid year");

    if is_empty_string(&title) || is_empty_string(&author) {
        println!("Title and author cannot be empty!");
        return;
    }

    library.add_book(Book {
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        year,
    });

    println!("Book added successfully!");
}

fn remove_book(library: &mut Library) {
    println!("Enter the title of the book to remove:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");

    match library.remove_book(&title.trim()) {
        Ok(_) => {
            println!("Book removed successfully!");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}