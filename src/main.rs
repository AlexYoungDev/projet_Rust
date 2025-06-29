struct Book {
    title: String,
    author: String,
    year: u32,
    available: bool,
}

fn main() {
    use std::io;

    let mut books = vec![
        Book {
            title: "Dragon Ball".to_string(),
            author: "Akira Toriyama".to_string(),
            year: 1984,
            available: true,
        },
        Book {
            title: "Le Seigneur des Anneaux".to_string(),
            author: "J.R.R. Tolkien".to_string(),
            year: 1954,
            available: true,
        },
        Book {
            title: String::from("Game of Thrones"),
            author: String::from("George R. R. Martin"),
            year: 1996,
            available: false,
        },
    ];

    loop {
        println!("\n1. Add book");
        println!("2. Show all books");
        println!("3. Show available books");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim() == "1" {
            let mut title = String::new();
            println!("Title:");
            io::stdin().read_line(&mut title).unwrap();
            let title_clean = title.trim();

            if books.iter().any(|b| b.title == title_clean) {
                println!("A book with this title already exists.");
                continue;
            }

            let mut author = String::new();
            println!("Author:");
            io::stdin().read_line(&mut author).unwrap();

            let mut year = String::new();
            println!("Year:");
            io::stdin().read_line(&mut year).unwrap();

            books.push(Book {
                title: title.trim().to_string(),
                author: author.trim().to_string(),
                year: year.trim().parse().unwrap_or(0),
                available: true,
            });
        } else if choice.trim() == "2" {
            for book in &books {
                println!("{} by {} ({}) - {}", book.title, book.author, book.year, if book.available { "Available" } else { "Borrowed" });
            }
        } else if choice.trim() == "3" {
            for book in &books {
                if book.available {
                    println!("{} by {} ({})", book.title, book.author, book.year);
                }
            }
        } else if choice.trim() == "4" {
            break;
        }
    }
}
