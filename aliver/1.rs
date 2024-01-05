#![feature(prelude_import)]
#![feature(async_closure)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod index {
    pub async fn index() -> ::aerr::Result<impl Into<::aerr::Msg>> {
        Ok(alive::status().await?)
    }
}
use index::index;
mod ping {
    use std::sync::atomic::Ordering::Relaxed;
    use aerr::err;
    use axum::http::StatusCode;
    use tokio::time::{sleep, Duration};
    pub async fn ping() -> ::aerr::Result<impl Into<::aerr::Msg>> {
        let pre = alive::cron::TS.load(Relaxed);
        let now = sts::sec();
        let diff = if now > pre { now - pre } else { 0 };
        if diff > 300 {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event aliver/src/ping.rs:13",
                            "aliver::ping",
                            ::tracing::Level::ERROR,
                            ::core::option::Option::Some("aliver/src/ping.rs"),
                            ::core::option::Option::Some(13u32),
                            ::core::option::Option::Some("aliver::ping"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::ERROR
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::ERROR
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                        if match ::tracing::Level::ERROR {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &value_set,
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::core::iter::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::core::option::Option::Some(
                                            &format_args!("alive cron expire") as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                    if match ::tracing::Level::ERROR {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::ERROR {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &{
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("alive cron expire") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            },
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                }
            };
            tokio::spawn(async {
                sleep(Duration::from_secs(3)).await;
                std::process::exit(1);
            });
            err(StatusCode::FAILED_DEPENDENCY, "alive cron expire".to_owned())?
        }
        return Ok(diff.to_string());
    }
}
use std::{net::SocketAddr, time::Duration};
use aok::Result;
use axum::{
    body::Body, error_handling::HandleErrorLayer, extract::Request,
    http::{self, HeaderValue, StatusCode},
    middleware, middleware::Next, response::IntoResponse, routing::get, BoxError, Router,
};
use ping::ping;
use tower::ServiceBuilder;
use tower_http::compression::{
    predicate::{NotForContentType, Predicate, SizeAbove},
    CompressionLayer,
};
#[allow(non_snake_case)]
pub fn PORT() -> u16 {
    ::genv::get_or_default("PORT", 5123)
}
#[allow(clippy::upper_case_acronyms)]
struct __StaticInitGeneratorFor_HEALTHCHECK;
impl ::static_init::Generator<String> for __StaticInitGeneratorFor_HEALTHCHECK {
    #[inline]
    fn generate(&self) -> String {
        ::genv::get("HEALTHCHECK")
    }
}
impl ::static_init::GeneratorTolerance for __StaticInitGeneratorFor_HEALTHCHECK {
    const INIT_FAILURE: bool = true;
    const FINAL_REGISTRATION_FAILURE: bool = false;
}
static HEALTHCHECK: ::static_init::lazy::LesserLazy<
    String,
    __StaticInitGeneratorFor_HEALTHCHECK,
> = {
    let _ = ();
    unsafe {
        ::static_init::lazy::LesserLazy::<
            String,
            __StaticInitGeneratorFor_HEALTHCHECK,
        >::from_generator(__StaticInitGeneratorFor_HEALTHCHECK)
    }
};
pub static TEXT_JSON: &str = "text/json";
async fn header(req: Request<Body>, next: Next) -> impl IntoResponse {
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert(http::header::CONTENT_TYPE, HeaderValue::from_static(TEXT_JSON));
    res
}
const TIMEOUT: u64 = 600;
fn main() -> Result<()> {
    let body = async {
        tokio::spawn(async {
            let healthcheck = HEALTHCHECK.as_str();
            alive::cron::run(async move || {
                    {
                        if let Err(err) = ireq::get(healthcheck).await {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event aliver/src/main.rs:47",
                                            "aliver",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("aliver/src/main.rs"),
                                            ::core::option::Option::Some(47u32),
                                            ::core::option::Option::Some("aliver"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &value_set,
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("{0}", err) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                    if match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &{
                                                                #[allow(unused_imports)]
                                                                use ::tracing::field::{debug, display, Value};
                                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                                __CALLSITE
                                                                    .metadata()
                                                                    .fields()
                                                                    .value_set(
                                                                        &[
                                                                            (
                                                                                &::core::iter::Iterator::next(&mut iter)
                                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                                ::core::option::Option::Some(
                                                                                    &format_args!("{0}", err) as &dyn Value,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    )
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                }
                            };
                        }
                    };
                })
                .await;
        });
        let middleware = ServiceBuilder::new()
            .layer(
                HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok((StatusCode::REQUEST_TIMEOUT, "timeout"))
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Internal Error: {0}", error),
                                );
                                res
                            },
                        ))
                    }
                }),
            )
            .timeout(Duration::from_secs(TIMEOUT).into())
            .layer(ServiceBuilder::new());
        loginit::init();
        let predicate = SizeAbove::new(256)
            .and(NotForContentType::GRPC)
            .and(NotForContentType::IMAGES);
        let app = Router::new()
            .route("/", get(aerr::FnAny(index)))
            .route("/ping", get(aerr::FnAny(ping)))
            .layer(middleware)
            .layer(CompressionLayer::new().compress_when(predicate))
            .layer(ServiceBuilder::new().layer(middleware::from_fn(header)));
        let addr = SocketAddr::from(([0, 0, 0, 0], PORT()));
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event aliver/src/main.rs:79",
                        "aliver",
                        ::tracing::Level::INFO,
                        ::core::option::Option::Some("aliver/src/main.rs"),
                        ::core::option::Option::Some(79u32),
                        ::core::option::Option::Some("aliver"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("http://{0}", addr) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("http://{0}", addr) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
