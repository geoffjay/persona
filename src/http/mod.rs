use futures::future::BoxFuture;
use futures::FutureExt;
use gpui_http_client::http::{HeaderValue, Request, Response};
use gpui_http_client::{AsyncBody, HttpClient, RedirectPolicy, Url};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};

/// A simple HTTP client using reqwest for GPUI image loading
pub struct ReqwestHttpClient {
    client: reqwest::Client,
    handle: Handle,
    // Store runtime in an Arc to keep it alive and allow Clone
    _runtime: Arc<Runtime>,
    user_agent: HeaderValue,
}

impl ReqwestHttpClient {
    pub fn new() -> Arc<Self> {
        // Create a Tokio runtime for HTTP requests
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");

        let handle = runtime.handle().clone();
        let client = runtime.block_on(async {
            reqwest::Client::builder()
                .build()
                .expect("Failed to create reqwest client")
        });

        Arc::new(Self {
            client,
            handle,
            _runtime: Arc::new(runtime),
            user_agent: HeaderValue::from_static("persona-ui/0.1"),
        })
    }
}

impl HttpClient for ReqwestHttpClient {
    fn type_name(&self) -> &'static str {
        "ReqwestHttpClient"
    }

    fn user_agent(&self) -> Option<&HeaderValue> {
        Some(&self.user_agent)
    }

    fn proxy(&self) -> Option<&Url> {
        None
    }

    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, anyhow::Result<Response<AsyncBody>>> {
        let client = self.client.clone();
        let handle = self.handle.clone();

        async move {
            let (parts, _body) = req.into_parts();
            let uri = parts.uri.to_string();

            // Check redirect policy from extensions
            let follow_redirects = parts
                .extensions
                .get::<RedirectPolicy>()
                .map(|p| !matches!(p, RedirectPolicy::NoFollow))
                .unwrap_or(true);

            // Execute the request in the Tokio runtime
            let result = handle
                .spawn(async move {
                    // Build the reqwest request
                    let mut builder = client.request(parts.method, &uri);

                    // Add headers
                    for (name, value) in parts.headers.iter() {
                        if let Ok(v) = value.to_str() {
                            builder = builder.header(name.as_str(), v);
                        }
                    }

                    // Handle redirect policy
                    if !follow_redirects {
                        let no_redirect_client = reqwest::Client::builder()
                            .redirect(reqwest::redirect::Policy::none())
                            .build()?;
                        builder =
                            no_redirect_client.request(builder.build()?.method().clone(), &uri);
                    }

                    // Send the request
                    let response = builder.send().await?;

                    // Convert to http response
                    let status = response.status();
                    let headers = response.headers().clone();
                    let bytes = response.bytes().await?;

                    let mut http_response = Response::builder().status(status.as_u16());

                    for (name, value) in headers.iter() {
                        if let Ok(v) = value.to_str() {
                            http_response = http_response.header(name.as_str(), v);
                        }
                    }

                    let response = http_response.body(AsyncBody::from(bytes.to_vec()))?;
                    Ok::<_, anyhow::Error>(response)
                })
                .await??;

            Ok(result)
        }
        .boxed()
    }
}
