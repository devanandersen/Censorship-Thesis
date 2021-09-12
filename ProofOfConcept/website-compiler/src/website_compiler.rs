use std::error::Error;

pub async fn get_website(url: &str) -> Result<String, Box<dyn Error>> {
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("{}", res.as_str().to_string());

    Ok(res.as_str().to_string())
}
