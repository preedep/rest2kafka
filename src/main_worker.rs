use dotenv::dotenv;

fn main() {
    dotenv().ok();

    pretty_env_logger::init();
}
