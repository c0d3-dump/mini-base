-- Add up migration script here
CREATE TABLE IF NOT EXISTS
    roles (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(255) UNIQUE NOT NULL,
        is_default TINYINT(1) NOT NULL DEFAULT 0,
        can_read TINYINT(1) NOT NULL DEFAULT 0,
        can_write TINYINT(1) NOT NULL DEFAULT 0,
        can_delete TINYINT(1) NOT NULL DEFAULT 0
    );

CREATE TABLE IF NOT EXISTS
    users (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        email VARCHAR(100) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        role_id INTEGER,
        FOREIGN KEY (role_id) REFERENCES roles (id)
    );

CREATE TABLE IF NOT EXISTS
    storage (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        file_name VARCHAR(255) NOT NULL,
        unique_name VARCHAR(36) UNIQUE NOT NULL,
        uploaded_by INTEGER NOT NULL,
        FOREIGN KEY (uploaded_by) REFERENCES users (id)
    );

CREATE TABLE IF NOT EXISTS
    queries (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(255) UNIQUE NOT NULL,
        exec_type VARCHAR(50) NOT NULL DEFAULT 'get' CHECK (exec_type IN ('get', 'post', 'delete', 'put')),
        query TEXT DEFAULT ''
    );

CREATE TABLE IF NOT EXISTS
    role_access (
        role_id INTEGER NOT NULL,
        query_id INTEGER NOT NULL,
        FOREIGN KEY (role_id) REFERENCES roles (id),
        FOREIGN KEY (query_id) REFERENCES queries (id),
        UNIQUE (role_id, query_id)
    );