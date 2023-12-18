extern crate rocket;

use api_try::try_agin::{Mutation, Query, CreateIssue,DeleteIssue, UpdateIssue, RemoveLabelsFromLabelable};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use rocket::routes;
// const GITHUB_API_URL: &str = "https://api.github.com/user";
use crate::api_try::try_agin::graphql_mutation;
use crate::api_try::try_agin::AddLabelsToLabelable;
pub mod api_try;
#[rocket::launch]
fn rocket() -> _ {
    let issue = CreateIssue::init();
    let delete = DeleteIssue::init();
    let update = UpdateIssue::init();
    let labels = AddLabelsToLabelable::init();
    let remove_labels = RemoveLabelsFromLabelable::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription).data(issue).data(delete).data(update).data(labels).data(remove_labels).finish();
    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_mutation])
}
