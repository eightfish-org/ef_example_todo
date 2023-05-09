use eightfish::{EightFishModel, Info, Module, Request, Response, Result, Router, Status};
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, DbValue, Decode, ParameterValue};

const REDIS_URL_ENV: &str = "REDIS_URL";
const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Clone, Serialize, Deserialize, EightFishModel, Default)]
pub struct Todo {
    id: String,
    description: String,
    completed: bool,
}

pub struct TodoModule;

impl TodoModule {
    fn list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();
        println!("in handler todo list: params: {:?}", params);

        // construct a sql statement
        let sql_statement = Todo::build_get_list_sql(None, None);
        println!("in handler todo: list: {:?}", sql_statement);
        let sql_params: Vec<ParameterValue> = vec![];
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params).unwrap();
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
            action: "list".to_string(),
            target: "".to_string(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();
        println!("in handler todo: params: {:?}", params);

        let todo_id = params.get("id").unwrap();

        // construct a sql statement
        let (sql_statement, sql_params) = Todo::build_get_one_sql_and_params(todo_id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params).unwrap();
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
            action: "get_one".to_string(),
            target: todo_id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn newitem(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

        let description = params.get("description").unwrap();

        let id = req.ext().get("random_str").unwrap();

        // construct a struct
        let todo = Todo {
            id: id.clone(),
            description: description.clone(),
            completed: false,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = todo.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);
        println!("in handler new: _execute_results: {:?}", _execute_results);

        let results: Vec<Todo> = vec![todo];
        // results.push(todo);

        let info = Info {
            model_name: Todo::model_name(),
            action: "new".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

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
        let (sql_statement, sql_params) = todo.build_update_sql_and_params();
        println!("{}, {}", sql_statement, sql_params.len());
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);

        let results: Vec<Todo> = vec![todo];
        // results.push(todo);

        let info = Info {
            model_name: Todo::model_name(),
            action: "update".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

        let id = params.get("id").unwrap();

        // construct a sql statement
        let (sql_statement, sql_params) = Todo::build_delete_sql_and_params(id.as_str());
        println!("in handler delete: statement: {:?}", sql_statement);
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);
        // TODO check the pg result

        let results: Vec<Todo> = vec![];

        let info = Info {
            model_name: Todo::model_name(),
            action: "delete".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }
}

impl Module for TodoModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/todo", Self::list);
        router.get("/todo/:id", Self::get_one);
        router.post("/todo/new", Self::newitem);
        router.post("/todo/update", Self::update);
        router.post("/todo/delete/:id", Self::delete);

        Ok(())
    }
}
