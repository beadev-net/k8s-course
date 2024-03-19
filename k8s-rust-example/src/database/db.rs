use mysql::prelude::*;
use mysql::*;
use serde_json;

struct Request {
    path: String,
    data: Option<String>,
}

fn connect() -> Result<mysql::PooledConn> {
    let url = "mysql://root:root@localhost:3306/db_name";

    let pool = Pool::new(url)?;

    return Ok(pool.get_conn()?);
}

pub fn create_table() -> Result<()> {
    let mut conn = connect().unwrap();

    conn.query_drop(
        r"CREATE TABLE requests (
            path varchar(100) not null unique,
            data json
        )",
    )?;

    Ok(())
}

pub fn insert(path: &str, data: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = connect().unwrap();

    let requests = Request {
        path: path.to_string(),
        data: Some(data.to_string()),
    };

    conn.exec_drop(
        "INSERT INTO requests (path, data) VALUES (?, ?)",
        (requests.path, requests.data),
    )?;

    Ok(())
}

pub fn get_mock_path(path: &str) -> std::result::Result<serde_json::Value, mysql::Error> {
    let mut conn = connect().unwrap();

    let requests = Request {
        path: path.to_string(),
        data: None,
    };

    let stmt = format!(
        "SELECT data FROM requests WHERE path LIKE \"{path}\" LIMIT 1",
        path = requests.path
    );

    let query_result: Option<String> = conn.query_first(&stmt)?;

    match query_result {
        Some(query_result) => {
            let result_to_string = query_result.to_string();
            let result_to_json = serde_json::from_str(&result_to_string).unwrap();

            Ok(result_to_json)
        }
        None => Ok(serde_json::from_str("{}").unwrap()),
    }
}
