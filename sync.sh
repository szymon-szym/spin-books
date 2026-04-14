rsync -avz --relative spin.toml migration.sql target/wasm32-wasip1/release/books_spin.wasm szymon@szymonpi3.local:~/projects/spin-books
rsync -avz --relative spin-books.service szymon@szymonpi3.local:/etc/systemd/system/
