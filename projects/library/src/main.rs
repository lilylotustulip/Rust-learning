use std::io;

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

fn user_inp()-> Patron{
    println!("enter your name: ");
    let mut patron_name = String::new();
    io::stdin().read_line(&mut patron_name).expect("failed to read");
    let name = patron_name.trim();

    println!("enter your ID: ");
    let mut patron_id = String::new();
    io::stdin().read_line(&mut patron_id).expect("failed to read");
    let id: u32 = patron_id.trim().parse().expect("invalid type, enter a num please!");

    Patron{
        name: name.to_string(),
        id,
        fines_due: 0,
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

    let patron_input1 = user_inp();


    patron1.update_balance(2);
    book1.book_availability();

    println!("the book availability: {}", book1.is_available);
    println!("patron fines: {}", patron1.fines_due);

    println!("book info: name: {} | author: {} | availability: {}", book1.name, book1.author, book1.is_available);
    println!("patron info: name: {} | id: {} | fines: {}", patron1.name, patron1.id, patron1.fines_due);

    println!("patron infos: name:{} | id: {} | fines: {}", patron_input1.name, patron_input1.id, patron_input1.fines_due);

}
