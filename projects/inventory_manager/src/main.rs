enum ItemCategory{
    Book {author: String, pages: i64},
    Electronics {duration: i64},
    Clothing {size: String},
}

impl ItemCategory {
    fn category_description(&self)-> String {
        match self {
            ItemCategory::Book {author, pages} => format!("book by {author}, pages {pages}"),
            ItemCategory::Electronics {duration} => format!("electronics with {duration} month warranty"),
            ItemCategory::Clothing {size} => format!("clothing size: {size}")
        }
    }
}

fn main() {
    let book1 = ItemCategory::Book {
        author: String::from("cookie"),
        pages: 150,
    };
    println!("{}", book1.category_description());

    let electronics1 = ItemCategory::Electronics {duration: 3};
    println!("{}", electronics1.category_description());

    let clothing1 = ItemCategory::Clothing {size: String::from("M")};
    println!("{}", clothing1.category_description());

}