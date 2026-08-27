// Library book tracker

struct book {
    title:String,
    author:String,
    pages:u32,
}

impl book {
    fn new(title:String, author:String, pages:u32) -> Self {
        Self { title, author, pages }
    }

    fn summary(&self) {
        println!("title: {} | author: {} | pages: {}", self.title, self.author, self.pages);
    }
}

fn main() {

    let book1 = book{
        title : String::from("cookies"),
        author : String::from("chocolatew"),
        pages : 30,
    };

    book1.summary();
}