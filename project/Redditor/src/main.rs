use std::{collections::HashSet, io, thread, time::Duration};
fn main() -> Result<(), ureq::Error> {
    let mut input = String::new();
    let mut subreddit: &str = "foxes"; //default subreddit
    let mut sort_order: &str = "hot"; //default sort order
    let mut refresh_time: i32 = 5; //default refresh time
    let mut new_posts_found: bool;

    println!(
        "Usage:[subreddit] [sort order(hot/new/top)] [refresh time in sec] (Default: foxes, hot, 5 secs)"
    );
    println!("Enter the name of the subreddit you want to see: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();

            let mut parts = input.split_whitespace();
            subreddit = parts.next().unwrap_or("foxes");

            sort_order = parts.next().unwrap_or("hot");

            refresh_time = parts.next().unwrap_or("5").parse::<i32>().unwrap_or(5);
        }
        Err(error) => println!("error: {error}"),
    }

    let mut seen_posts: HashSet<String> = HashSet::new();

    thread::spawn(move || {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.trim() == "exit" {
                        std::process::exit(0);
                    }
                }
                Err(error) => println!("error: {error}"),
            }
        }
    });

    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_order);

    loop {
        let response = ureq::get(&url).set("User-Agent", "reddit client").call();

        if response.is_ok() {
            let json: serde_json::Value = response.unwrap().into_json()?;

            new_posts_found = false;

            for post in json["data"]["children"].as_array().unwrap() {
                let post_id = post["data"]["id"].as_str().unwrap();

                if !seen_posts.contains(post_id) {
                    seen_posts.insert(post_id.to_string());

                    println!(
                        "Title: {}",
                        post["data"]["title"].as_str().unwrap_or("[No title]")
                    );
                    println!("Author: {}", post["data"]["author"]);
                    println!("Creation date: {}", post["data"]["created_utc"]);
                    println!(
                        "URL: https://www.reddit.com{}",
                        post["data"]["permalink"].as_str().unwrap_or("#")
                    );

                    println!("\n\n");

                    new_posts_found = true;
                }
            }

            if !new_posts_found {
                println!("No new posts found!");
            }
        } else {
            println!("Error: {}", response.unwrap_err());
        }

        thread::sleep(Duration::from_secs(refresh_time as u64));
    }
}
