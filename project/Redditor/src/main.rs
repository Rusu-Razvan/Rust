use chrono::Utc;
use serde_json::Value;
use std::{collections::HashSet, io, thread, time::Duration};

fn main() -> Result<(), Box<ureq::Error>> {
    let mut input = String::new();
    let mut subreddit: &str;
    let mut sort_order: &str;
    let mut refresh_time: i32;
    let mut new_posts_found: bool;

    loop {
        println!();
        println!(
            "Usage:[subreddit] [sort order(hot/new/top)] [refresh time in sec] (Default: foxes, hot, 5 secs)"
        );
        println!("Enter the name of the subreddit you want to see: ");

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        input = input.trim().to_string();

        {
            let mut parts = input.split_whitespace();
            subreddit = parts.next().unwrap_or("foxes");
            sort_order = parts.next().unwrap_or("hot");

            let refresh_time_input = parts.next().unwrap_or("5");
            match refresh_time_input.parse::<i32>() {
                Ok(value) if value > 0 => refresh_time = value,
                _ => {
                    println!("Error: Invalid refresh time. Please enter a positive integer.");
                    continue;
                }
            }
        }

        if !["hot", "new", "top"].contains(&sort_order) {
            println!("Error: Invalid sort order. Valid values are: hot, new, top.");
            continue;
        }

        let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_order);

        let response = ureq::get(&url).set("User-Agent", "reddit client").call();

        if response.is_err() {
            println!("Error: Request failed due to network or server issues.");
            continue;
        }

        if response.as_ref().unwrap().status() == 404 {
            println!("Error: Invalid subreddit or Reddit API issue.");
            continue;
        }

        let json: Value = response.unwrap().into_json().unwrap_or(Value::Null);

        if json["data"]["children"].is_null()
            || json["data"]["children"].as_array().unwrap().is_empty()
        {
            println!("Error: Invalid subreddit or Reddit API issue.");
            continue;
        }

        break;
    }

    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_order);
    let mut seen_posts: HashSet<String> = HashSet::new();

    thread::spawn(move || {
        let mut input = String::new();
        loop {
            if io::stdin().read_line(&mut input).is_ok() && input.trim() == "exit" {
                std::process::exit(0);
            }
        }
    });

    loop {
        let response = ureq::get(&url).set("User-Agent", "reddit client").call();

        if response.is_ok() {
            let json: Value = response.unwrap().into_json().unwrap_or_else(|_| {
                println!("Error: Unable to parse the response from Reddit.");
                Value::Null
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
