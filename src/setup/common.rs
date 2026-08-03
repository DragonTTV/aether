use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tar::Archive;
use tempfile::tempdir;

const STABLE_API: &str = "https://api.github.com/repos/DragonTTV/aether/releases/latest";

const RELEASES_API: &str = "https://api.github.com/repos/DragonTTV/aether/releases";

#[derive(Debug, Clone)]
pub struct Release {
    pub version: String,
    pub download_url: String,
    pub checksum_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    prerelease: bool,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

pub fn fetch_latest_release(current: &Version) -> Result<Release, String> {
    if current.pre.is_empty() {
        fetch_latest_stable_release()
    } else {
        fetch_latest_prerelease()
    }
}

fn fetch_latest_stable_release() -> Result<Release, String> {
    let client = Client::new();

    let release: GithubRelease = client
        .get(STABLE_API)
        .header("User-Agent", "Aether")
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;

    release_from_github(release)
}

fn fetch_latest_prerelease() -> Result<Release, String> {
    let client = Client::new();

    let releases: Vec<GithubRelease> = client
        .get(RELEASES_API)
        .header("User-Agent", "Aether")
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;

    let release = releases
        .into_iter()
        .find(|r| r.prerelease)
        .ok_or("No prerelease found.")?;

    release_from_github(release)
}

fn release_from_github(release: GithubRelease) -> Result<Release, String> {
    let archive = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".tar.gz"))
        .ok_or("Release archive not found.")?;

    let checksum = release.assets.iter().find(|a| a.name == "SHA256SUMS");

    Ok(Release {
        version: release.tag_name,
        download_url: archive.browser_download_url.clone(),
        checksum_url: checksum.map(|c| c.browser_download_url.clone()),
    })
}

pub fn download_release(release: &Release) -> Result<PathBuf, String> {
    let client = Client::new();

    let response = client
        .get(&release.download_url)
        .header("User-Agent", "Aether")
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    let temp = tempdir().map_err(|e| e.to_string())?;

    let archive_path = temp.path().join("release.tar.gz");

    let mut file = File::create(&archive_path).map_err(|e| e.to_string())?;

    let bytes = response.bytes().map_err(|e| e.to_string())?;

    file.write_all(&bytes).map_err(|e| e.to_string())?;

    // Prevent deletion when tempdir is dropped.
    let _path = temp.keep();

    Ok(archive_path)
}

pub fn verify_checksum(archive: &Path, release: &Release) -> Result<(), String> {
    let Some(url) = &release.checksum_url else {
        return Ok(());
    };

    let client = Client::new();

    let checksum_file = client
        .get(url)
        .header("User-Agent", "Aether")
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let expected = checksum_file
        .lines()
        .next()
        .ok_or("Invalid checksum file.")?
        .split_whitespace()
        .next()
        .ok_or("Invalid checksum format.")?;

    let bytes = std::fs::read(archive).map_err(|e| e.to_string())?;

    let hash = Sha256::digest(&bytes);

    let actual = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    if actual != expected {
        return Err("Checksum verification failed.".into());
    }

    Ok(())
}

pub fn extract_archive(archive: &Path) -> Result<PathBuf, String> {
    let extract_dir = std::env::temp_dir().join("aether-update");

    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    }

    fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;

    let file = fs::File::open(archive).map_err(|e| e.to_string())?;

    let decoder = GzDecoder::new(file);

    let mut archive = Archive::new(decoder);

    archive.unpack(&extract_dir).map_err(|e| e.to_string())?;

    Ok(extract_dir)
}
