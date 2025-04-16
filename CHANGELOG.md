# Changelog

## 2.0.0

## Breaking changes

This release changes to SurrealDB v2. It is required to migrate existing databases.
Check out the SurrealDB [changelog](https://surrealdb.com/releases#v2-0-0) and [migration guide](https://surrealdb.com/docs/surrealdb/installation/upgrading/migrating-data-to-2x) on how to migrate.

## Added

- Boardgame availability option
- Embedded SurrealDB instance

## Changed

- [**BREAKING**] Migrated to SurrealDB v2
- Changed default `webp_quality` option to `1.00`
- Changed default `db_adr` option to `rocksdb:./data/bgcat.db`
- Improved CSS
- Pagination now shows less pages
- Container: Minified CSS and JS

## Removed

- Container: Removed SurrealDB in favor of the embedded instance
- Removed option `db_start_local`

## 1.0.0

Initial release
