pub mod try_agin {

    use async_graphql::{
        Context, FieldResult, InputObject, Object,
     Schema, SimpleObject, EmptySubscription,
    };
    use async_graphql_rocket::{
         GraphQLRequest as OtherGraphQLRequest, GraphQLResponse,
    };
    use isahc::{ReadResponseExt, RequestExt};
    use rocket:: State;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value as Json};

    use crate::query_back;

    pub struct Mutation;
    pub const GITHUB_GRAPHQL_URL: &str = "https://api.github.com/graphql";
    const GITHUB_TOKEN: &str = "yoru gitHub token";


    #[rocket::post("/graphql", data = "<request>")]
    pub async fn graphql_mutation(
        schema: &State<ProjectSchema>,
        request: OtherGraphQLRequest,
    ) -> GraphQLResponse {
        // println!("{:#?}",request);
        request.execute(schema.inner()).await
    }

    #[derive(InputObject, Serialize, Debug)]
    pub struct CreateIssue {
        repositoryId: String, // Update the field name here
        title: String,
        body: String,
    }

    #[derive(Debug, Deserialize, SimpleObject)]
    pub struct CreateIssueResponse {
        issue: Option<Issue>,
    }

    #[derive(Debug, Deserialize, SimpleObject)]
    pub struct Issue {
        number: Option<String>,
        url: Option<String>,
    }

    impl CreateIssue {
        pub fn init() -> Self {
            CreateIssue {
                repositoryId: String::new(),
                title: String::new(),
                body: String::new(),
            }
        }
    }

    #[Object]
    impl Mutation {
        async fn CreateIssue(
            &self,
            _ctx: &Context<'_>,
            input: CreateIssue,
        ) -> FieldResult<CreateIssueResponse> {
            println!("{:?}", input);
            // let query = json!({"query":input});
            // let query = json!("query");
            let query = json!(format!(
                r#" mutation {{
  createIssue(
    input: {{
      repositoryId: "{}"
      title: "{}"
      body: "{}"}}
  ) {{
    issue {{
      number
      url
    }}
  }}
}}"#,
                input.repositoryId, input.title, input.body
            ));
            let response = isahc::http::Request::post(GITHUB_GRAPHQL_URL)
                .header("Authorization", format!("Bearer {}", GITHUB_TOKEN))
                .body(json!({ "query": query }).to_string())
                .unwrap()
                .send();

            match response {
                Ok(mut res) => {
                    let body = res.text().unwrap();
                    let json: Json = serde_json::from_str(&body).unwrap();
                    println!("{:#?}", json);
                    let data = CreateIssueResponse {
                        issue: Some(Issue {
                            number: Some("created succes fully".to_string()),
                            url: Some("all set and done".to_string()),
                        }),
                    };
                    //Issue {
                    //     number: Some("created succes fully".to_string()),
                    //     url: Some("all set and done".to_string()),
                    // };

                    Ok(data)
                }
                Err(_) => Ok(query_back::try_agin::CreateIssueResponse { issue: None }),
            }

            /*
            match response {
                Ok(mut res) => {
                    let body = res.text().unwrap();
                    let json: Json = serde_json::from_str(&body).unwrap();
                    let result: CreateIssueResponse = serde_json::from_value(json)?;
                    println!("{:#?}", result);
                    Ok(result)
                }
                Err(_) => Ok(CreateIssueResponse { issue: None }),
            }
            */
        }
    }
    pub struct Query;
    #[Object]
    impl Query {
        async fn data(&self) -> FieldResult<String> {
            Ok("hello".to_string())
        }
    }
    pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
}
