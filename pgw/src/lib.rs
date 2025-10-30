#![feature(fundamental)]
#![feature(macro_metavar_expr)]

use tokio_postgres::types::ToSql;
pub use tokio_postgres::{self, types};

mod conn;
use conn::conn;
mod into_statement;
mod tls;
pub use into_statement::IntoStatement;
mod pg;
pub use pg::{Pg, Sql};

pub type Args<'a> = &'a [&'a (dyn ToSql + Sync)];

pub const NONE: Args = &[];

#[cfg(feature = "global")]
mod global;

#[cfg(feature = "global")]
pub use global::{PG, Sqlify};
