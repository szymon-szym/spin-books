use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Serialize, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[http_component]
fn handle_books_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("{:?}", &req.uri());

    let mut router = spin_sdk::http::Router::new();

    router.add(
        "/book/:id",
        spin_sdk::http::Method::Get,
        api::get_book_by_id,
    );

    println!("Handling request to {:?}", req.header("spin-full-url"));

    Ok(router.handle(req))
}

mod api {
    use spin_sdk::{
        http::Params,
        sqlite3::{Connection, Value},
    };

    use anyhow::anyhow;

    use super::*;

    pub(crate) fn get_book_by_id(
        _req: Request,
        params: Params,
    ) -> anyhow::Result<impl IntoResponse> {
        println!("{:?}", &params);

        let id_param = params.get("id").ok_or(anyhow!("Missing id parameter"))?;

        let id = id_param.parse::<i64>()?;

        let query_params = [Value::Integer(id)];

        let connection = Connection::open_default()?;

        let rowset = connection.execute(
            "SELECT title, author, year FROM books WHERE id = ?",
            &query_params,
        )?;

        let books: Vec<Book> = rowset
            .rows()
            .map(|row| Book {
                title: row.get::<&str>("title").unwrap().to_string(),
                author: row.get::<&str>("author").unwrap().to_string(),
                year: row.get::<u32>("year").unwrap(),
            })
            .collect();

        let book = books.first().ok_or(anyhow!("Book not found"))?;

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&book)?)
            .build())
    }
}
