use clap::{Parser, Subcommand};
use clap::ValueEnum;
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
        source: String,
        /// Play immediately, skipping the queue.
        #[arg(short = 'n', long = "now")]
        now:bool,
        /// Treat the source as a library track ID.
        #[arg(short = 'i', long = "id")]
        id: bool,
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
    /// Manage the playlist.
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
    Volume{
        /// Volume level from 0 to 100.
        level: u8
    },
    /// Display the current playback status.
    Status,
    /// Manage the Aether Daemon
    Daemon{
        #[command(subcommand)]
        subcommand:DaemonCommand
    }
}
#[derive(Subcommand)]
pub enum QueueCommand{
    /// Add a track to the queue.
    Add{
        /// File, directory, or URL to add.
        source: String,
        /// Treat the source as a library track ID.
        #[arg(short = 'i', long = "id")]
        id: bool,
    },
    /// Remove a track from the queue.
    Remove{
        /// Index of the track in the queue.
        index: usize
    },
    /// Display the current queue.
    List,
    /// Clears the current queue.
    Clear
}


#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Title,
    Artist,
    Album,
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortBy::Title => "title",
            SortBy::Artist => "artist",
            SortBy::Album => "album",
        }
    }
}

#[derive(Subcommand)]
pub enum LibraryCommand{
    /// Scan a directory and add all supported audio files to the library.
    Scan{
        /// Directory to scan recursively.
        path: String,
    },
    /// Display all tracks in the library.
    List{
        #[arg(short, long)]
        sort: Option<SortBy>,
    },
    /// Search the library by title, artist, album, or filename.
    Search {
        /// Search query.
        query: String,
    },
    ///Displays information about track of a particular ID.
    Info{
        /// Library track ID.
        id: u64,
    },
    /// Rescan all previously added library directories.
    Rescan,
}
#[derive(Subcommand)]
pub enum PlaylistCommand{
    // To be added
}

#[derive(Subcommand)]
pub enum DaemonCommand {
    /// Start the Aether daemon.
    Start,
    /// Stop the Aether daemon.
    Stop,
    /// Restart the Aether daemon.
    Restart,
    /// Display the daemon status.
    Status
}
