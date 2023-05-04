#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

// Define a struct to represent a User
#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    id: usize,
    name: String,
    email: String,
}

// Define a global vector to store users
lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

// Define a route to get all users
#[get("/users")]
fn get_all_users() -> Json<Vec<User>> {
    let users = USERS.lock().unwrap();
    Json(users.clone())
}

// Define a route to get a user by ID
#[get("/users/<id>")]
fn get_user(id: usize) -> Option<Json<User>> {
    let users = USERS.lock().unwrap();
    let user = users.iter().find(|&user| user.id == id)?;
    Some(Json(user.clone()))
}

// Define a route to create a new user
#[post("/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    let mut users = USERS.lock().unwrap();
    let id = users.len() + 1;
    let new_user = User {
        id,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    users.push(new_user.clone());
    Json(new_user)
}

// Define a route to update an existing user
#[put("/users/<id>", format = "json", data = "<user>")]
fn update_user(id: usize, user: Json<User>) -> Option<Json<User>> {
    let mut users = USERS.lock().unwrap();
    let index = users.iter().position(|u| u.id == id)?;
    let updated_user = User {
        id,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    users[index] = updated_user.clone();
    Some(Json(updated_user))
}

// Define a route to delete a user
#[delete("/users/<id>")]
fn delete_user(id: usize) -> Option<Json<User>> {
    let mut users = USERS.lock().unwrap();
    let index = users.iter().position(|u| u.id == id)?;
    let deleted_user = users.remove(index);
    Some(Json(deleted_user))
}

// Initialize the Rocket application
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get_all_users, get_user, create_user, update_user, delete_user])
}

// Launch the Rocket application
fn main() {
    rocket().launch();
}