use reqwest::{Client, StatusCode, redirect::Policy};

pub mod error;
pub mod models;

use error::{PResult, PackagesSiteError};
pub use models::*;
use models::{depends::Depends, index::Index, info::Info, rdepends::RDepends, search::Search};

pub(crate) const PACKAGES_SITE_URL: &str = "https://packages.aosc.io";

pub struct PackagesSiteClient {
    pub url: String,
    client: reqwest::Client,
}

impl Default for PackagesSiteClient {
    fn default() -> Self {
        Self {
            url: PACKAGES_SITE_URL.to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

impl PackagesSiteClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Self {
        Self::new(
            std::env::var("PACKAGE_SITE_URL")
                .expect("PACKAGE_SITE_URL environment variable is not set"),
        )
    }

    pub async fn depends(&self, packages: &[String]) -> PResult<Vec<(String, Depends)>> {
        let mut res = Vec::new();
        for package in packages.iter() {
            if let Ok(dep) = self
                .client
                .get(format!("{}/packages/{}?type=json", &self.url, package))
                .send()
                .await?
                .json::<Depends>()
                .await
            {
                res.push((package.clone(), dep));
            }
        }
        Ok(res)
    }

    pub async fn rdepends(&self, packages: &[String]) -> PResult<Vec<(String, RDepends)>> {
        let mut res = Vec::new();
        for package in packages.iter() {
            if let Ok(revdep) = self
                .client
                .get(format!("{}/revdep/{}?type=json", &self.url, package))
                .send()
                .await?
                .json::<RDepends>()
                .await
            {
                res.push((package.clone(), revdep));
            }
        }
        Ok(res)
    }

    pub async fn info(&self, packages: &[String]) -> PResult<Vec<Info>> {
        let mut res = Vec::new();
        for package in packages.iter() {
            if let Ok(info) = self
                .client
                .get(format!("{}/packages/{}?type=json", &self.url, package))
                .send()
                .await?
                .json::<Info>()
                .await
            {
                res.push(info);
            }
        }
        Ok(res)
    }

    pub async fn search(&self, pattern: &str, noredir: bool) -> PResult<SearchExactMatch> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        let response = client
            .get(format!(
                "{}/search?q={}&type=json{}",
                &self.url,
                pattern,
                if noredir { "&noredir=true" } else { "" }
            ))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(SearchExactMatch::Search(response.json::<Search>().await?)),
            StatusCode::SEE_OTHER => Ok(SearchExactMatch::Info(Box::new(
                self.info(&[pattern.to_string()]).await?.pop().unwrap(),
            ))),
            code => Err(PackagesSiteError::UnexpectedStatus(code)),
        }
    }

    pub async fn index(&self) -> PResult<Index> {
        Ok(self
            .client
            .get(format!("{}/?type=json", &self.url))
            .send()
            .await?
            .json::<Index>()
            .await?)
    }
}
