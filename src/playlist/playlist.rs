#[derive(Clone)]
pub struct Playlist {
    pub id: u64,
    pub name: String,
    pub track_ids: Vec<u64>,
}
