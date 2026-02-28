pub mod aggragte;
pub mod event;

use eventually::version::Version;
use sea_query::{Expr, Iden, OnConflict, PostgresQueryBuilder, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{AnyPool, Row};
use uuid::Uuid;

// --- Tabellen-Definitionen für sea-query ---

#[derive(Iden)]
pub enum Events {
    Table,
    GlobalPosition,
    EventId,
    EventType,
    AggregateType,
    AggregateId,
    AggregateVersion,
    Data,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
pub enum Snapshots {
    Table,
    AggregateType,
    AggregateId,
    AggregateVersion,
    State,
    CreatedAt,
    UpdatedAt,
}

// --- Helper: Datenbank-Agnostische Konflikterkennung ---

pub fn is_conflict_error(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        let code = db_err.code().unwrap_or_default();
        // 23505 = Postgres Unique Violation
        // 2067 / 19 = SQLite Unique Constraint Violation
        return code == "23505" || code == "2067" || code == "19";
    }
    false
}

// Ein einfaches Enum, um zur Laufzeit den richtigen Builder zu wählen
#[derive(Clone, Copy)]
pub enum DbBackend {
    Postgres,
    Sqlite,
}
