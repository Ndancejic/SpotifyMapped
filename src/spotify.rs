extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::model::context::SimplifiedPlayingContext;
use std::error::Error;
use rspotify::util::get_token;
use rspotify::model::track::FullTrack;
use rspotify::model::artist::SimplifiedArtist;

pub struct spotify_ref
{
    spotify: Spotify,
    oauth: SpotifyOAuth,
    client_credentials: SpotifyClientCredentials,
}

impl spotify_ref
{
    pub async fn new(scope: &str, redirect_uri: &str) -> Self
    {
        let mut oauth = SpotifyOAuth::default()
            .scope(scope)
            .redirect_uri(redirect_uri)
            .build();

        let my_token = get_token(&mut oauth)
            .await;

        match (my_token)
        {
                Some(token) =>
                {
                    let client_credentials = SpotifyClientCredentials::default()
                        .token_info(token)
                        .build();

                    let spotify = Spotify::default()
                        .client_credentials_manager(client_credentials.clone())
                        .build();

                    spotify_ref
                    {
                        spotify: spotify,
                        oauth: oauth,
                        client_credentials: client_credentials,
                    }
                },
                None =>
                {
                    println!("Unable to get token");
                    panic!()
                }
        }
    }

    async fn get_client_credentials(&self, scope: &str, redirect_uri: &str)
        -> Option<SpotifyClientCredentials>
    {
        let mut oauth = SpotifyOAuth::default()
            .scope(scope)
            .redirect_uri(redirect_uri)
            .build();

        match get_token(&mut oauth).await
        {
            Some(token) => {
                return Some(SpotifyClientCredentials::default()
                    .token_info(token)
                    .build());
            },
            None => return None
        };
    }

    async fn get_current_track(&self) -> Option<FullTrack>
    {
        match self.spotify.current_playing(None)
            .await
        {
            Ok(playing_context) =>
            {
                match playing_context
                {
                    Some(full_track) => return full_track.item,
                    None => return None
                }
            },
            Err(err) =>
            {
                println!("{}", err);
                return None;
            }
        }
    }

    pub async fn get_current_song(&self, song: &mut String)
    {
        let full_track = self.get_current_track().await;

        match full_track
        {
            Some(track) =>
            {
                song.clear();
                song.push_str(&track.name);
            },
            None => song.clear()
        }
    }

    pub async fn get_current_artists(&self, artists: &mut Vec<String>)
    {
        let full_track = self.get_current_track().await;

        match full_track
        {
            Some(track) =>
            {
                artists.clear();
                for artist in track.artists
                {
                    artists.push(artist.name);
                };
            },
            None => artists.clear()
        }
    }
}
