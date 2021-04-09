extern crate reqwest;
extern crate scraper;
extern crate select;

use select::document::Document;
use select::predicate::{Class};
use tokio1::runtime::Runtime;

pub async fn get_location(artist: &str)
{
    let mut url = String::from("https://en.wikipedia.org/wiki/");

    url.push_str(&artist.replace(" ","_"));

    // reqwest depends on tokio 1.x so create a new runtime
    let rt = match Runtime::new()
    {
        Ok(res) => res,
        Err(_err) => panic!()
    };

    rt.block_on
    (
        async {
            let resp = match reqwest::get(url)
                .await
            {
                Ok(res) => res,
                Err(_err) => panic!()
            };
            let body = match resp.text()
                .await
            {
                Ok(res) => res,
                Err(_err) => panic!()
            };
            let document = Document::from(&*body);
            let mut origin = String::new();

            for node in document.find(Class("infobox-label")) {
                if node.text() == "Origin"
                {
                    origin = match node.next()
                    {
                        Some(res) => res.text().clone(),
                        None => origin
                    };
                    break;
                }
                else if node.text() == "Born"
                {
                    origin = match node.next()
                    {
                        Some(res) => res.text().clone(),
                        None => origin
                    };
                }
            }

            println!("{:?}", origin);
        }
    );
}
