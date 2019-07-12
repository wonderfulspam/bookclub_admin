CREATE TABLE books (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    content BLOB NOT NULL,
    mime_type TEXT NOT NULL
)