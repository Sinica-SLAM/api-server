use api_server::routes::authorization::{Claims, JWT_SECRET};
use clap::Parser;
use jsonwebtoken::{decode, DecodingKey, Validation};
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub token: String,
}

fn main() {
    let args = Args::parse();

    match decode::<Claims>(
        args.token.as_str(),
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    ) {
        Ok(claims) => println!("{:?}", claims),
        Err(e) => println!("{:?}", e),
    }
}
