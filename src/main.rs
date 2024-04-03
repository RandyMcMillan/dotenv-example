fn main() {
    dotenv::dotenv().ok();
    for (k, v) in std::env::vars() {
        //eprintln!("{}={}", k, v);
        eprintln!("{}={}", k, v);
    }
}
