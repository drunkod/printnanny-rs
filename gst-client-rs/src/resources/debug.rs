//! Define [`struct@Debug`] which encapsulate methods
//! of [`Debug API`][1] for [`GStreamer`][2]
//!
//! Details about [`GStreamer Debugging`]
//!
//! [GStreamer]: https://gstreamer.freedesktop.org/
//! [1]: https://developer.ridgerun.com/wiki/index.php/GStreamer_Daemon_-_C_API#Debug
//! [2]: https://developer.ridgerun.com/wiki/index.php/GStreamer_Debugging
use crate::{gstd_types, Error, GstClient};

/// Performs requests to `debug/` endpoint
#[derive(Debug, Clone)]
pub struct Debug {
    client: GstClient,
}

impl Debug {
    pub(crate) fn new(client: &GstClient) -> Self {
        Self {
            client: client.clone(),
        }
    }
    /// Performs `PUT debug/enable?name=enable`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn enable(&self) -> Result<gstd_types::Response, Error> {
        let url = self.client.base_url.join("debug/enable?name=true").map_err(Error::IncorrectApiUrl)?;
        let resp = self.client.put(url).await?;
        self.client.process_resp(resp).await
    }

    /// Performs `PUT debug/enable?name=false`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn disable(&self) -> Result<gstd_types::Response, Error> {
        let url = self.client.base_url.join("debug/enable?name=false").map_err(Error::IncorrectApiUrl)?;
        let resp = self.client.put(url).await?;
        self.client.process_resp(resp).await
    }

    /// Performs `PUT debug/reset?name={value}`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn reset(&self, value: bool) -> Result<gstd_types::Response, Error> {
        let val = if value { "true" } else { "false" };
        let url = self.client.base_url.join(&format!("debug/reset?name={val}")).map_err(Error::IncorrectApiUrl)?;
        let resp = self.client.put(url).await?;
        self.client.process_resp(resp).await
    }
    /// Performs `PUT debug/threshold?name={value}`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn threshold(&self, value: &str) -> Result<gstd_types::Response, Error> {
        let url = self.client.base_url.join(&format!("debug/threshold?name={value}")).map_err(Error::IncorrectApiUrl)?;
        let resp = self
            .client
            .put(url)
            .await?;
        self.client.process_resp(resp).await
    }
    /// Performs `PUT debug/color?name=true`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn enable_color(&self) -> Result<gstd_types::Response, Error> {
        let url = self.client.base_url.join("debug/color?name=true").map_err(Error::IncorrectApiUrl)?;
        let resp = self.client.put(url).await?;
        self.client.process_resp(resp).await
    }
    /// Performs `PUT debug/color?name=false`
    /// API request, returning the parsed [`gstd_types::Response`]
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails.
    /// See [`Error`] for details.
    pub async fn disable_color(&self) -> Result<gstd_types::Response, Error> {
        let url = self.client.base_url.join("debug/color?name=false").map_err(Error::IncorrectApiUrl)?;
        let resp = self.client.put(url).await?;
        self.client.process_resp(resp).await
    }
}
