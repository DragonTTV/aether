use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::library::Library;
use crate::player::{Metadata, Track};
use std::time::Duration;
use crate::playlist::Playlist;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open() -> Result<Self, String> {
        let path = database_path()?;

        let connection = Connection::open(path)
            .map_err(|e| e.to_string())?;
        connection.execute_batch("PRAGMA foreign_keys = ON;").map_err(|e| e.to_string())?;
        Ok(Self { connection })
    }
    pub fn initialize(&self) -> Result<(), String> {
        self.connection
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS tracks (
                    id           INTEGER PRIMARY KEY,
                    source       TEXT NOT NULL UNIQUE,
                    title        TEXT,
                    artist       TEXT,
                    duration_ms  INTEGER,
                    album        TEXT,
                    genre        TEXT,
                    track_number INTEGER,
                    disc_number  INTEGER,
                    artwork      TEXT,
                    release_date TEXT
                );

                CREATE TABLE IF NOT EXISTS scan_paths (
                    path TEXT PRIMARY KEY
                );
                CREATE TABLE IF NOT EXISTS playlists (
                    id   INTEGER PRIMARY KEY,
                    name TEXT NOT NULL UNIQUE
                );

                CREATE TABLE IF NOT EXISTS playlist_tracks (
                    playlist_id INTEGER NOT NULL,
                    track_id    INTEGER NOT NULL,
                    position    INTEGER NOT NULL,

                    PRIMARY KEY (playlist_id, position),
                    FOREIGN KEY (playlist_id)
                        REFERENCES playlists(id)
                        ON DELETE CASCADE
                );
                ",
            )
            .map_err(|e| e.to_string())
    }

    pub fn load_library(&self) -> Result<Library, String> {
        let mut library = Library::new();

        let mut statement = self
            .connection
            .prepare(
                "
                    SELECT
                        id,
                        source,
                        title,
                        artist,
                        duration_ms,
                        album,
                        genre,
                        track_number,
                        disc_number,
                        artwork,
                        release_date
                    FROM tracks
                    ORDER BY id
                ",
            )
            .map_err(|e| e.to_string())?;

        let tracks = statement
            .query_map([], |row| {
                let duration_ms: Option<i64> = row.get(4)?;
                let id: i64 = row.get(0)?;
                Ok(Track {
                    id: id as u64,
                    source: row.get(1)?,
                    metadata: Metadata {
                        title: row.get(2)?,
                        artist: row.get(3)?,
                        duration: duration_ms.map(|ms| Duration::from_millis(ms as u64)),
                        album: row.get(5)?,
                        genre: row.get(6)?,
                        track_number: row.get(7)?,
                        disc_number: row.get(8)?,
                        artwork: row.get(9)?,
                        release_date: row.get(10)?,
                    },
                })
            })
            .map_err(|e| e.to_string())?;

        for track in tracks {
            let track = track.map_err(|e| e.to_string())?;
            let id = track.id;

            library.add_with_id(track, id);
        }

        let mut statement = self
            .connection
            .prepare("SELECT path FROM scan_paths ORDER BY path")
            .map_err(|e| e.to_string())?;

        let paths = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        for path in paths {
            library.add_scan_path(path.map_err(|e| e.to_string())?);
        }

        Ok(library)
    }

    pub fn save_library(&mut self, library: &Library) -> Result<(), String> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|e| e.to_string())?;

        transaction
            .execute("DELETE FROM tracks", [])
            .map_err(|e| e.to_string())?;

        transaction
            .execute("DELETE FROM scan_paths", [])
            .map_err(|e| e.to_string())?;

        for track in library.tracks() {
            let duration_ms = track
                .metadata
                .duration
                .map(|duration| duration.as_millis() as i64);

            transaction
                .execute(
                    "
                    INSERT INTO tracks (
                        id,
                        source,
                        title,
                        artist,
                        duration_ms,
                        album,
                        genre,
                        track_number,
                        disc_number,
                        artwork,
                        release_date
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                    ",
                    params![
                        track.id as i64,
                        track.source,
                        track.metadata.title,
                        track.metadata.artist,
                        duration_ms,
                        track.metadata.album,
                        track.metadata.genre,
                        track.metadata.track_number,
                        track.metadata.disc_number,
                        track.metadata.artwork,
                        track.metadata.release_date,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }

        for path in library.scan_paths() {
            transaction
                .execute(
                    "INSERT INTO scan_paths (path) VALUES (?1)",
                    params![path],
                )
                .map_err(|e| e.to_string())?;
        }

        transaction.commit().map_err(|e| e.to_string())
    }

    pub fn is_library_empty(&self) -> Result<bool, String> {
        let track_count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        let path_count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM scan_paths", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        Ok(track_count == 0 && path_count == 0)
    }

    pub fn create_playlist(&self, name: &str) -> Result<u64, String> {
        self.connection
            .execute(
                "INSERT INTO playlists (name) VALUES (?1)",
                params![name],
            )
            .map_err(|e| e.to_string())?;

        Ok(self.connection.last_insert_rowid() as u64)
    }

    pub fn list_playlists(&self) -> Result<Vec<Playlist>, String> {
        let mut statement = self
            .connection
            .prepare("SELECT id, name FROM playlists ORDER BY name")
            .map_err(|e| e.to_string())?;

        let playlists = statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;

                Ok(Playlist {
                    id: id as u64,
                    name: row.get(1)?,
                    track_ids: Vec::new(),
                })
            })
            .map_err(|e| e.to_string())?;

        playlists
            .map(|playlist| playlist.map_err(|e| e.to_string()))
            .collect()
    }

    pub fn get_playlist(&self, id: u64) -> Result<Option<Playlist>, String> {
        let result = self.connection.query_row(
            "SELECT id, name FROM playlists WHERE id = ?1",
            params![id as i64],
            |row| {
                let id: i64 = row.get(0)?;

                Ok(Playlist {
                    id: id as u64,
                    name: row.get(1)?,
                    track_ids: Vec::new(),
                })
            },
        );

        let mut playlist = match result {
            Ok(playlist) => playlist,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(error) => return Err(error.to_string()),
        };

        let mut statement = self
            .connection
            .prepare(
                "
                SELECT track_id
                FROM playlist_tracks
                WHERE playlist_id = ?1
                ORDER BY position
                ",
            )
            .map_err(|e| e.to_string())?;

        let track_ids = statement
            .query_map(params![id as i64], |row| {
                let track_id: i64 = row.get(0)?;
                Ok(track_id as u64)
            })
            .map_err(|e| e.to_string())?;

        for track_id in track_ids {
            playlist.track_ids.push(track_id.map_err(|e| e.to_string())?);
        }

        Ok(Some(playlist))
    }

    pub fn add_tracks_to_playlist(&mut self, playlist_id: u64, track_ids: &[u64]) -> Result<(), String> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|e| e.to_string())?;

        let mut next_position: i64 = transaction
            .query_row(
                "
                SELECT COALESCE(MAX(position) + 1, 0)
                FROM playlist_tracks
                WHERE playlist_id = ?1
                ",
                params![playlist_id as i64],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        for track_id in track_ids {
            transaction
                .execute(
                    "
                    INSERT INTO playlist_tracks (
                        playlist_id,
                        track_id,
                        position
                    )
                    VALUES (?1, ?2, ?3)
                    ",
                    params![playlist_id as i64, *track_id as i64, next_position],
                )
                .map_err(|e| e.to_string())?;

            next_position += 1;
        }

        transaction.commit().map_err(|e| e.to_string())
    }

    pub fn remove_track_from_playlist(&mut self, playlist_id: u64, position: usize) -> Result<bool, String> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|e| e.to_string())?;

        let removed = transaction
            .execute(
                "
                DELETE FROM playlist_tracks
                WHERE playlist_id = ?1 AND position = ?2
                ",
                params![playlist_id as i64, position as i64],
            )
            .map_err(|e| e.to_string())?;

        if removed == 0 {
            return Ok(false);
        }

        transaction
            .execute(
                "
                UPDATE playlist_tracks
                SET position = position - 1
                WHERE playlist_id = ?1 AND position > ?2
                ",
                params![playlist_id as i64, position as i64],
            )
            .map_err(|e| e.to_string())?;

        transaction.commit().map_err(|e| e.to_string())?;

        Ok(true)
    }

    pub fn delete_playlist(&self, id: u64) -> Result<bool, String> {
        let deleted = self
            .connection
            .execute(
                "DELETE FROM playlists WHERE id = ?1",
                params![id as i64],
            )
            .map_err(|e| e.to_string())?;

        Ok(deleted > 0)
    }

    pub fn rename_playlist(&self, id: u64, name: &str) -> Result<bool, String> {
        let updated = self
            .connection
            .execute(
                "UPDATE playlists SET name = ?1 WHERE id = ?2",
                params![name, id as i64],
            )
            .map_err(|e| e.to_string())?;

        Ok(updated > 0)
    }

    pub fn clear_playlist(&self, playlist_id: u64) -> Result<usize, String> {
        self.connection
            .execute(
                "DELETE FROM playlist_tracks WHERE playlist_id = ?1",
                params![playlist_id as i64],
            )
            .map_err(|e| e.to_string())
    }

    pub fn move_track_in_playlist(&mut self, playlist_id: u64, from: usize, to: usize) -> Result<bool, String> {
        let playlist = self
            .get_playlist(playlist_id)?
            .ok_or("Playlist not found.")?;

        if from >= playlist.track_ids.len() || to >= playlist.track_ids.len() {
            return Ok(false);
        }

        if from == to {
            return Ok(true);
        }

        let mut track_ids = playlist.track_ids;

        let track_id = track_ids.remove(from);
        track_ids.insert(to, track_id);

        let transaction = self
            .connection
            .transaction()
            .map_err(|e| e.to_string())?;

        transaction
            .execute(
                "DELETE FROM playlist_tracks WHERE playlist_id = ?1",
                params![playlist_id as i64],
            )
            .map_err(|e| e.to_string())?;

        for (position, track_id) in track_ids.iter().enumerate() {
            transaction
                .execute(
                    "
                    INSERT INTO playlist_tracks (
                        playlist_id,
                        track_id,
                        position
                    )
                    VALUES (?1, ?2, ?3)
                    ",
                    params![
                        playlist_id as i64,
                        *track_id as i64,
                        position as i64
                    ],
                )
                .map_err(|e| e.to_string())?;
        }

        transaction.commit().map_err(|e| e.to_string())?;

        Ok(true)
    }
    pub fn remove_missing_tracks_in_playlist(&mut self, playlist_id: u64, library: &Library) -> Result<usize, String>{
        let playlist = self
            .get_playlist(playlist_id)?
            .ok_or("Playlist not found")?;
        let origianl_length = playlist.track_ids.len();
        let track_ids: Vec<u64> = playlist
            .track_ids
            .into_iter()
            .filter(|id| library.get(*id).is_some())
            .collect();
        let removed = origianl_length - track_ids.len();
        let transaction = self
            .connection
            .transaction()
            .map_err(|e| e.to_string())?;
        transaction
            .execute(
                "DELETE FROM playlist_tracks WHERE playlist_id = ?1", 
                params![playlist_id as i64],
            )
            .map_err(|e| e.to_string())?;
        for (position, track_id) in track_ids.iter().enumerate(){
            transaction
                .execute(
                    "
                    INSERT INTO playlist_tracks(
                        playlist_id,
                        track_id,
                        position
                    )
                    VALUES(?1, ?2, ?3)
                    ",
                    params![playlist_id as i64, *track_id as i64, position as i64],    
                )
                .map_err(|e| e.to_string())?;
        }
        transaction.commit().map_err(|e| e.to_string())?;
        Ok(removed)
    }

    pub fn get_playlists(&self) -> Result<Vec<Playlist>, String>{
        let mut statment = self
            .connection
            .prepare("SELECT id FROM playlists ORDER BY id")
            .map_err(|e| e.to_string())?;
        let ids = statment
            .query_map([], |row|row.get::<_, i64>(0))
            .map_err(|e| e.to_string())?;

        let mut playlists = Vec::new();
        for id in ids{
            let id = id.map_err(|e|e.to_string())? as u64;
            if let Some(playlist) = self.get_playlist(id)? {
                playlists.push(playlist)
            }

        }
        Ok(playlists)
    } 

    pub fn update_playlist_track_ids(&mut self, id_map: &HashMap<u64, u64>,) -> Result<(), String>{
        let playlists = self.get_playlists()?;
        let transaction = self.connection.transaction().map_err(|e|e.to_string())?;

        for playlist in playlists{
            transaction
                .execute(
                    "DELETE FROM playlist_tracks WHERE playlist_id = ?",
                    [playlist.id as i64]
                )
                .map_err(|e|e.to_string())?;
            for(position, track_id) in playlist.track_ids.iter().enumerate(){
                let new_id = *id_map.get(track_id).unwrap_or(track_id);

                transaction
                    .execute(
                        "INSERT INTO playlist_tracks (playlist_id, track_id, position)
                            VALUES (?, ?, ?)",
                        (playlist.id as i64, new_id as i64, position as i64))
                    .map_err(|e|e.to_string())?;
            }
        }

        transaction.commit().map_err(|e|e.to_string())?;
        Ok(())
    }

}

fn database_path() -> Result<PathBuf, String> {
    let mut path = dirs::data_local_dir()
        .ok_or("Could not determine local data directory.")?;

    path.push("aether");

    std::fs::create_dir_all(&path)
        .map_err(|e| e.to_string())?;

    path.push("aether.db");

    Ok(path)
}