use std::{
    collections::HashMap, error::Error, ffi::OsString, iter::Map, net::Ipv4Addr, path::Path,
    time::Duration,
};

use reqwest::{header::HeaderMap, Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum TorrentStatus {}
pub struct TorrentTracker {
    url: Url,
    tier: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub result: Value,
    pub id: usize,
    pub error: Option<String>,
}
pub struct Options {}

pub struct DelugeInterface {
    client: Client,
    ip: Ipv4Addr,
    port: Option<String>,
    password: String, // Should look into not saving this
}
/**
TODO Change `Box<dyn Error>` to real error type
*/

impl DelugeInterface {
    pub fn new(
        ip: Ipv4Addr,
        port: Option<String>,
        password: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        log::debug!("Creating Headers");
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);
        log::debug!("Creating Client");
        log::debug!("ip: {}", &ip);
        log::debug!("port: {:?}", port.as_ref());
        let client = ClientBuilder::new()
            .default_headers(headers)
            .gzip(true)
            .cookie_store(true)
            .build()?;
        let password = password.unwrap_or_else(|| String::from("deluge"));
        Ok(Self {
            client,
            ip,
            port,
            password,
        })
    }

    fn url(&self) -> String {
        let port = self
            .port
            .as_ref()
            .map_or_else(String::new, |x| String::from(":") + x);
        format!("http://{}{}/json", self.ip, port)
    }

    pub async fn test(&self) -> Result<(), Box<dyn Error>> {
        log::debug!("{}", self.url());
        log::info!("Start Test");
        self.login().await?;

        let json_info = String::from(r#"{"method": "web.connected", "params": [], "id": 1}"#);
        log::debug!("{}", &json_info);
        let res = self.client.post(self.url()).body(json_info).send().await?;
        let res_text = res.text().await?;
        let res_json: Response = serde_json::from_str(&res_text)?;
        log::debug!("{:?}", &res_json);

        self.disconnect().await?;
        log::info!("End Test");
        Ok(())
    }

    async fn login(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Logging In");
        let json_info = (
            String::from(r#"{"method": "auth.login", "params": [""#),
            (r#""], "id": 1}"#),
        );
        let json_info = json_info.0 + &self.password + json_info.1;
        log::debug!("Login info: {}", &json_info);
        let res = self.client.post(self.url()).body(json_info).send().await?;
        log::debug!("Got Response");
        let res_text = res.text().await?;
        let res_json: Response = serde_json::from_str(&res_text)?;
        log::debug!("Login response: {:?}", &res_json);
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        log::debug!("Disconnecting");
        let json_info = String::from(r#"{"method": "web.disconnect", "params": [], "id": 1}"#);
        log::debug!("{}", &json_info);
        let res = self.client.post(self.url()).body(json_info).send().await?;
        let res_text = res.text().await?;
        let res_json: Response = serde_json::from_str(&res_text)?;
        log::debug!("{:?}", &res_json);
        Ok(())
    }

    pub async fn add_torrent_file_async(
        &self,
        filename: &OsString,
        filedump: &str,
        options: &Options,
        save_state: Option<bool>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn prefetch_magnet_metadata(
        &self,
        magnet_uri: &str,
        timeout: Option<Duration>,
    ) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    pub async fn add_torrent_file(
        &self,
        filename: &OsString,
        filedump: &str,
        options: &Options,
    ) -> Result<Option<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn add_torrent_files(
        &self,
        torrent_files: &[(&OsString, &str, &Options)],
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn add_torrent_url(
        &self,
        url: Url,
        options: &Options,
        headers: Option<&HeaderMap>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn add_torrent_magnet(
        &self,
        uri: &str,
        options: &Options,
    ) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    pub async fn remove_torrent(
        &self,
        torrent_id: &str,
        remove_data: bool,
    ) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    pub async fn remove_torrents(
        &self,
        torrent_ids: &[&str],
        remove_data: bool,
    ) -> Result<(), Box<dyn Error>> {
        // Actually has rich error
        todo!()
    }

    pub async fn get_sessions_status(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, TorrentStatus>, Box<dyn Error>> {
        todo!()
    }

    pub async fn force_reannounce(&self, torrent_ids: &[&str]) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn pause_torrents(&self, torrent_ids: &[&str]) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn connect_peer(
        &self,
        torrent_id: &str,
        ip: Ipv4Addr,
        port: u16,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn move_storage(
        &self,
        torrent_ids: &[&str],
        dest: &OsString,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn pause_session(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    pub async fn resume_session(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    pub async fn is_session_paused(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    pub async fn resume_torrents(&self, torrent_ids: &[&str]) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    pub async fn get_torrent_status(
        &self,
        torrent_id: &str,
        keys: &[&str],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, Box<dyn Error>> {
        todo!()
    }
    pub async fn get_torrents_status(
        &self,
        filter_dict: &Map<String, Value>,
        keys: &[&str],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_filter_tree(
        &self,
        show_zero_hits: Option<bool>,
        hide_cat: Option<&[&str]>,
    ) -> Result<Map<String, (Value, usize)>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_session_state(&self) -> Result<Vec<String>, Box<dyn Error>> {
        log::info!("Getting session state");
        self.login().await?;

        let json_info =
            String::from(r#"{"method": "core.get_session_state", "params": [], "id": 1}"#);
        log::debug!("{}", &json_info);
        let res = self.client.post(self.url()).body(json_info).send().await?;
        log::debug!("Got Response");
        self.disconnect().await?;
        let res_text = res.text().await?;
        let res_json: Response = serde_json::from_str(&res_text)?;
        log::debug!("{:?}", &res_json);

        let out = res_json
            .result
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().unwrap_or_default().to_owned())
            .collect();
        log::debug!("{:?}", &out);
        Ok(out)
    }

    pub async fn get_config(&self) -> Result<Map<String, Value>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_config_value(&self, key: &str) -> Result<Value, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_config_values(
        &self,
        keys: &[&str],
    ) -> Result<Map<String, Value>, Box<dyn Error>> {
        todo!()
    }

    pub async fn set_config(&self, config: &Map<String, Value>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn get_listen_port(&self) -> Result<u16, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_proxy(&self) -> Result<Map<String, Value>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_available_plugins(&self) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn get_enabled_plugins(&self) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    pub async fn enable_plugin(&self, plugin: &str) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    pub async fn disable_plugin(&self, plugin: &str) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    pub async fn force_recheck(&self, torrents_id: &[&str]) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn set_torrent_options(
        &self,
        torrents_id: &[&str],
        options: &Options,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn set_torrent_trackers(
        &self,
        torrent_id: &str,
        trackers: &TorrentTracker,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn get_magnet_uri(&self, torrent_id: &str) -> Result<String, Box<dyn Error>> {
        log::debug!("Getting Magnet Uri of {}", torrent_id);
        self.login().await?;
        let json_info = (
            String::from(r#"{"method": "core.get_manget_uri", "params": [""#),
            (r#""], "id": 1}"#),
        );
        let json_info = json_info.0 + torrent_id + json_info.1;
        let json_info: Value = serde_json::from_str(&json_info)?;
        let res = self.client.post(self.url()).json(&json_info).send().await?;
        self.disconnect().await?;
        let res_json: Response = res.json().await?;
        let magnet_uri = res_json
            .result
            .as_str()
            .map(|x| x.to_owned())
            .ok_or("Response JSON doesn't satisfy response schema")?; // There's probably a better way to do this (like adding a Response type which is deserializable.)
        log::debug!("{}", &magnet_uri);
        Ok(magnet_uri)
    }
}
