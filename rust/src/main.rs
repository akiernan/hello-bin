use figlet_rs::FIGfont;
use hello_lib::greet;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let greeting = greet(None);
    let figure = standard_font.convert(&greeting);
    println!("{}", figure.unwrap());
}
