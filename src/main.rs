#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate statsd;

use std::sync::Mutex;

use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use todolist::Todo;
use todolist::TodoList;

use statsd::Client;

mod todolist;

type ID = usize;
type Todos = Mutex<TodoList>;

struct InternalState {
    pub todolist: Todos,
    pub statsd: Client,
}

#[get("/")]
fn index(state: State<InternalState>) -> Option<JsonValue> {
    let statsd = &state.statsd;
    statsd.incr("todos.index.success.counter");
    statsd.time("todos.index.duration", || {
        Some(json!({ "data": [] }))
    })
}

#[post("/", format = "json", data = "<todo>")]
fn create(todo: Json<Todo>, state: State<InternalState>) -> Option<JsonValue> {
    let statsd = &state.statsd;
    let mut todolist = state.todolist.lock().unwrap();

    statsd.time("todos.create.duration", || {
        match todolist.add(&todo.name, todo.completed) {
            Ok(todo) => {
                statsd.incr("todos.create.success.counter");
                Some(json!({ "data": todo }))
            },
            Err(_error) => {
                statsd.incr("todos.create.error.counter");
                None
            }
        }
    })
}

#[put("/<id>", format = "json", data = "<todo>")]
fn update(id: ID, todo: Json<Todo>, state: State<InternalState>) -> Option<JsonValue> {
    let statsd = &state.statsd;
    let mut todolist = state.todolist.lock().unwrap();

    statsd.time("todos.update.duration", || {
        match todolist.update(id, &todo.name, todo.completed) {
            Ok(todo) => {
                statsd.incr("todos.update.success.counter");
                Some(json!({ "data": todo }))
            }
            Err(_error) => {
                statsd.incr("todos.update.error.counter");
                None
            }
        }
    })
}

#[get("/<id>")]
fn show(id: ID, state: State<InternalState>) -> Option<JsonValue> {
    let statsd = &state.statsd;
    let todolist = state.todolist.lock().unwrap();

    statsd.time("todos.show.duration", || {
        match todolist.find(id) {
            Ok(todo) => {
                statsd.incr("todos.show.success.counter");
                Some(json!({ "data": todo }))
            }
            Err(_error) => {
                statsd.incr("todos.show.error.counter");
                None
            }
        }
    })
}

#[delete("/<id>")]
fn delete(id: ID, state: State<InternalState>) -> Status {
    let statsd = &state.statsd;
    let mut todolist = state.todolist.lock().unwrap();

    statsd.time("todos.delete.duration", || {
        match todolist.delete(id) {
            Ok(()) => {
                statsd.incr("todos.delete.success.counter");
                Status::NoContent
            },
            Err(_err) => {
                statsd.incr("todos.delete.error.counter");
                Status::NotFound
            }
        }
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "status": "error",
        "reason": "The request was well-formed but was unable to be followed due to semantic errors"
    })
}

fn rocket() -> rocket::Rocket {
    let statsd = Client::new("127.0.0.1:8125", "rust-todos").unwrap();
    let todolist = Mutex::new(TodoList::new());
    let state = InternalState {
        todolist: todolist,
        statsd: statsd,
    };

    rocket::ignite()
        .mount("/todos", routes![index, create, show, update, delete])
        .register(catchers![not_found, unprocessable_entity])
        .manage(state)
}

fn main() {
    rocket().launch();
}
