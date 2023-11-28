use shuttle_runtime::{CustomError, Error};
use std::net::SocketAddr;

pub struct AxumService(pub axum::Router);

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for AxumService {
    /// Takes the router that is returned by the user in their [shuttle_runtime::main] function
    /// and binds to an address passed in by shuttle.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0.into_make_service())
            .await
            .map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<axum::Router> for AxumService {
    fn from(router: axum::Router) -> Self {
        Self(router)
    }
}

pub type ShuttleAxum = Result<AxumService, Error>;
