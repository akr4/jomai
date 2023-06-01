# jomai-core

## Setup

```bash
export DATABASE_URL=sqlite:jomai.db
sqlx db create
sqlx migrate run
cargo sqlx prepare
```
