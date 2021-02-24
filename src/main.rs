use std::io;
extern crate reqwest;
extern crate json;
extern crate scraper;
use reqwest::Client;
use std::collections::HashMap;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut num_artists = String::new();
    println!("How many artists?");

    io::stdin()
        .read_line(&mut num_artists)
        .expect("Failed to read line");

    let mut num_artists: u32 = num_artists.trim().parse().expect("Please type a number!");
    let mut artists = Vec::new();
    
    while num_artists > 0 {
        println!("Next artist");

        let mut artist = String::new();

        io::stdin()
            .read_line(&mut artist)
            .expect("Failed to read line");

        artists.push(artist);
        num_artists -= 1;
    }

    let client = Client::new();


    for artist in artists {
        let location = loc(&artist, &client);
        //match location {
            //Some(x) => println!("{}", x),
            //None => {},
        //}
        println!("{}", location.await?);
    }
    Ok(())
}

async fn loc(name: &str, client: &Client) -> Result<String, reqwest::Error>
{
    //let mut wiki_name = name.split(" ").collect();
    let wiki_name = "Metro_Boomin";
    let wiki_url = &(format!("https://en.wikipedia.org/w/api.php?action=query&prop=revisions&rvprop=content&format=json&titles={}&rvsection=0", wiki_name)).to_string();
    let resp = client.get(wiki_url).send().await?.text().await?;
    let pages = &json::parse(&resp).unwrap()["query"]["pages"];
    let entry = &pages.entries().next().unwrap().1["revisions"][0]["*"];
    println!("{}", entry.pretty(2));
    
    Ok("".to_string())
}
