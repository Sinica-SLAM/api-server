use std::time::{SystemTime, UNIX_EPOCH};

use api_server::routes::authorization::{Claims, JWT_SECRET};
use clap::Parser;
use jsonwebtoken::{encode, EncodingKey, Header};
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub id: i32,

    #[clap(short, long, default_value_t = 7776000)]
    pub expire: u64,
}

pub fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
fn main() {
    let args = Args::parse();

    print!("{}", args.expire);

    let claims = Claims {
        id: args.id,
        exp: args.expire + get_current_timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .expect("token encode fail");

    println!("Token: {}", token);
}
