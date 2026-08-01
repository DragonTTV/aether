use clap::ValueEnum;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(
    version = concat!(env!("CARGO_PKG_VERSION"), " (Library Release)"),
    about = "A daemon-based terminal music player",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    /// Play a local file or URL.
    Play {
        /// File, directory or URL to play.
        source: String,
        /// Play immediately, skipping the queue.
        #[arg(short = 'n', long = "now")]
        now: bool,
        /// Treat the source as a library track ID.
        #[arg(short = 'i', long = "id", conflicts_with = "playlist")]
        id: bool,
        /// Treat the source as a playlist ID.
        #[arg(short = 'p', long = "playlist", conflicts_with = "id")]
        playlist: bool,
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
    Search {
        /// Search query for local and online sources.
        query: String,
    },
    Repeat {
        mode: Option<RepeatModeArg>,
    },
    Shuffle {
        enabled: Option<ShuffleModeArg>,
    },
    /// Manage the playback queue.
    Queue {
        #[command(subcommand)]
        subcommand: QueueCommand,
    },
    /// Manage the playlist.
    Playlist {
        #[command(subcommand)]
        subcommand: PlaylistCommand,
    },
    /// Manage the local music library.
    Library {
        #[command(subcommand)]
        subcommand: LibraryCommand,
    },
    /// Display information about the currently playing track.
    Now,
    /// Set the playback volume (0–100).
    Volume {
        /// Volume level from 0 to 100.
        level: u8,
    },
    /// Display the current playback status.
    Status,
    /// Manage the Aether Daemon
    Daemon {
        #[command(subcommand)]
        subcommand: DaemonCommand,
    },
    Seek {
        /// Position in seconds
        position: u64,
    },
}
#[derive(Subcommand)]
pub enum QueueCommand {
    /// Add a track to the queue.
    Add {
        /// File, directory, or URL to add.
        source: String,
        /// Treat the source as a library track ID.
        #[arg(short = 'i', long = "id")]
        id: bool,
    },
    /// Remove a track from the queue.
    Remove {
        /// Index of the track in the queue.
        index: usize,
    },
    /// Display the current queue.
    List,
    /// Clears the current queue.
    Clear,
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
pub enum LibraryCommand {
    /// Scan a directory and add all supported audio files to the library.
    Scan {
        /// Directory to scan recursively.
        path: String,
    },
    /// Display all tracks in the library.
    List {
        #[arg(short, long)]
        sort: Option<SortBy>,
    },
    /// Search the library by title, artist, album, or filename.
    Search {
        /// Search query.
        query: String,
    },
    ///Displays information about track of a particular ID.
    Info {
        /// Library track ID.
        id: u64,
    },
    /// Rescan all previously added library directories.
    Rescan{
        /// Reassign track IDs.
        #[arg(long)]
        reid: bool
    },
}
#[derive(Subcommand)]
pub enum PlaylistCommand {
    /// Create a new playlist.
    Create {
        /// Name of the playlist.
        name: String,
    },

    /// Display all playlists.
    List,
    /// Display a playlist and its tracks.
    Show {
        /// Playlist ID.
        id: u64,
    },
    /// Add a library track to a playlist.
    Add {
        /// Playlist ID.
        playlist_id: u64,

        /// One or more library track IDs.
        #[arg(required = true, num_args = 1..)]
        track_ids: Vec<u64>, 
    },
    /// Remove a track from a playlist by position.
    Remove {
        /// Playlist ID.
        playlist_id: u64,

        /// Position of the track in the playlist.
        #[arg(required_unless_present_any = ["all", "missing"])]
        position: Option<usize>,

        /// Remove all tracks from the playlist.
        #[arg(long, conflicts_with_all = ["position", "missing"],  required_unless_present_any = ["position", "missing"])]
        all: bool,

        ///Remove any or all missing tracks from playlist.
        #[arg(long, conflicts_with_all=["all", "position"], required_unless_present_any = ["all", "position"])]
        missing: bool,
    },
    /// Delete a playlist.
    Delete {
        /// Playlist ID.
        id: u64,
    },
    /// Rename a playlist.
    Rename {
        /// Playlist ID.
        id: u64,

        /// New playlist name.
        name: String,
    },
    /// Move a track to another position in a playlist.
    Move {
        /// Playlist ID.
        playlist_id: u64,

        /// Current position of the track.
        from: usize,

        /// New position for the track.
        to: usize,
    },
    /// Display information about a playlist.
    Info {
        /// Playlist ID.
        id: u64,
    },
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
    Status,
}

#[derive(clap::ValueEnum, Clone)]
pub enum RepeatModeArg {
    Off,
    Track,
    Queue,
}
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ShuffleModeArg{
    On,
    Off,
}