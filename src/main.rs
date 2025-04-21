mod supported_id;

use poem::{listener::TcpListener, Route, Server, http::StatusCode};
use poem_openapi::{param::Query, OpenApi, OpenApiService};
use num_enum::TryFromPrimitive;

use crate::supported_id::{SupportedChainId, SUPPORTED_CHAIN_IDS};


#[derive(Debug, TryFromPrimitive)]
#[repr(u64)]
enum SupportedChainIdEnum {
    Ethereum = 1,
    Base = 8543
}

// these are just stubs for an actual route's logic, for the sake of example
fn inner_handler_enum(_chain_id: SupportedChainIdEnum) -> poem::Result<()> { Ok(()) }
fn inner_handler(_chain_id: SupportedChainId) -> poem::Result<()> { Ok(()) }

struct Api;

#[allow(unused)]
#[OpenApi]
impl Api {
    // the first example in the blog post
    async fn chain_supported_1st_enum_example(
        &self,
        Query(chain_id): Query<u64>,
    ) -> poem::Result<()> {
        let id = SupportedChainIdEnum::try_from(chain_id)
            .map_err(|_| poem::Error::from_string(
                format!("unsupported chain ID: {}", chain_id),
                StatusCode::BAD_REQUEST,
            ))?;

        inner_handler_enum(id)
    }

    // the better way of doing it with an enum
    async fn chain_supported_enum(
        &self,
        Query(chain_id): Query<SupportedChainIdEnum>,
    ) -> poem::Result<()> {
        inner_handler_enum(chain_id)
    }

    // this is the noisier way to handle it with runtime sets
    async fn chain_supported_manual_check(
        &self,
        Query(chain_id): Query<u64>,
    ) -> poem::Result<()> {
        match SUPPORTED_CHAIN_IDS.contains(&chain_id) {
            true => {
                inner_handler(SupportedChainId(chain_id))
            }
            false => Err(poem::Error::from_string(
                format!("unsupported chain ID: {}", chain_id),
                StatusCode::BAD_REQUEST,
            )),
        }
    }

    // this is the nice way to do it with runtime sets
    async fn chain_supported(
        &self,
        Query(chain_id): Query<SupportedChainId>,
    ) -> poem::Result<()> {
        inner_handler(chain_id)
    }
}

#[tokio::main]
async fn main() {
    let api_service =OpenApiService::new(Api, "Set Parsing Example","1.0")
        .server("http://localhost:7870");
    let app = Route::new().nest("/", api_service);

    let _ = Server::new(TcpListener::bind("127.0.0.1:7870"))
        .run(app)
        .await;
}