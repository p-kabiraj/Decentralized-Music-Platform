#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Map, Address};

#[contract]
pub struct MusicContract;

#[derive(Clone)]
#[contracttype]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist_name: String,
    pub artist_address: Address,
    pub url: String,
}

#[contractimpl]
impl MusicContract {

    // Add a new song
    pub fn add_song(
        env: Env,
        id: u32,
        title: String,
        artist_name: String,
        artist_address: Address,
        url: String,
    ) {
        artist_address.require_auth(); // ensure artist signs transaction

        let mut songs: Map<u32, Song> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SONGS"))
            .unwrap_or(Map::new(&env));

        // Prevent duplicate IDs
        if songs.contains_key(id) {
            panic!("Song ID already exists");
        }

        let song = Song {
            id,
            title,
            artist_name,
            artist_address,
            url,
        };

        songs.set(id, song);
        env.storage().instance().set(&Symbol::new(&env, "SONGS"), &songs);
    }

    // Get song by ID
    pub fn get_song(env: Env, id: u32) -> Song {
        let songs: Map<u32, Song> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SONGS"))
            .unwrap();

        songs.get(id).unwrap()
    }

    // Total songs count
    pub fn total_songs(env: Env) -> u32 {
        let songs: Map<u32, Song> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SONGS"))
            .unwrap_or(Map::new(&env));

        songs.len()
    }
}