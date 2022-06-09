#![allow(unused_variables)] // TODO remove this when no more todo
use std::{
    collections::HashMap,
    mem,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use camino::{Utf8Path, Utf8PathBuf};
use reqwest::{header::HeaderMap, Client, ClientBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    error::{DelugeApiError, DelugeError},
    torrent_stuff::*,
};
pub struct Account {
    pub username: String,
    pub password: String,
    pub authlevel: String,
    pub authlevel_int: usize,
}

#[derive(Debug)]
pub struct DelugeInterface {
    client: Client,
    ip: Ipv4Addr,
    port: Option<String>,
}

#[derive(Debug)]
struct RequestBuilder<'a> {
    interface: &'a DelugeInterface,
    method: String,
    params: Vec<Value>,
}

impl<'a> RequestBuilder<'a> {
    fn add_params<T>(&mut self, params: &[T]) -> &mut Self
    where
        T: Serialize,
    {
        self.params.extend(params.iter().map(|v| json!(v)));
        self
    }
    fn add_param<T>(&mut self, param: &T) -> &mut Self
    where
        T: Serialize,
    {
        self.params.push(json!(param));
        self
    }

    async fn send<V>(&mut self) -> Result<TorrentResponse<V>, DelugeApiError>
    where
        V: DeserializeOwned,
    {
        let request = Request {
            method: mem::take(&mut self.method),
            params: mem::take(&mut self.params),
            id: 1,
        };
        log::debug!("Sending Request");
        log::debug!("{:?}", &request);
        let out = self
            .interface
            .client
            .post(&self.interface.url())
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        log::debug!("Got Response");
        Ok(out)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    params: Vec<Value>,
    id: usize,
}

impl DelugeInterface {
    pub fn new(ip: Ipv4Addr, port: Option<String>) -> Result<Self, DelugeApiError> {
        log::debug!("Creating Headers");
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        log::debug!(
            "Creating Client {{ ip: {}, port: {:?}}}",
            &ip,
            port.as_ref()
        );
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

    fn request(&self, method: &str) -> RequestBuilder {
        RequestBuilder {
            interface: self,
            method: method.to_string(),
            params: Vec::new(),
        }
    }

    pub async fn login(&self, password: String) -> Result<(), DelugeApiError> {
        log::debug!("Logging In");
        self.request("auth.login")
            .add_param(&password)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn disconnect(&self) -> Result<(), DelugeApiError> {
        log::debug!("Disconnecting");
        self.request("web.disconnect")
            .send()
            .await?
            .into_empty_result()
    }

    // ! Start of Core

    pub async fn add_torrent_file_async(
        &self,
        filename: &Utf8Path,
        filedump: &str,
        options: &TorrentOptions,
        save_state: Option<bool>,
    ) -> Result<Option<String>, DelugeApiError> {
        log::debug!("Adding Torrent File");
        let mut builder = self.request("core.add_torrent_file_async");
        builder
            .add_param(&filename)
            .add_param(&filedump)
            .add_param(options);
        if let Some(save_state) = save_state {
            builder.add_param(&save_state);
        }
        builder.send().await?.into_result()
    }

    pub async fn prefetch_magnet_metadata(
        &self,
        magnet_uri: &str,
        timeout: Option<Duration>,
    ) -> Result<(String, String), DelugeApiError> {
        log::debug!("Prefetching Magnet Metadata");
        let mut builder = self.request("core.prefetch_magnet_metadata");
        if let Some(timeout) = timeout {
            builder.add_param(&timeout.as_secs());
        }
        builder.send().await?.into_result()
    }

    pub async fn add_torrent_file(
        &self,
        filename: &Utf8Path,
        filedump: &str,
        options: &TorrentOptions,
    ) -> Result<Option<String>, DelugeApiError> {
        log::debug!("Adding Torrent File");
        self.request("core.add_torrent_file_async")
            .add_param(&filename)
            .add_param(&filedump)
            .add_param(options)
            .send()
            .await?
            .into_result()
    }

    pub async fn add_torrent_files(
        &self,
        torrent_files: &[(Utf8PathBuf, String, TorrentOptions)],
    ) -> Result<(), DelugeApiError> {
        self.request("core.add_torrent_files")
            .add_param(&torrent_files)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn add_torrent_url(
        &self,
        url: Url,
        options: &TorrentOptions,
        headers: Option<&HeaderMap>,
    ) -> Result<Option<String>, DelugeApiError> {
        log::debug!("Adding torrent from url");
        let mut builder = self.request("core.add_torrent_url");
        builder.add_param(&url.as_str()).add_param(options);
        if let Some(headers) = headers {
            builder.add_param(
                &headers
                    .into_iter()
                    .map(|(k, v)| match v.to_str() {
                        Ok(v) => Ok((k.to_string(), v.to_string())),
                        Err(_) => Err(DelugeApiError::IncorrectHeaderFormat),
                    })
                    .collect::<Result<HashMap<String, String>, DelugeApiError>>()?,
            );
        }
        builder.send().await?.into_result()
    }

    pub async fn add_torrent_magnet(
        &self,
        uri: &str,
        options: &TorrentOptions,
    ) -> Result<String, DelugeApiError> {
        log::debug!("Adding Torrent from magnet");
        let out = self
            .request("core.add_torrent_magnet")
            .add_param(&uri)
            .add_param(&options)
            .send()
            .await?
            .into_result();
        let out = if let Err(DelugeApiError::Deluge(DelugeError::DuplicateTorrent(id))) = out {
            id
        } else {
            out?
        };
        Ok(out)
    }

    pub async fn remove_torrent(
        &self,
        torrent_id: &str,
        remove_data: bool,
    ) -> Result<bool, DelugeApiError> {
        log::debug!("Removing Torrent");
        self.request("core.remove_torrent")
            .add_param(&torrent_id)
            .add_param(&remove_data)
            .send()
            .await?
            .into_result()
    }

    pub async fn remove_torrents(
        &self,
        torrent_ids: &[String],
        remove_data: bool,
    ) -> Result<(), DelugeApiError> {
        // Actually has rich error
        todo!("Needs rich error process on into_result")
    }

    pub async fn get_sessions_status(
        &self,
        keys: &[String],
    ) -> Result<HashMap<String, TorrentStatus>, DelugeApiError> {
        self.request("core.get_sessions_status")
            .add_param(&keys)
            .send()
            .await?
            .into_result()
    }

    pub async fn force_reannounce(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.get_sessions_status")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), DelugeApiError> {
        log::debug!("Pausing Torrent");
        self.request("core.pause_torrent")
            .add_param(&torrent_id)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn pause_torrents(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        log::debug!("Pausing Torrents");
        self.request("core.pause_torrents")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn connect_peer(
        &self,
        torrent_id: &str,
        ip: Ipv4Addr,
        port: u16,
    ) -> Result<(), DelugeApiError> {
        log::debug!("Connecting to Peer");
        self.request("core.connect_peer")
            .add_param(&ip.to_string())
            .add_param(&port)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn move_storage(
        &self,
        torrent_ids: &[String],
        dest: &Utf8Path,
    ) -> Result<(), DelugeApiError> {
        self.request("core.move_storage")
            .add_param(&torrent_ids)
            .add_param(&dest)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn pause_session(&self) -> Result<(), DelugeApiError> {
        log::debug!("Pausing Session");
        self.request("core.pause_session")
            .send()
            .await?
            .into_empty_result()
    }
    pub async fn resume_session(&self) -> Result<(), DelugeApiError> {
        log::debug!("Resuming Session");
        self.request("core.resume_session")
            .send()
            .await?
            .into_empty_result()
    }
    pub async fn is_session_paused(&self) -> Result<bool, DelugeApiError> {
        log::debug!("Checking if session is paused");
        self.request("core.is_session_paused")
            .send()
            .await?
            .into_result()
    }
    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), DelugeApiError> {
        log::debug!("Resume Torrent");
        self.request("core.resume_torrent")
            .add_param(&torrent_id)
            .send()
            .await?
            .into_empty_result()
    }
    pub async fn resume_torrents(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        log::debug!("Resuming Torrents");
        self.request("core.resume_torrents")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }
    pub async fn get_torrent_status(
        &self,
        torrent_id: &str,
        keys: &[String],
        diff: Option<bool>,
    ) -> Result<HashMap<String, Value>, DelugeApiError> {
        log::debug!("Getting torrent status");
        let mut builder = self.request("core.get_torrent_status");
        builder.add_param(&torrent_id).add_param(&keys);
        if let Some(diff) = diff {
            builder.add_param(&diff);
        }
        builder.send().await?.into_result()
    }
    pub async fn get_torrents_status(
        &self,
        filter_dict: &HashMap<String, Value>,
        keys: &[String],
        diff: Option<bool>,
    ) -> Result<HashMap<String, Value>, DelugeApiError> {
        let mut builder = self.request("core.get_torrents_status");
        builder.add_param(filter_dict).add_param(&keys);
        if let Some(diff) = diff {
            builder.add_param(&diff);
        }
        builder.send().await?.into_result()
    }

    pub async fn get_filter_tree(
        &self,
        show_zero_hits: Option<bool>,
        hide_cat: Option<&[String]>,
    ) -> Result<HashMap<String, (Value, usize)>, DelugeApiError> {
        let mut builder = self.request("core.get_filter_tree");
        if let Some(hide_cat) = hide_cat {
            if let Some(szh) = show_zero_hits {
                builder.add_param(&szh);
            } else {
                builder.add_param(&true);
            }
            builder.add_param(&hide_cat);
        }
        builder.send().await?.into_result()
    }

    pub async fn get_session_state(&self) -> Result<Vec<String>, DelugeApiError> {
        log::debug!("Getting session state");

        self.request("core.get_session_state")
            .send()
            .await?
            .into_result()
    }

    pub async fn get_config(&self) -> Result<HashMap<String, Value>, DelugeApiError> {
        log::debug!("Getting config");
        self.request("core.get_config").send().await?.into_result()
    }

    pub async fn get_config_value(&self, key: &str) -> Result<Value, DelugeApiError> {
        log::debug!("Getting config");

        self.request("core.get_config_value")
            .add_param(&key)
            .send()
            .await?
            .into_result()
    }

    pub async fn get_config_values(
        &self,
        keys: &[String],
    ) -> Result<HashMap<String, Value>, DelugeApiError> {
        self.request("core.get)config_values")
            .add_param(&keys)
            .send()
            .await?
            .into_result()
    }

    pub async fn set_config(&self, config: &HashMap<String, Value>) -> Result<(), DelugeApiError> {
        todo!()
    }

    pub async fn get_listen_port(&self) -> Result<u16, DelugeApiError> {
        self.request("core.get_listen_port")
            .send()
            .await?
            .into_result()
    }

    pub async fn get_proxy(&self) -> Result<HashMap<String, Value>, DelugeApiError> {
        self.request("core.get_proxy").send().await?.into_result()
    }

    pub async fn get_available_plugins(&self) -> Result<Vec<String>, DelugeApiError> {
        self.request("core.get_available_plugins")
            .send()
            .await?
            .into_result()
    }

    pub async fn get_enabled_plugins(&self) -> Result<Vec<String>, DelugeApiError> {
        self.request("core.get_enabled_plugins")
            .send()
            .await?
            .into_result()
    }

    pub async fn enable_plugin(&self, plugin: &str) -> Result<bool, DelugeApiError> {
        self.request("core.enable_plugin")
            .add_param(&plugin)
            .send()
            .await?
            .into_result()
    }

    pub async fn disable_plugin(&self, plugin: &str) -> Result<bool, DelugeApiError> {
        self.request("core.disable_plugin")
            .add_param(&plugin)
            .send()
            .await?
            .into_result()
    }

    pub async fn force_recheck(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.force_recheck")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn set_torrent_options(
        &self,
        torrent_ids: &[String],
        options: &TorrentOptions,
    ) -> Result<(), DelugeApiError> {
        self.request("core.set_torrent_options")
            .add_param(&torrent_ids)
            .add_param(options)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn set_torrent_trackers(
        &self,
        torrent_id: &str,
        trackers: &TorrentTracker,
    ) -> Result<(), DelugeApiError> {
        self.request("core.set_trackers")
            .add_param(&torrent_id)
            .add_param(trackers)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn get_magnet_uri(&self, torrent_id: &str) -> Result<String, DelugeApiError> {
        log::debug!("Getting Magnet Uri of {}", torrent_id);
        self.request("core.get_magnet_uri")
            .add_param(&torrent_id)
            .send()
            .await?
            .into_result()
    }

    pub async fn get_path_size(&self) -> Result<Option<usize>, DelugeApiError> {
        log::debug!("Getting Path Size");
        let path_size = self
            .request("core.get_path_size")
            .send()
            .await?
            .into_result()?;
        Ok(match path_size {
            -1 => None,
            _ => Some(path_size.try_into()?),
        })
    }
    pub async fn create_torrent(
        &self,
        torrent: Torrent,
        add_to_session: bool,
    ) -> Result<(), DelugeError> {
        todo!()
    }

    pub async fn upload_plugin(
        &self,
        filename: Utf8PathBuf,
        filedump: &[u8],
    ) -> Result<(), DelugeApiError> {
        self.request("core.upload_plugin")
            .add_param(&filename)
            .add_param(&filedump)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn rescan_plugins(&self) -> Result<(), DelugeApiError> {
        log::debug!("Rescanning Plugins");
        self.request("core.rescan_plugins")
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn rename_files(
        &self,
        torrent_id: &str,
        filenames: &[(usize, Utf8PathBuf)],
    ) -> Result<(), DelugeApiError> {
        self.request("core.rename_files")
            .add_param(&torrent_id)
            .add_param(&filenames)
            .send()
            .await?
            .into_empty_result()
    }
    pub async fn rename_folder(
        &self,
        torrent_id: &str,
        folder: Utf8PathBuf,
        new_folder: Utf8PathBuf,
    ) -> Result<(), DelugeApiError> {
        self.request("core.rename_folder")
            .add_param(&torrent_id)
            .add_param(&folder)
            .add_param(&new_folder)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn queue_top(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.queue_top")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn queue_up(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.queue_up")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn queue_down(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.queue_down")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn queue_bottom(&self, torrent_ids: &[String]) -> Result<(), DelugeApiError> {
        self.request("core.queue_bottom")
            .add_param(&torrent_ids)
            .send()
            .await?
            .into_empty_result()
    }

    pub async fn glob(&self, path: Utf8PathBuf) -> Result<Vec<String>, DelugeApiError> {
        self.request("core.glob")
            .add_param(&path)
            .send()
            .await?
            .into_result()
    }

    pub async fn test_listen_port(&self) -> Result<bool, DelugeApiError> {
        log::debug!("Test Listen Port");
        self.request("core.test_listen_port")
            .send()
            .await?
            .into_result()
    }

    pub async fn get_free_space(&self, path: Option<Utf8PathBuf>) -> Result<usize, DelugeApiError> {
        todo!()
    }

    pub async fn external_ip(&self) -> Result<IpAddr, DelugeApiError> {
        self.request("core.external_ip").send().await?.into_result()
    }

    pub async fn get_libtorrent_version(&self) -> Result<String, DelugeApiError> {
        self.request("core.get_libtorrent_version")
            .send()
            .await?
            .into_result()
    }

    pub async fn get_completion_paths(
        &self,
        args: &HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, DelugeApiError> {
        todo!()
    }
    pub async fn get_known_accounts(&self) -> Result<Vec<Account>, DelugeApiError> {
        todo!()
    }
    pub async fn get_auth_levels_mappings(
        &self,
    ) -> Result<(HashMap<String, usize>, HashMap<usize, String>), DelugeApiError> {
        todo!()
    }
    pub async fn create_account(&self, account: Account) -> Result<bool, DelugeApiError> {
        todo!()
    }
    pub async fn update_account(&self, account: Account) -> Result<bool, DelugeApiError> {
        todo!()
    }
    pub async fn remove_account(&self, username: &str) -> Result<bool, DelugeApiError> {
        self.request("core.remove_account")
            .add_param(&username)
            .send()
            .await?
            .into_result()
    }

    // ! End of Core

    // ! Start of Daemon
    // pub async fn shutdown(&self){todo!()}
    // pub async fn get_method_list(&self){todo!()}
    pub async fn get_version(&self) -> Result<String, DelugeApiError> {
        log::debug!("Getting Version");
        self.request("daemon.get_version")
            .send()
            .await?
            .into_result()
    }
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
