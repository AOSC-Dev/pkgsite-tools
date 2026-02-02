#[cfg(feature = "nyquest")]
use nyquest::{AsyncClient, ClientBuilder, Request, r#async::Response};
#[cfg(feature = "reqwest")]
use reqwest::{Client, Response, StatusCode, redirect::Policy};
#[cfg(feature = "nyquest")]
use std::borrow::Cow;

pub mod error;
pub mod models;

use error::{PResult, PackagesSiteError};
pub use models::*;
use models::{
    depends::Depends, files::Files, index::Index, info::Info, rdepends::RDepends, search::Search,
    updates::Updates,
};

pub(crate) const PACKAGES_SITE_URL: &str = "https://packages.aosc.io";

pub struct PackagesSiteClient {
    pub url: String,
    #[cfg(feature = "reqwest")]
    client: Client,
    #[cfg(feature = "nyquest")]
    client: AsyncClient,
}

impl PackagesSiteClient {
    #[cfg(feature = "reqwest")]
    pub fn new(url: String) -> PResult<Self> {
        Ok(Self {
            url,
            client: Client::builder().redirect(Policy::none()).build()?,
        })
    }

    #[cfg(feature = "reqwest")]
    pub fn default_url() -> PResult<Self> {
        Self::new(PACKAGES_SITE_URL.to_owned())
    }

    #[cfg(feature = "reqwest")]
    pub fn from_env() -> PResult<Self> {
        Self::new(std::env::var("PACKAGE_SITE_URL")?)
    }

    #[cfg(feature = "reqwest")]
    async fn get_data<S>(&self, url: S) -> PResult<Response>
    where
        S: AsRef<str>,
    {
        Ok(self.client.get(url.as_ref()).send().await?)
    }

    #[cfg(feature = "nyquest")]
    pub async fn new(url: String) -> PResult<Self> {
        Ok(Self {
            url,
            client: ClientBuilder::default()
                .no_redirects()
                .build_async()
                .await?,
        })
    }

    #[cfg(feature = "nyquest")]
    pub async fn default_url() -> PResult<Self> {
        Ok(Self::new(PACKAGES_SITE_URL.to_owned()).await?)
    }

    #[cfg(feature = "nyquest")]
    pub async fn from_env() -> PResult<Self> {
        Ok(Self::new(std::env::var("PACKAGE_SITE_URL")?).await?)
    }

    #[cfg(feature = "nyquest")]
    async fn get_data<S>(&self, url: S) -> PResult<Response>
    where
        S: Into<Cow<'static, str>>,
    {
        Ok(self.client.request(Request::get(url)).await?)
    }

    pub async fn depends<'a>(&self, packages: &'a [String]) -> PResult<Vec<(&'a str, Depends)>> {
        let mut res = Vec::new();
        for package in packages {
            if let Ok(dep) = self
                .get_data(format!("{}/packages/{}?type=json", &self.url, package))
                .await?
                .json::<Depends>()
                .await
            {
                res.push((package.as_str(), dep));
            }
        }
        Ok(res)
    }

    pub async fn rdepends<'a>(&self, packages: &'a [String]) -> PResult<Vec<(&'a str, RDepends)>> {
        let mut res = Vec::new();
        for package in packages.iter() {
            if let Ok(revdep) = self
                .get_data(format!("{}/revdep/{}?type=json", &self.url, package))
                .await?
                .json::<RDepends>()
                .await
            {
                res.push((package.as_str(), revdep));
            }
        }
        Ok(res)
    }

    pub async fn info<I, S>(&self, packages: I) -> PResult<Vec<Info>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut res = Vec::new();
        for package in packages {
            if let Ok(info) = self
                .get_data(format!(
                    "{}/packages/{}?type=json",
                    &self.url,
                    package.as_ref()
                ))
                .await?
                .json::<Info>()
                .await
                .inspect_err(|e| eprintln!("Failed to parse response data to Info: {e}"))
            {
                res.push(info);
            }
        }
        Ok(res)
    }

    pub async fn search(&self, pattern: &str, noredir: bool) -> PResult<SearchOrInfo> {
        let response = self
            .get_data(format!(
                "{}/search?q={}&type=json&noredir={}",
                &self.url, pattern, noredir
            ))
            .await?;

        #[cfg(feature = "reqwest")]
        let status = response.status().as_u16();
        #[cfg(feature = "nyquest")]
        let status = response.status().code();
        match status {
            200 => Ok(SearchOrInfo::Search(response.json::<Search>().await?)),
            303 => Ok(SearchOrInfo::Info(
                self.info(&[pattern]).await?.pop().unwrap(),
            )),
            #[cfg(feature = "reqwest")]
            code => Err(PackagesSiteError::UnexpectedStatus(
                StatusCode::from_u16(code).unwrap(),
            )),
            #[cfg(feature = "nyquest")]
            code => Err(PackagesSiteError::UnexpectedStatus(code)),
        }
    }

    pub async fn index(&self) -> PResult<Index> {
        Ok(self
            .get_data(format!("{}/?type=json", &self.url))
            .await?
            .json::<Index>()
            .await?)
    }

    pub async fn updates(&self) -> PResult<Updates> {
        Ok(self
            .get_data(format!("{}/updates?type=json", &self.url))
            .await?
            .json::<Updates>()
            .await?)
    }

    pub async fn files(
        &self,
        arch: &str,
        branch: &str,
        package: &str,
        version: &str,
    ) -> PResult<Files> {
        let response = self
            .get_data(format!(
                "{}/files/{}/{}/{}/{}?type=json",
                &self.url,
                arch.to_string(),
                branch,
                package,
                version
            ))
            .await?;
        #[cfg(feature = "reqwest")]
        let status = response.status().as_u16();
        #[cfg(feature = "nyquest")]
        let status = response.status().code();
        if status != 200 {
            #[cfg(feature = "reqwest")]
            return Err(PackagesSiteError::UnexpectedStatus(
                StatusCode::from_u16(status).unwrap(),
            ));
            #[cfg(feature = "nyquest")]
            return Err(PackagesSiteError::UnexpectedStatus(status));
        } else {
            Ok(response.json::<Files>().await?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PackagesSiteClient;
    use crate::error::PResult;

    #[tokio::test]
    async fn test_fetch() -> PResult<()> {
        #[cfg(feature = "reqwest")]
        let client = PackagesSiteClient::default_url()?;
        #[cfg(feature = "nyquest")]
        nyquest_preset::register();
        #[cfg(feature = "nyquest")]
        let client = PackagesSiteClient::default_url().await?;

        dbg!(client.index().await?);

        Ok(())
    }
}
