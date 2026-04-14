CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    year INTEGER NOT NULL
);

INSERT INTO books (id, title, author, year) VALUES (1, 'Wieża Jaskółki', 'Andrzej Sapkowski', 2014)
ON CONFLICT(id) DO UPDATE SET title=excluded.title, author=excluded.author, year=excluded.year;
