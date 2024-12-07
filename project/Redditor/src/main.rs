fn main() -> Result<(), ureq::Error> {
    let url = "https://www.reddit.com/r/pics/new.json";
    let response = ureq::get(&url)
        .set("Header", "header value")
        .call();
        
        if response.is_ok() 
        {
            let json:serde_json::Value= response.unwrap().into_json()?;
        

        for post in json["data"]["children"].as_array().unwrap() {
            println!("Title: {}", post["data"]["title"].as_str().unwrap_or("[No title]"));
            println!("Author: {}", post["data"]["author"]);
            println!("Creation date: {}", post["data"]["created_utc"]);
            println!("URL: https://www.reddit.com{}", post["data"]["permalink"].as_str().unwrap_or("#"));

            println!("\n\n");
        }

    }
       


    Ok(())
}
