#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::{
    response::content,
    State,
};
use std::{env, error, fmt};
use rust_graphql_study::{Context,QueryRoot};

type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Context {})
        .manage(Schema::new(
            QueryRoot,
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
