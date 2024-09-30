use super::state::Entry;
use gloo_net::http::Request;
use web_sys::FormData;
use web_sys::UrlSearchParams;
use yew::Callback;

pub fn emit_todos(update_todo_cb: Callback<Vec<Entry>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let url = "http://127.0.0.1:3000/todo/v1/list";
        let fetched_todos: Vec<Entry> = Request::get(url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        update_todo_cb.emit(fetched_todos);
    });
}
pub fn update_todo(todo: Entry) {
    wasm_bindgen_futures::spawn_local(async move {
        let url = "http://127.0.0.1:3000/todo/v1/update";
        let url_params = UrlSearchParams::new().unwrap();
        url_params.append("id", &todo.id);
        url_params.append("description", &todo.description);
        url_params.append("completed", &format!("{}", todo.completed));
        let _update_response = Request::post(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(url_params)
            .send()
            .await
            .unwrap();
    });
}
pub fn add_todo(todo: Entry) {
    wasm_bindgen_futures::spawn_local(async move {
        let url = "http://127.0.0.1:3000/todo/v1/new";
        let form_data = FormData::new().unwrap();
        form_data
            .append_with_str("description", &todo.description)
            .unwrap();
        form_data
            .append_with_str("completed", &format!("{}", todo.completed))
            .unwrap();
        let url_params = UrlSearchParams::new().unwrap();
        url_params.append("description", &todo.description);
        url_params.append("completed", &format!("{}", todo.completed));
        let _added_entry = Request::post(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(url_params)
            .send()
            .await
            .unwrap();
    });
}
pub fn delete_todo(id: String) {
    wasm_bindgen_futures::spawn_local(async move {
        let url = "http://127.0.0.1:3000/todo/v1/delete/:id";
        let url_params = UrlSearchParams::new().unwrap();
        url_params.append("id", id.as_str());
        let _deleted_entry_response = Request::post(url).body(url_params).send().await.unwrap();
    });
}
