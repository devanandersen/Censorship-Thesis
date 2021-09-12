mod website_compiler;
use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    let future_get_website = website_compiler::get_website();
    block_on(future_get_website)
}
