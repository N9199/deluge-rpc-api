#![allow(unused_variables)] // TODO remove this when no more unimplemented
use std::{
    collections::HashMap,
    error::Error,
    ffi::OsString,
    iter::Map,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use reqwest::{header::HeaderMap, Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub enum TorrentStatus {}
pub struct TorrentTracker {
    pub url: Url,
    pub tier: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentResponse {
    pub result: Value,
    pub id: usize,
    pub error: Option<String>,
}
pub struct TorrentOptions {}
pub struct Account {
    pub username: String,
    pub password: String,
    pub authlevel: String,
    pub authlevel_int: usize,
}
type TorrentError = Box<dyn Error>;

pub struct DelugeInterface {
    client: Client,
    ip: Ipv4Addr,
    port: Option<String>,
}
/**
TODO Change `TorrentError` to real error type
*/

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    params: Vec<Value>,
    id: usize,
}

impl Request {
    fn new(method: &str, params: Option<Vec<Value>>) -> Self {
        Self {
            method: method.to_string(),
            params: params.unwrap_or_default(),
            id: 1,
        }
    }
}

impl DelugeInterface {
    pub fn new(ip: Ipv4Addr, port: Option<String>) -> Result<Self, TorrentError> {
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
        Ok(Self { client, ip, port })
    }

    fn url(&self) -> String {
        let port = self
            .port
            .as_ref()
            .map_or_else(String::new, |x| String::from(":") + x);
        format!("http://{}{}/json", self.ip, port)
    }

    async fn request(&self, request: Request) -> Result<TorrentResponse, TorrentError> {
        log::debug!("Sending Response");
        let out = self
            .client
            .post(&self.url())
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        log::debug!("Got Response");
        Ok(out)
    }

    pub async fn login(&self, password: String) -> Result<(), TorrentError> {
        log::info!("Logging In");
        let request = Request::new("auth.login", Some(vec![json!(password)]));
        log::debug!("Login info: {:?}", &request);
        let res_json = self.request(request).await?;
        log::debug!("Login response: {:?}", &res_json);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), TorrentError> {
        log::debug!("Disconnecting");
        let request = Request::new("web.disconnect", None);
        log::debug!("{:?}", &request);
        let res_json = self.request(request).await?;
        log::debug!("{:?}", &res_json);
        Ok(())
    }

    // ! Start of Core

    pub async fn add_torrent_file_async(
        &self,
        filename: &OsString,
        filedump: &str,
        options: &TorrentOptions,
        save_state: Option<bool>,
    ) -> Result<Option<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn prefetch_magnet_metadata(
        &self,
        magnet_uri: &str,
        timeout: Option<Duration>,
    ) -> Result<(String, String), TorrentError> {
        log::debug!("Prefetching Magnet Metadata");
        let request = Request::new(
            "core.prefetch_magnet_metadata",
            timeout.map(|x| vec![json!(x.as_secs())]),
        );
        let res_json = self.request(request).await?;
        let out = res_json
            .result
            .as_array()
            .map(|x| {
                x.iter()
                    .filter_map(|x| x.as_str())
                    .map(|x| Some(x.to_owned()))
                    .collect::<Vec<_>>()
            })
            .map(|mut x| match x.len() {
                2 => Some((x[0].take().unwrap(), x[1].take().unwrap())),
                _ => None,
            })
            .flatten()
            .ok_or("TorrentResponse JSON doesn't satisfy response schema")?; // Again, there's probably a better way to do this
        Ok(out)
    }

    pub async fn add_torrent_file(
        &self,
        filename: &OsString,
        filedump: &str,
        options: &TorrentOptions,
    ) -> Result<Option<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn add_torrent_files(
        &self,
        torrent_files: &[(&OsString, &str, &TorrentOptions)],
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn add_torrent_url(
        &self,
        url: Url,
        options: &TorrentOptions,
        headers: Option<&HeaderMap>,
    ) -> Result<Option<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn add_torrent_magnet(
        &self,
        uri: &str,
        options: &TorrentOptions,
    ) -> Result<String, TorrentError> {
        unimplemented!()
    }

    pub async fn remove_torrent(
        &self,
        torrent_id: &str,
        remove_data: bool,
    ) -> Result<bool, TorrentError> {
        log::debug!("Removing Torrent");
        let request = Request::new(
            "core.remove_torrent",
            Some(vec![json!(torrent_id), json!(remove_data)]),
        );
        let res_json = self.request(request).await?;
        let out = res_json
            .result
            .as_bool()
            .ok_or("TorrentResponse JSON doesn't satisfy response schema")?;
        Ok(out)
    }

    pub async fn remove_torrents(
        &self,
        torrent_ids: &[&str],
        remove_data: bool,
    ) -> Result<(), TorrentError> {
        // Actually has rich error
        unimplemented!()
    }

    pub async fn get_sessions_status(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, TorrentStatus>, TorrentError> {
        unimplemented!()
    }

    pub async fn force_reannounce(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), TorrentError> {
        log::debug!("Pausing Torrent");
        let request = Request::new("core.pause_torrent", Some(vec![json!(torrent_id)]));
        let res_json = self.request(request).await?;
        Ok(())
    }

    pub async fn pause_torrents(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        log::debug!("Pausing Torrents");
        let request = Request::new(
            "core.pause_torrents",
            Some(torrent_ids.iter().map(|x| json!(x)).collect()),
        );
        let res_json = self.request(request).await?;
        Ok(())
    }

    pub async fn connect_peer(
        &self,
        torrent_id: &str,
        ip: Ipv4Addr,
        port: u16,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn move_storage(
        &self,
        torrent_ids: &[&str],
        dest: &OsString,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn pause_session(&self) -> Result<(), TorrentError> {
        let request = Request::new("core.pause_session", None);
        let res_json = self.request(request).await?;
        Ok(())
    }
    pub async fn resume_session(&self) -> Result<(), TorrentError> {
        unimplemented!()
    }
    pub async fn is_session_paused(&self) -> Result<bool, TorrentError> {
        unimplemented!()
    }
    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), TorrentError> {
        unimplemented!()
    }
    pub async fn resume_torrents(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }
    pub async fn get_torrent_status(
        &self,
        torrent_id: &str,
        keys: &[&str],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }
    pub async fn get_torrents_status(
        &self,
        filter_dict: &Map<String, Value>,
        keys: &[&str],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }

    pub async fn get_filter_tree(
        &self,
        show_zero_hits: Option<bool>,
        hide_cat: Option<&[&str]>,
    ) -> Result<Map<String, (Value, usize)>, TorrentError> {
        unimplemented!()
    }

    pub async fn get_session_state(&self) -> Result<Vec<String>, TorrentError> {
        log::info!("Getting session state");

        let request = Request::new("core.get_session_state", None);
        log::debug!("{:?}", &request);
        let res_json = self.request(request).await?;
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

    pub async fn get_config(&self) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }

    pub async fn get_config_value(&self, key: &str) -> Result<Value, TorrentError> {
        unimplemented!()
    }

    pub async fn get_config_values(
        &self,
        keys: &[&str],
    ) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }

    pub async fn set_config(&self, config: &Map<String, Value>) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn get_listen_port(&self) -> Result<u16, TorrentError> {
        unimplemented!()
    }

    pub async fn get_proxy(&self) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }

    pub async fn get_available_plugins(&self) -> Result<Vec<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn get_enabled_plugins(&self) -> Result<Vec<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn enable_plugin(&self, plugin: &str) -> Result<bool, TorrentError> {
        unimplemented!()
    }

    pub async fn disable_plugin(&self, plugin: &str) -> Result<bool, TorrentError> {
        unimplemented!()
    }

    pub async fn force_recheck(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn set_torrent_options(
        &self,
        torrent_ids: &[&str],
        options: &TorrentOptions,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn set_torrent_trackers(
        &self,
        torrent_id: &str,
        trackers: &TorrentTracker,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn get_magnet_uri(&self, torrent_id: &str) -> Result<String, TorrentError> {
        log::debug!("Getting Magnet Uri of {}", torrent_id);
        let request = Request::new("core.get_magnet_uri", Some(vec![json!(torrent_id)]));
        let res_json = self.request(request).await?;
        let magnet_uri = res_json
            .result
            .as_str()
            .map(|x| x.to_owned())
            .ok_or("TorrentResponse JSON doesn't satisfy response schema")?;
        log::debug!("{}", &magnet_uri);
        Ok(magnet_uri)
    }

    pub async fn get_path_size(&self) -> Result<Option<usize>, TorrentError> {
        log::debug!("Getting Path Size");
        let request = Request::new("core.get_path_size", None);
        let res_json = self.request(request).await?;
        let path_size = res_json
            .result
            .as_i64()
            .ok_or("TorrentResponse JSON doesn't satisfy response schema")?;
        Ok(match path_size {
            -1 => None,
            _ => Some(path_size.try_into()?),
        })
    }
    pub async fn create_torrent(
        &self,
        path: OsString,
        tracker: TorrentTracker,
        piece_length: usize,
        comment: Option<String>,
        target: Option<OsString>,
        webseeds: Option<Vec<String>>,
        private: Option<bool>,
        created_by: Option<String>,
        trackers: Option<Vec<String>>,
        add_to_session: bool,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn upload_plugin(
        &self,
        filename: OsString,
        filedump: &[u8],
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn rescan_plugins(&self) -> Result<(), TorrentError> {
        log::debug!("Rescanning Plugins");
        let request = Request::new("core.rescan_plugins", None);
        let res_json = self.request(request).await?;
        Ok(())
    }

    pub async fn rename_files(
        &self,
        torrent_id: &str,
        filenames: &[(usize, OsString)],
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }
    pub async fn rename_folder(
        &self,
        torrent_id: &str,
        folder: OsString,
        new_folder: OsString,
    ) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn queue_top(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn queue_up(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn queue_down(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn queue_bottom(&self, torrent_ids: &[&str]) -> Result<(), TorrentError> {
        unimplemented!()
    }

    pub async fn glob(&self, path: OsString) -> Result<Vec<String>, TorrentError> {
        unimplemented!()
    }

    pub async fn test_listen_port(&self) -> Result<bool, TorrentError> {
        log::debug!("Test Listen Port");
        let request = Request::new("core.test_listen_port", None);
        let res_json = self.request(request).await?;
        let out = res_json
            .result
            .as_bool()
            .ok_or("TorrentResponse JSON doesn't satisfy response schema")?;
        Ok(out)
    }

    pub async fn get_free_space(&self, path: Option<OsString>) -> Result<usize, TorrentError> {
        unimplemented!()
    }

    pub async fn external_ip(&self) -> Result<IpAddr, TorrentError> {
        unimplemented!()
    }

    pub async fn get_libtorrent_version(&self) -> Result<String, TorrentError> {
        unimplemented!()
    }

    pub async fn get_completion_paths(
        &self,
        args: &Map<String, Value>,
    ) -> Result<Map<String, Value>, TorrentError> {
        unimplemented!()
    }
    pub async fn get_known_accounts(&self) -> Result<Vec<Account>, TorrentError> {
        unimplemented!()
    }
    pub async fn get_auth_levels_mappings(
        &self,
    ) -> Result<(Map<String, usize>, Map<usize, String>), TorrentError> {
        unimplemented!()
    }
    pub async fn create_account(&self, account: Account) -> Result<bool, TorrentError> {
        unimplemented!()
    }
    pub async fn update_account(&self, account: Account) -> Result<bool, TorrentError> {
        unimplemented!()
    }
    pub async fn remove_account(&self, username: &str) -> Result<bool, TorrentError> {
        unimplemented!()
    }

    // ! End of Core

    // ! Start of Daemon
    // pub async fn shutdown(&self){unimplemented!()}
    // pub async fn get_method_list(&self){unimplemented!()}
    // pub async fn get_version(&self){unimplemented!()}
    // pub async fn authorized_call(&self, rpc)
    // ! End of Daemon
    // ! Start of Web
    // pub async fn change_password(&self, old_password, new_password){unimplemented!()}
    // pub async fn check_session(&self, session_id=None){unimplemented!()}
    // pub async fn delete_session(&self){unimplemented!()}
    // pub async fn login(&self, password){unimplemented!()}
    // pub async fn connect(&self, host_id){unimplemented!()}
    // pub async fn connected(&self){unimplemented!()}
    // pub async fn disconnect(&self){unimplemented!()}
    // pub async fn update_ui(&self, keys, filter_dict){unimplemented!()}
    // pub async fn get_torrent_files(&self, torrent_id){unimplemented!()}
    // pub async fn download_torrent_from_url(&self, url, cookie=None){unimplemented!()}
    // pub async fn get_torrent_info(&self, filename){unimplemented!()}
    // pub async fn get_magnet_info(&self, uri){unimplemented!()}
}
