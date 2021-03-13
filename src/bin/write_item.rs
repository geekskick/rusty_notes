extern crate diesel_demo;
extern crate clap;

use self::diesel_demo::models::NewItem;
use self::diesel_demo::{create_item, establish_connection};
use clap::{App, Arg};

fn main(){

    let args = App::new("Create a to do list item")
                .arg(Arg::with_name("title").value_name("title").takes_value(true).required(true).help("The title of the item in the todo list"))
                .arg(Arg::with_name("detail").short("d").long("detail").value_name("detail").takes_value(true).help("Any required detail about the item in the to do list")).get_matches();
    let title = args.value_of("title").unwrap();
    let detail = args.value_of("detail").unwrap_or("");
    println!("{}, {}", title, detail);
    let item = NewItem::new(title, detail);
    println!("{:?}", item);
    println!("Inserted {} items", create_item(&establish_connection(), item));
}