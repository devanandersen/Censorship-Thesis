mod website_fetcher;

#[tokio::main]
async fn main() {
    let website_one = website_fetcher::get_website("https://facebook.com").await;
    println!("{:?}", website_one);
    let website_two = website_fetcher::get_website("https://google.com").await;
    println!("{:?}", website_two);
}
