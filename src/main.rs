use token::{get_token_name, TokenKind};

fn main() {
    let kind = TokenKind::Identifier;
    let name = get_token_name(kind);

    println!("Token kind {name}");
}
