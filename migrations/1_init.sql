CREATE TABLE pastebin (
    key TEXT PRIMARY KEY,
    content TEXT,
    language TEXT,
    created_time INTEGER,
    expire_time INTEGER,
    length INTEGER,
)