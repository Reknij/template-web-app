--- Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL,
    alias TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL DEFAULT '',
    user_type INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%SZ', 'now', 'utc')),
    updated_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%MZ', 'now', 'utc')),
    is_deleted BOOLEAN NOT NULL DEFAULT false
);
CREATE INDEX IF NOT EXISTS idx_users_id ON users (id);
CREATE INDEX IF NOT EXISTS idx_users_username_password ON users (username, password);
CREATE INDEX IF NOT EXISTS idx_users_user_type ON users (user_type);
CREATE INDEX IF NOT EXISTS idx_users_alias ON users (alias);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_is_deleted ON users (is_deleted);
---