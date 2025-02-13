use tokenblitz::tokenize_text;

fn main() {
    let text = "Hello from the Rust CLI!";
    println!("Tokenizing: {}", text);
    
    let tokens = tokenize_text(text);
    println!("Tokens: {:?}", tokens);
}