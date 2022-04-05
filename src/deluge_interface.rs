#![allow(unused_variables)] // TODO remove this when no more unimplemented
use std::{
    collections::HashMap,
    ffi::OsString,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use reqwest::{header::HeaderMap, Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::{torrent_stuff::*, error::DelugeError};

pub struct Account {
    pub username: String,
    pub password: String,
    pub authlevel: String,
    pub authlevel_int: usize,
}
pub struct DelugeInterface {
    client: Client,
    ip: Ipv4Addr,
    port: Option<String>,
}

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
    pub fn new(ip: Ipv4Addr, port: Option<String>) -> Result<Self, DelugeError> {
        log::debug!("Creating Headers");
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
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

    async fn request(&self, request: Request) -> Result<TorrentResponse, DelugeError> {
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

    pub async fn login(&self, password: String) -> Result<(), DelugeError> {
        log::info!("Logging In");
        let request = Request::new("auth.login", Some(vec![json!(password)]));
        log::debug!("Login info: {:?}", &request);
        let res_json = self.request(request).await?;
        log::debug!("Login response: {:?}", &res_json);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), DelugeError> {
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
    ) -> Result<Option<String>, DelugeError> {
        log::debug!("Adding Torrent File");
        let params = vec![
            json!(filename),
            json!(filedump),
            options.to_json(),
            json!(save_state),
        ];
        let res_json = self
            .request(Request::new("core.add_torrent_file_async", Some(params)))
            .await?;
        let out = match res_json.result {
            Value::Null => None,
            Value::String(out) => Some(out),
            _ => {
                return Err(DelugeError::Json);
            }
        };
        Ok(out)
    }

    pub async fn prefetch_magnet_metadata(
        &self,
        magnet_uri: &str,
        timeout: Option<Duration>,
    ) -> Result<(String, String), DelugeError> {
        log::debug!("Prefetching Magnet Metadata");
        let res_json = self
            .request(Request::new(
                "core.prefetch_magnet_metadata",
                timeout.map(|x| vec![json!(x.as_secs())]),
            ))
            .await?;
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
            .ok_or(DelugeError::Json)?; // Again, there's probably a better way to do this
        Ok(out)
    }

    pub async fn add_torrent_file(
        &self,
        filename: &OsString,
        filedump: &str,
        options: &TorrentOptions,
    ) -> Result<Option<String>, DelugeError> {
        todo!()
    }

    pub async fn add_torrent_files(
        &self,
        torrent_files: &[(OsString, String, TorrentOptions)],
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn add_torrent_url(
        &self,
        url: Url,
        options: &TorrentOptions,
        headers: Option<&HeaderMap>,
    ) -> Result<Option<String>, DelugeError> {
        todo!()
    }

    pub async fn add_torrent_magnet(
        &self,
        uri: &str,
        options: &TorrentOptions,
    ) -> Result<String, DelugeError> {
        todo!()
    }

    pub async fn remove_torrent(
        &self,
        torrent_id: &str,
        remove_data: bool,
    ) -> Result<bool, DelugeError> {
        log::debug!("Removing Torrent");
        let res_json = self
            .request(Request::new(
                "core.remove_torrent",
                Some(vec![json!(torrent_id), json!(remove_data)]),
            ))
            .await?;
        let out = res_json.result.as_bool().ok_or(DelugeError::Json)?;
        Ok(out)
    }

    pub async fn remove_torrents(
        &self,
        torrent_ids: &[String],
        remove_data: bool,
    ) -> Result<(), DelugeError> {
        // Actually has rich error
        todo!()
    }

    pub async fn get_sessions_status(
        &self,
        keys: &[String],
    ) -> Result<HashMap<String, TorrentStatus>, DelugeError> {
        todo!()
    }

    pub async fn force_reannounce(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), DelugeError> {
        log::debug!("Pausing Torrent");
        let res_json = self
            .request(Request::new(
                "core.pause_torrent",
                Some(vec![json!(torrent_id)]),
            ))
            .await?;
        Ok(())
    }

    pub async fn pause_torrents(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        log::debug!("Pausing Torrents");
        let res_json = self
            .request(Request::new(
                "core.pause_torrents",
                Some(torrent_ids.iter().map(|x| json!(x)).collect()),
            ))
            .await?;
        Ok(())
    }

    pub async fn connect_peer(
        &self,
        torrent_id: &str,
        ip: Ipv4Addr,
        port: u16,
    ) -> Result<(), DelugeError> {
        log::debug!("Connecting to Peer");
        let res_json = self
            .request(Request::new(
                "core.connect_peer",
                Some(vec![json!(ip.to_string()), json!(port)]),
            ))
            .await?;
        Ok(())
    }

    pub async fn move_storage(
        &self,
        torrent_ids: &[String],
        dest: &OsString,
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn pause_session(&self) -> Result<(), DelugeError> {
        log::debug!("Pausing Session");
        let res_json = self
            .request(Request::new("core.pause_session", None))
            .await?;
        Ok(())
    }
    pub async fn resume_session(&self) -> Result<(), DelugeError> {
        log::debug!("Resuming Session");
        let res_json = self
            .request(Request::new("core.resume_session", None))
            .await?;
        Ok(())
    }
    pub async fn is_session_paused(&self) -> Result<bool, DelugeError> {
        log::debug!("Checking if session is paused");
        let res_json = self
            .request(Request::new("core.is_session_paused", None))
            .await?;
        let out = res_json.result.as_bool().ok_or(DelugeError::Json)?;
        Ok(out)
    }
    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), DelugeError> {
        log::debug!("Resume Torrent");
        let res_json = self
            .request(Request::new(
                "core.resume_torrent",
                Some(vec![json!(torrent_id)]),
            ))
            .await?;
        Ok(())
    }
    pub async fn resume_torrents(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        log::debug!("Resuming Torrents");
        let res_json = self
            .request(Request::new(
                "core.resume_torrents",
                Some(torrent_ids.iter().map(|x| json!(x)).collect()),
            ))
            .await?;
        Ok(())
    }
    pub async fn get_torrent_status(
        &self,
        torrent_id: &str,
        keys: &[String],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, DelugeError> {
        log::debug!("Getting torrent status");
        let mut params = vec![json!(torrent_id)];
        params.extend(keys.iter().map(|x| json!(x)));
        params.push(json!(diff));
        let res_json = self
            .request(Request::new("core.get_torrent_status", Some(params)))
            .await?;
        let out = res_json.result.as_object().ok_or(DelugeError::Json)?.to_owned();
        Ok(out)
    }
    pub async fn get_torrents_status(
        &self,
        filter_dict: &Map<String, Value>,
        keys: &[String],
        diff: Option<bool>,
    ) -> Result<Map<String, Value>, DelugeError> {
        todo!()
    }

    pub async fn get_filter_tree(
        &self,
        show_zero_hits: Option<bool>,
        hide_cat: Option<&[String]>,
    ) -> Result<Map<String, (Value, usize)>, DelugeError> {
        todo!()
    }

    pub async fn get_session_state(&self) -> Result<Vec<String>, DelugeError> {
        log::info!("Getting session state");

        let res_json = self
            .request(Request::new("core.get_session_state", None))
            .await?;

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

    pub async fn get_config(&self) -> Result<Map<String, Value>, DelugeError> {
        todo!()
    }

    pub async fn get_config_value(&self, key: &str) -> Result<Value, DelugeError> {
        todo!()
    }

    pub async fn get_config_values(
        &self,
        keys: &[String],
    ) -> Result<Map<String, Value>, DelugeError> {
        todo!()
    }

    pub async fn set_config(&self, config: &Map<String, Value>) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn get_listen_port(&self) -> Result<u16, DelugeError> {
        todo!()
    }

    pub async fn get_proxy(&self) -> Result<Map<String, Value>, DelugeError> {
        todo!()
    }

    pub async fn get_available_plugins(&self) -> Result<Vec<String>, DelugeError> {
        todo!()
    }

    pub async fn get_enabled_plugins(&self) -> Result<Vec<String>, DelugeError> {
        todo!()
    }

    pub async fn enable_plugin(&self, plugin: &str) -> Result<bool, DelugeError> {
        todo!()
    }

    pub async fn disable_plugin(&self, plugin: &str) -> Result<bool, DelugeError> {
        todo!()
    }

    pub async fn force_recheck(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn set_torrent_options(
        &self,
        torrent_ids: &[String],
        options: &TorrentOptions,
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn set_torrent_trackers(
        &self,
        torrent_id: &str,
        trackers: &TorrentTracker,
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn get_magnet_uri(&self, torrent_id: &str) -> Result<String, DelugeError> {
        log::debug!("Getting Magnet Uri of {}", torrent_id);
        let res_json = self
            .request(Request::new(
                "core.get_magnet_uri",
                Some(vec![json!(torrent_id)]),
            ))
            .await?;
        let magnet_uri = res_json
            .result
            .as_str()
            .map(|x| x.to_owned())
            .ok_or(DelugeError::Json)?;
        log::debug!("{}", &magnet_uri);
        Ok(magnet_uri)
    }

    pub async fn get_path_size(&self) -> Result<Option<usize>, DelugeError> {
        log::debug!("Getting Path Size");
        let res_json = self
            .request(Request::new("core.get_path_size", None))
            .await?;
        let path_size = res_json.result.as_i64().ok_or(DelugeError::Json)?;
        Ok(match path_size {
            -1 => None,
            _ => Some(path_size.try_into()?),
        })
    }
    // pub async fn create_torrent(
    //     &self,
    //     path: OsString,
    //     tracker: TorrentTracker,
    //     piece_length: usize,
    //     comment: Option<String>,
    //     target: Option<OsString>,
    //     webseeds: Option<Vec<String>>,
    //     private: Option<bool>,
    //     created_by: Option<String>,
    //     trackers: Option<Vec<String>>,
    //     add_to_session: bool,
    // ) -> Result<(), DelugeError> {
    //     todo!()
    // }

    pub async fn upload_plugin(
        &self,
        filename: OsString,
        filedump: &[u8],
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn rescan_plugins(&self) -> Result<(), DelugeError> {
        log::debug!("Rescanning Plugins");
        let res_json = self
            .request(Request::new("core.rescan_plugins", None))
            .await?;
        Ok(())
    }

    pub async fn rename_files(
        &self,
        torrent_id: &str,
        filenames: &[(usize, OsString)],
    ) -> Result<(), DelugeError> {
        todo!()
    }
    pub async fn rename_folder(
        &self,
        torrent_id: &str,
        folder: OsString,
        new_folder: OsString,
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn queue_top(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn queue_up(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn queue_down(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn queue_bottom(&self, torrent_ids: &[String]) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn glob(&self, path: OsString) -> Result<Vec<String>, DelugeError> {
        todo!()
    }

    pub async fn test_listen_port(&self) -> Result<bool, DelugeError> {
        log::debug!("Test Listen Port");
        let res_json = self
            .request(Request::new("core.test_listen_port", None))
            .await?;
        let out = res_json.result.as_bool().ok_or(DelugeError::Json)?;
        Ok(out)
    }

    pub async fn get_free_space(&self, path: Option<OsString>) -> Result<usize, DelugeError> {
        todo!()
    }

    pub async fn external_ip(&self) -> Result<IpAddr, DelugeError> {
        todo!()
    }

    pub async fn get_libtorrent_version(&self) -> Result<String, DelugeError> {
        todo!()
    }

    pub async fn get_completion_paths(
        &self,
        args: &Map<String, Value>,
    ) -> Result<Map<String, Value>, DelugeError> {
        todo!()
    }
    pub async fn get_known_accounts(&self) -> Result<Vec<Account>, DelugeError> {
        todo!()
    }
    pub async fn get_auth_levels_mappings(
        &self,
    ) -> Result<(Map<String, usize>, Map<usize, String>), DelugeError> {
        todo!()
    }
    pub async fn create_account(&self, account: Account) -> Result<bool, DelugeError> {
        todo!()
    }
    pub async fn update_account(&self, account: Account) -> Result<bool, DelugeError> {
        todo!()
    }
    pub async fn remove_account(&self, username: &str) -> Result<bool, DelugeError> {
        todo!()
    }

    // ! End of Core

    // ! Start of Daemon
    // pub async fn shutdown(&self){todo!()}
    // pub async fn get_method_list(&self){todo!()}
    // pub async fn get_version(&self){todo!()}
    // pub async fn authorized_call(&self, rpc)
    // ! End of Daemon
    // ! Start of Web
    // pub async fn change_password(&self, old_password, new_password){todo!()}
    // pub async fn check_session(&self, session_id=None){todo!()}
    // pub async fn delete_session(&self){todo!()}
    // pub async fn login(&self, password){todo!()}
    // pub async fn connect(&self, host_id){todo!()}
    // pub async fn connected(&self){todo!()}
    // pub async fn disconnect(&self){todo!()}
    // pub async fn update_ui(&self, keys, filter_dict){todo!()}
    // pub async fn get_torrent_files(&self, torrent_id){todo!()}
    // pub async fn download_torrent_from_url(&self, url, cookie=None){todo!()}
    // pub async fn get_torrent_info(&self, filename){todo!()}
    // pub async fn get_magnet_info(&self, uri){todo!()}
}
