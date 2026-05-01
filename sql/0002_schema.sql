CREATE TABLE IF NOT EXISTS telegram_admins(
  id BIGSERIAL PRIMARY KEY,
  uid BIGINT NOT NULL,
  login VARCHAR NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_id_telegram_admins ON telegram_admins(id);
CREATE INDEX IF NOT EXISTS idx_uid_telegram_admins ON telegram_admins(uid);
