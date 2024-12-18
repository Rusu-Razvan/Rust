use chrono::Utc;
use serde_json::Value;
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

    if !["hot", "new", "top"].contains(&sort_order) {
        println!("Error: Invalid sort order. Valid values are: hot, new, top.");
        return Ok(());
    }

    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_order);

    let response = ureq::get(&url).set("User-Agent", "reddit client").call();

    if response?.status() == 404 {
        println!("Error: The subreddit '{}' does not exist.", subreddit);
        return Ok(());
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

    loop {
        let response = ureq::get(&url).set("User-Agent", "reddit client").call();

        if response.is_ok() {
            let json: serde_json::Value = response.unwrap().into_json().unwrap_or_else(|_| {
                println!("Error: Unable to parse the response from Reddit.");
                return Value::Null;
            });

            if json == Value::Null {
                println!("Error: Invalid subreddit or Reddit API issue.");
                continue;
            }

            new_posts_found = false;

            if let Some(posts) = json["data"]["children"].as_array() {
                for post in posts {
                    let post_id = post["data"]["id"].as_str().unwrap();

                    if !seen_posts.contains(post_id) {
                        seen_posts.insert(post_id.to_string());

                        let created_utc = post["data"]["created_utc"].as_f64().unwrap_or(0.0);
                        let datetime =
                            chrono::DateTime::<Utc>::from_timestamp(created_utc as i64, 0)
                                .unwrap_or_else(Utc::now);

                        println!(
                            "Title: {}",
                            post["data"]["title"].as_str().unwrap_or("[No title]")
                        );
                        println!("Author: {}", post["data"]["author"]);
                        println!("Creation date: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
                        println!(
                            "URL: https://www.reddit.com{}",
                            post["data"]["permalink"].as_str().unwrap_or("#")
                        );

                        println!("\n\n");

                        new_posts_found = true;
                    }
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
