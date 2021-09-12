mod website_compiler;

#[tokio::main]
async fn main() {
    let website_one = website_compiler::get_website("https://facebook.com").await;
    println!("{}", website_one);
    let website_two = website_compiler::get_website("https://google.com").await;
    println!("{}", website_two);
}
