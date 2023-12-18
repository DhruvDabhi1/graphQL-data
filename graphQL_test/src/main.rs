extern crate rocket;

use query_back::try_agin::{Mutation, Query, CreateIssue};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use rocket::routes;
// const GITHUB_API_URL: &str = "https://api.github.com/user";
use crate::query_back::try_agin::graphql_mutation;
pub mod query_back;
#[rocket::launch]
fn rocket() -> _ {
    let issue = CreateIssue::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription).data(issue).finish();
    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_mutation])
}
