use std::io;
fn main() -> Result<(), ureq::Error> {

    let mut input = String::new();
    let mut subreddit:&str = "foxes"; //default subreddit
    let mut sort_order:&str = "hot"; //default sort order

    println!("Usage:[subreddit] [sort order(hot/new/top)] (Default subreddit: foxes, Default sort order: hot)");
    println!("Enter the name of the subreddit you want to see: ");


    match io::stdin().read_line(&mut input) {
        Ok(_) => {
           
           input = input.trim().to_string();
           
            let mut parts = input.split_whitespace();
             subreddit = parts.next().unwrap_or("foxes");
             sort_order = parts.next().unwrap_or("hot");
        }
        Err(error) => println!("error: {error}"),
    }


    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_order);

   
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
    else
    {
        println!("Error: {}", response.unwrap_err());
    }

    Ok(())
}
