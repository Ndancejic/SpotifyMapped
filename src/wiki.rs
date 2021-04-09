extern crate reqwest;
extern crate select;
extern crate regex;
extern crate google_maps;
extern crate num_traits;

use select::document::Document;
use select::predicate::{Class};
use tokio1::runtime::Runtime;
use regex::Regex;
use num_traits::cast::ToPrimitive;
use std::env;

use google_maps::prelude::*;

pub async fn get_location(artist: &str) -> (i64, i64)
{
    // string of origin location
    let mut origin = String::new();

    // set up the wikipedia url
    let mut url = String::from("https://en.wikipedia.org/wiki/");
    url.push_str(&artist.replace(" ","_"));

    // reqwest depends on tokio 1.x so create a new runtime
    // create a string of the urls html
    let rt = match Runtime::new()
    {
        Ok(res) => res,
        Err(_err) => panic!()
    };
    let mut body = String::new();
    rt.block_on
    (
        async {
            let resp = match reqwest::get(url)
                .await
            {
                Ok(res) => res,
                Err(_err) => panic!()
            };
            body = match resp.text()
                .await
            {
                Ok(res) => res,
                Err(_err) => panic!()
            };
        }
    );

    // create a document from the html
    let document = Document::from(&*body);
    let reg = match Regex::new(r"[[:alpha:]]+, [[:alpha:]]+,? ?[[:alpha:]]*")
    {
        Ok(res) => res,
        Err(_err) => panic!()
    };

    for node in document.find(Class("infobox-label")) {
        if node.text() == "Origin"
        {
            origin = match node.next()
            {
                Some(res) => res.text().clone(),
                None => continue
            };
            break;
        }
        else if node.text() == "Born"
        {
            origin.clear();
            let temp = match node.next()
            {
                Some(res) => res.text().clone(),
                None => continue
            };
            origin = match reg.find(&temp)
            {
                Some(res) => res.as_str().to_string().clone(),
                None => continue
            }
        }
    }

    if origin != ""
    {
        let key = "GOOGLE_API";
        let val = match env::var(key) {
            Ok(res) => res,
            Err(_err) =>
            {
                println!("please run $ export GOOGLE_API=\"YOUR API KEY\"");
                return (0,0);
            }
        };
        // set up google maps client
        let mut google_maps_client = ClientSettings::new(&val);
        let location = match google_maps_client.geocoding()
            .with_address(&origin)
            .execute()
            {
                Ok(res) => res.results[0].geometry.location.to_owned(),
                Err(_err) => panic!()
            };
        let lat = location.lat.to_i64().unwrap();
        let lng = location.lng.to_i64().unwrap();
        return (lat, lng);
    }
    return (0,0);
}
