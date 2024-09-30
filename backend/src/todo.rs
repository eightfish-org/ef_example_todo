use eightfish_derive::EightFishModel;
use eightfish_sdk::{
    EightFishModel, HandlerCRUD, Info, Module, Request, Response, Result, Router, Status,
};
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, DbValue, Decode, ParameterValue};
use sql_builder::SqlBuilder;

const REDIS_URL_ENV: &str = "REDIS_URL_ENV";
const DB_URL_ENV: &str = "DB_URL_ENV";

#[derive(Debug, Clone, Serialize, Deserialize, EightFishModel, Default)]
pub struct Todo {
    id: String,
    description: String,
    completed: bool,
}

pub struct TodoModule;

impl TodoModule {
    fn list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        println!("in handler todo list: params: {:?}", params);

        // construct a sql statement
        let sql = SqlBuilder::select_from(&Todo::model_name())
            .fields(&Todo::fields())
            .sql()?;

        println!("in handler todo: list: {:?}", sql);
        let sql_params: Vec<ParameterValue> = vec![];
        let rowset = pg_conn.query(&sql, &sql_params).unwrap();
        println!("in handler todo: rowset: {:?}", rowset);
        // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
        // rust struct vec, later we may find a gerneral type converter way
        let mut results: Vec<Todo> = vec![];
        for row in rowset.rows {
            let todo = Todo::from_row(row);
            results.push(todo);
        }
        println!("in handler list: results: {:?}", results);

        let info = Info {
            model_name: Todo::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        println!("in handler todo: params: {:?}", params);

        let todo_id = params.get("id").expect("missing id");

        // construct a sql statement
        let (sql, sql_params) = Todo::build_get_by_id(todo_id);
        let rowset = pg_conn.query(&sql, &sql_params).unwrap();
        println!("in handler todo: rowset: {:?}", rowset);

        // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
        // rust struct vec, later we may find a gerneral type converter way
        let mut results: Vec<Todo> = vec![];
        for row in rowset.rows {
            let todo = Todo::from_row(row);
            results.push(todo);
        }
        println!("in handler todl: results: {:?}", results);

        let info = Info {
            model_name: Todo::model_name(),
            action: HandlerCRUD::GetOne,
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn newitem(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let description = params.get("description").expect("missing description");
        let id = req.ext().get("random_str").expect("missing random_str");

        // construct a struct
        let todo = Todo {
            id: id.clone(),
            description: description.clone(),
            completed: false,
        };

        // construct a sql statement and param
        let (sql, sql_params) = todo.build_insert();
        let _execute_results = pg_conn.execute(&sql, &sql_params);
        println!("in handler new: _execute_results: {:?}", _execute_results);

        let results: Vec<Todo> = vec![todo];
        // results.push(todo);
        let info = Info {
            model_name: Todo::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").unwrap();
        let description = params.get("description").unwrap();
        let completed = params.get("completed").unwrap();
        println!(
            "in handler update: id: {}, description: {}, completed: {}, status: {}",
            id,
            description,
            completed,
            completed == "true"
        );
        // construct a struct
        let todo = Todo {
            id: id.clone(),
            description: description.clone(),
            completed: completed == "true",
        };

        // construct a sql statement and params
        let (sql, sql_params) = todo.build_update();
        println!("{}, {}", sql, sql_params.len());
        let _execute_results = pg_conn.execute(&sql, &sql_params);

        let results: Vec<Todo> = vec![todo];
        let info = Info {
            model_name: Todo::model_name(),
            action: HandlerCRUD::Update,
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").unwrap();

        // construct a sql statement
        let (sql, sql_params) = Todo::build_delete(id.as_str());
        println!("in handler delete: statement: {:?}", sql);
        let _execute_results = pg_conn.execute(&sql, &sql_params);
        // TODO check the pg result

        let results: Vec<Todo> = vec![];

        let info = Info {
            model_name: Todo::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }
}

impl Module for TodoModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/todo/v1/list", Self::list);
        router.get("/todo/v1/:id", Self::get_one);
        router.post("/todo/v1/new", Self::newitem);
        router.post("/todo/v1/update", Self::update);
        router.post("/todo/v1/delete/:id", Self::delete);

        Ok(())
    }
}
