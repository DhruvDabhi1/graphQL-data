    extern crate rocket;
    use query::Query;
    use api_try::try_agin::{Mutation,CreateIssue,DeleteIssue, UpdateIssue, RemoveLabelsFromLabelable};
    use async_graphql::{
        http::{playground_source, GraphQLPlaygroundConfig},
        EmptyMutation, EmptySubscription, Schema,
    };
    use rocket::routes;
    use schema::{Createlabels, Repository, GetLables};
    // const GITHUB_API_URL: &str = "https://api.github.com/user";
    use crate::api_try::try_agin::graphql_mutation;
    use crate::api_try::try_agin::AddLabelsToLabelable;
    pub mod api_try;
    pub mod all_functions;
    pub mod schema;
    pub mod query;
    #[rocket::launch]
    fn rocket() -> _ {
        let issue = CreateIssue::init();
        let delete = DeleteIssue::init();
        let update = UpdateIssue::init();
        let create_label = Createlabels::init();
        let labels = AddLabelsToLabelable::init();
        let remove_labels = RemoveLabelsFromLabelable::init();
        let repo = Repository::init();
        let get_lab = GetLables::init();
        let schema = Schema::build(Query, Mutation, EmptySubscription).data(issue).data(delete).data(update).data(labels).data(remove_labels).data(create_label).data(repo).data(get_lab).finish();
        rocket::build()
            .manage(schema)
            .mount("/", routes![graphql_mutation])
    }
