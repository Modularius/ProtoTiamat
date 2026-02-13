use std::str::FromStr;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUrl {
    url: Url
}

impl FromStr for PublicUrl {
    type Err = <Url as FromStr>::Err;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self{ url: Url::from_str(string)? })
    }
}

impl PublicUrl {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn header_form(&self) -> String {
        let mut url = self.url.to_string();
        if let Some('/') = url.chars().last() {
            url.pop();
        }
        url
    }

    pub fn router_base_form(&self) -> String {
        cfg_if! {
            if #[cfg(feature = "hydrate")] {
                if self.url.path() == "/" {
                    self.url.path().to_string()
                } else {
                    Default::default()
                }
            } else {
                Default::default()
            }
        }
    }

    pub fn hydrate_form(&self) -> &'static str {
        let mut url = self.url.to_string();
        if let Some('/') = url.chars().last() {
            url.pop();
        }
        url.leak()
    }
}