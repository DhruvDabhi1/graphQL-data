    extern crate rocket;
        use rocket::State;
    use query::Query;
     use async_graphql_rocket::{GraphQLRequest as OtherGraphQLRequest, GraphQLResponse};
    use api_try::try_agin::{Mutation,CreateIssue,DeleteIssue, UpdateIssue, RemoveLabelsFromLabelable};
    use async_graphql::{
        http::{playground_source, GraphQLPlaygroundConfig},
        EmptyMutation, EmptySubscription, Schema, ID, Object, SimpleObject, CustomDirective,
    };
    use rocket::{routes, async_trait, http::impl_from_uri_param_identity};

    use schema::{Createlabels, Repository, GetLables};
    use async_graphql::Directive;
    
    extern crate async_graphql;
    use async_graphql::*;
    use async_graphql::ResolveFut;
    use async_graphql::Context;
    // const GITHUB_API_URL: &str = "https://api.github.com/user";
    use crate::api_try::try_agin::graphql_mutation;
    use crate::api_try::try_agin::AddLabelsToLabelable;
    pub mod api_try;
    pub mod all_functions;
    pub mod schema;
    pub mod query;
    
struct ConcatDirective {
    value: String,
}

#[async_trait::async_trait]
impl CustomDirective for ConcatDirective {
    async fn resolve_field(&self, _ctx: &Context<'_>, resolve: ResolveFut<'_>) -> ServerResult<Option<Value>> {
        resolve.await.map(|value| {
            value.map(|value| match value {
                Value::String(str) => Value::String(str + &self.value),
                _ => value,
            })
        })
    }
}

#[Directive(name = "concat",location = "Field")]
fn concat(value: String) -> impl CustomDirective {
    ConcatDirective { value }
}
#[derive(Default)]
struct Hello;

#[Object]
    impl Hello{
        async fn data(&self,input :String) -> String{
            input
        }
    }
    type Say = Schema<Hello, EmptyMutation, EmptySubscription>;
    
    
    struct Planet {
        id: ID,
        
    }
    
    #[Object(extends)]
    impl Planet {
        #[graphql(external)]
        async fn id(&self) -> &ID {
            &self.id
        }
    }
    
    #[derive(Default)]
    struct MainQuery;
    #[Object]
    impl MainQuery{
        async fn product_query(&self) -> Hello {
            Hello::default()
        }
        
        async fn ask_query(&self) -> Query {
        Query::default()
    }
    }

    type SimpleSchema = Schema<MainQuery,EmptyMutation,EmptySubscription>;
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
        // let schema_say = async_graphql::SchemaBuilder::<Hello , EmptyMutation, EmptySubscription>::extension;
        let schema_2 = Schema::build(Hello, EmptyMutation,EmptySubscription);
        // let schema = Schema::build(Query, Mutation, EmptySubscription).data(issue).data(delete).data(update).data(labels).data(remove_labels).data(create_label).data(repo).data(get_lab).directive(concat).finish();
        let schema = Schema::build(MainQuery, Mutation, EmptySubscription).data(issue).data(delete).data(update).data(labels).data(remove_labels).data(create_label).data(repo).data(get_lab).directive(concat).finish();
        rocket::build()
            .manage(schema)
            .mount("/", routes![graphql_handler])
    }

    async fn schema() -> Schema<MainQuery, async_graphql::EmptyMutation, async_graphql::EmptySubscription> {
    Schema::build(MainQuery::default(), async_graphql::EmptyMutation::default(), async_graphql::EmptySubscription)
        .finish()
}

#[rocket::post("/graphql2", data = "<request>")]
async fn graphql_handler(schema:&State<Schema<MainQuery, Mutation, EmptySubscription>>, request:  OtherGraphQLRequest) ->GraphQLResponse {
     request.execute(schema.inner()).await
}