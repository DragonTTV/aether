use clap::{Parser, Subcommand};

#[derive(Parser)]

#[command(version, about, long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Command,
}   
#[derive(Subcommand)]
pub enum Command{
    /// Play a local file or URL.
    Play{
        /// File, directory or URL to play.
        source: String
    },
    /// Pauses the current song playing.
    Pause,
    /// Resumes the current song.
    Resume,
    /// Stops the current song.
    Stop,
    /// Plays the next song in queue.
    Next,
    /// Play the previous track in the queue.
    Prev,
    /// Search the library and supported online sources.
    Search{
        /// Search query for local and online sources.
        query: String
    },
    /// Manage the playback queue.
    Queue{
        #[command(subcommand)] 
        subcommand:QueueCommand,
    },
    /// Manage the playback queue.
    Playlist{
        #[command(subcommand)] 
        subcommand: PlaylistCommand
    },
    /// Manage the local music library.
    Library{
        #[command(subcommand)] 
        subcommand:LibraryCommand
    },
    /// Display information about the currently playing track.
    Now,
    /// Set the playback volume (0–100).
    Volume{level: u8},
}
#[derive(Subcommand)]
pub enum QueueCommand{
    /// Add a track to the queue.
    Add{
        /// File, directory, or URL to add.
        source: String
    },
    /// Remove a track from the queue.
    Remove{
        /// Index of the track in the queue.
        index: usize
    },
    /// Display the current queue.
    List,
    /// Display the current queue.
    Clear
}
#[derive(Subcommand)]
pub enum LibraryCommand{
//to be added
}
#[derive(Subcommand)]
pub enum PlaylistCommand{
//to be added
}
