struct Book{
    name:String,
    author:String,
    is_available: bool,
}

struct Patron{
    name:String,
    id:u32,
    fines_due:u32,
}

impl Patron {
    fn update_balance(&mut self, payed_bill:u32) {  
        self.fines_due = self.fines_due - payed_bill;
    }
}

impl Book{
     fn book_availability(&mut self) {
        self.is_available = false;
    }
}




fn main(){
    let mut book1 = Book{
        name: String::from("Bossypants"),
        author:String::from("tina fey"),
        is_available: true,
    };

    let mut patron1 = Patron{
        name: String::from("Alexa"),
        id: 123456,
        fines_due: 2,
    };

    patron1.update_balance(2);
    book1.book_availability();

    println!("the book availability: {}", book1.is_available);
    println!("patron fines: {}", patron1.fines_due);

    println!("book info: name: {} | author: {} | availability: {}", book1.name, book1.author, book1.is_available);
    println!("patron info: name: {} | id: {} | fines: {}", patron1.name, patron1.id, patron1.fines_due);

}
