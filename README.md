## Introduction

This project is an addition to the log collection project written in the Rust programming language! The project is a Telegram bot created to demonstrate error logs caused by various reasons in a Docker container.

## Using

**Before using it, you should understand that the project is based on another project: [logs-rust](https://github.com/MaksUnweb/logs-rust.git) and simply works with the database.**

## Required dependencies:

1) Rust and Cargo >= 1.95.
2) Docker.
3) A working project [logs-rust](https://github.com/MaksUnweb/logs-rust.git).


### Instructions:

1) Before using it, you need to create tables and indexes in the database based on the 0002_schema.sql file:
```sql
CREATE TABLE IF NOT EXISTS telegram_admins(
  id BIGSERIAL PRIMARY KEY,
  uid BIGINT NOT NULL,
  login VARCHAR NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_id_telegram_admins ON telegram_admins(id);
CREATE INDEX IF NOT EXISTS idx_uid_telegram_admins ON telegram_admins(uid);
```
2) After this, you need to create a bot using any convenient method (personally, I recommend using BotFatner)
3) After creating the bot, you need to add the following variables to your system's environment variables:

```
DB_URL=postgres://your_login:your_password@your_host/your_db?connect_timeout=your_timeout
TELOXIDE_TOKEN=your_token
```

Here you have login, password, host ig time should be yours, and the default database for the main project is logs_db (if you not changed).
TELOXIDE_TOKEN - it's your token for bot.


