use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::Duration;

use crate::common::*;

use chrono::prelude::*;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct Client {
    pub(crate) app_id: String,
    pub(crate) secret: String,
    pub(crate) client: HttpClient,
    pub(crate) access_token: RwLock<Option<AccessToken>>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub(crate) access_token: String,
    pub(crate) expires_in: i64,
    pub(crate) dt: DateTime<Local>,
}

pub trait TokenRepo {
    fn save(&self, token: &AccessToken) -> Result<()>;
    fn read(&self) -> Result<AccessToken>;
}

impl Client {
    pub fn new(app_id: String, secret: String) -> Self {
        let repo = FileTokenRepo::default();
        let token = repo.read().map(Some).unwrap_or(None);
        Client {
            app_id,
            secret,
            client: HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
            access_token: RwLock::new(token),
        }
    }
}

pub struct FileTokenRepo {
    path: PathBuf,
}

impl Default for FileTokenRepo {
    fn default() -> Self {
        FileTokenRepo {
            path: PathBuf::from("./.token"),
        }
    }
}

impl FileTokenRepo {
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl TokenRepo for FileTokenRepo {
    fn save(&self, token: &AccessToken) -> Result<()> {
        let file = File::create(self.get_path())?;
        serde_json::to_writer(file, token).map_err(From::from)
    }

    fn read(&self) -> Result<AccessToken> {
        let mut file = File::open(self.get_path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_json::from_str::<AccessToken>(&content).map_err(From::from)
    }
}
