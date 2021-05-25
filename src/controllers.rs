use rocket_contrib::json::Json;
use crate::types;

static mut COUNT: i64 = 0;
static mut BOOKS: Vec<types::Book> = Vec::new(); 

#[post("/book", format = "json", data = "<req>")]
pub fn create(req: Json<types::CreateBookRequest>) -> &'static str {
    unsafe {
        let book = types::Book{
            id: COUNT,
            title: req.title.to_string(),
            author: req.author.to_string()
        };
        
        BOOKS.push(book);    
        COUNT = COUNT + 1;
        
        println!("{:?}", BOOKS);
        "Created!"
    }
}

#[get("/books")]
pub fn read() -> &'static str {
    "Read"
}

#[put("/book", format = "json", data = "<req>")]
pub fn update(req: Json<types::UpdateBookRequest>) -> &'static str {
    print!("{}\n", req.id);
    print!("{:?}\n", req.title);
    print!("{:?}\n", req.author);
    "Updated!"
}

#[delete("/book", format = "json", data = "<req>")]
pub fn delete(req: Json<types::DeleteBookRequest>) -> &'static str {
    print!("{}\n", req.id);
    "Deleted!"
}
