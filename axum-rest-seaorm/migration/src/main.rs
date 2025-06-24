use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

//---------------------------------
// docker pull postgres:latest
//
//---------------------------------
// docker run -d \
//   -p 5432:5432 \
//   -e POSTGRES_DB=axum_seaorm_db \
//   -e POSTGRES_USER=axum \
//   -e POSTGRES_PASSWORD=1234 \
//   --name pg-axum \
//   postgres
//---------------------------------

//---------------------------------
// > sea-orm-cli migrate up
// Running `cargo run --manifest-path ./migration/Cargo.toml -- up -u postgress://axum:1234@localhost/axum_seaorm_db
// warning: `migration` (lib) generated 1 warning (run `cargo fix --lib -p migration` to apply 1 suggestion)
//    Compiling migration v0.1.0 (/Volumes/SSD_01/zorba/fun/rust-lang-study/axum-examples/axum-book-jpub/mytrial/rust-axum-book_jpub/axum-rest-seaorm/migration)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
//      Running `migration/target/debug/migration up -u 'postgres://axum:1234@localhost/axum_seaorm_db'`
// Applying all pending migrations
// Applying migration 'm20250624_091523_create_table'
// Migration 'm20250624_091523_create_table' has been applied


//---------------------------------
// > sea-orm-cli generate entity -o src/entities
// Connecting to Postgres ...
// Discovering schema ...
// ... discovered.
// Generating category.rs
//     > Column `name`: String, not_null
// Generating product.rs
//     > Column `id`: i32, auto_increment, not_null
//     > Column `title`: String, not_null
//     > Column `price`: i32, not_null
//     > Column `category`: String, not_null
// Generating users.rs
//     > Column `id`: i32, auto_increment, not_null
//     > Column `username`: String, not_null
//     > Column `password`: String, not_null
// Writing src/entities/category.rs
// Writing src/entities/product.rs
// Writing src/entities/users.rs
// Writing src/entities/mod.rs
// Writing src/entities/prelude.rs
// ... Done.

