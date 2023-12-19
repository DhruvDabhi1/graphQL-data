pub mod try_agin {
    use async_graphql::parser::Error;
    use async_graphql::{
        Context, EmptySubscription, FieldResult, InputObject, Object, Schema,
        SimpleObject
    };
        use serde_json::Value as Json;


    use async_graphql_rocket::{GraphQLRequest as OtherGraphQLRequest, GraphQLResponse};
    use dotenv::dotenv;
    use rocket::State;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    

    use crate::all_functions::function_handle::{client, responce_main};
    use crate::schema::{Createlabels, Repository, GetLables};
    use crate::query::Query;

    pub struct Mutation;
    pub const GITHUB_GRAPHQL_URL: &str = "https://api.github.com/graphql";
    // const GITHUB_TOKEN: &str = "yoru gitHub token";
    // const GITHUB_TOKEN: String = env::var("GITHUB_TOKEN").unwrap();

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
    #[derive(InputObject, Serialize, Debug)]
    pub struct UpdateIssue {
        issueId: String, // Update the field name here
        title: String,
        body: String,
        repositoryOwner: String,
        repositoryName: String,
        issueNumber: String,
    }

    #[derive(Debug, Deserialize, SimpleObject)]
    pub struct CreateIssueResponse {
        issue: Option<Issue>,
    }

    #[derive(Debug, Deserialize, SimpleObject)]
    pub struct Issue {
        number: Option<String>,
        url: Option<String>,
        title: Option<String>,
    }

    #[derive(InputObject, Serialize, Debug)]
    pub struct DeleteIssue {
        issue_id: String,
    }

    #[derive(Debug, Deserialize, SimpleObject)]
    pub struct ClientMutationId {
        data: Option<String>,
    }

    #[derive(InputObject, Serialize, Debug)]
    pub struct AddLabelsToLabelable {
        issueId: String,
        labelIds: Vec<String>,
    }
    #[derive(InputObject, Serialize, Debug)]
    pub struct RemoveLabelsFromLabelable {
        issueId: String,
        labelIds: Vec<String>,
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
    impl UpdateIssue {
        pub fn init() -> Self {
            UpdateIssue {
                issueId: String::new(),
                title: String::new(),
                body: String::new(),
                issueNumber: String::new(),
                repositoryOwner: String::new(),
                repositoryName: String::new(),
            }
        }
    }

    impl DeleteIssue {
        pub fn init() -> Self {
            DeleteIssue {
                issue_id: String::new(),
            }
        }
    }

    impl AddLabelsToLabelable {
        pub fn init() -> Self {
            AddLabelsToLabelable {
                issueId: String::new(),
                labelIds: Vec::new(),
            }
        }
    }
    impl RemoveLabelsFromLabelable {
        pub fn init() -> Self {
            RemoveLabelsFromLabelable {
                issueId: String::new(),
                labelIds: Vec::new(),
            }
        }
    }

    impl Createlabels{
        pub fn init() -> Self{
            Createlabels { repositoryId: String::new(), name: String::new(), color: String::new() }
        }
    }

    impl Repository{
        pub fn init() -> Self{
            Repository { owner: String::new(), name: String::new(),issue_need:String::new() }
        }
    }

    impl GetLables{
        pub fn init()-> Self{
            GetLables { owner:String::new(), name: String::new(), number: String::new(), first: String::new() }
        }
    }


    

    
    
    
    
    #[Object]
    impl Mutation {
        async fn create_labels(&self, input : Createlabels) ->FieldResult<Result<Json, Error>> {
              let query = json!(format!(
                r#"mutation {{
    createLabel(input: {{
    repositoryId: "{}",
    description: "mine"
    name:"{}",
    color:"{}",
    }}) {{
    label {{
      id
      name
      color
    }}
    }}
    }}
    "#,
                input.repositoryId, input.name, input.color
            ));
            let response = client(query);
    
            println!("{:#?}", response);
            responce_main(response)
        }
        async fn CreateIssue(
            &self,
            _ctx: &Context<'_>,
            input: CreateIssue,
        ) -> FieldResult<Result<Json, Error>> {
            dotenv().ok();
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
            let response = client(query);

            println!("{:#?}", response);
            responce_main(response)

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

        async fn Deleteissue(
            &self,
            _ctx: &Context<'_>,
            input: DeleteIssue,
        ) -> FieldResult<Result<Json, Error>> {
            dotenv().ok();
          
            let query = json!(format!(
                r#"mutation {{
                    deleteIssue(input: {{
                        issueId: "{}"
                    }}) {{
                        clientMutationId
  }}
}}"#,
                input.issue_id
            ));
            let response = client(query);
            println!("{:#?}", response);

            // Ok(ClientMutationId {
            //     data: Some("issue id deleted".to_string()),
            // })
            responce_main(response)
        }

        async fn updateIssue(&self, input: UpdateIssue) -> FieldResult<Result<Json, Error>> {
            dotenv().ok();
          println!("{:?}",input);
            let query = json!(format!(
                r#"
                mutation {{
                updateIssue(
                    input: {{
                        id: "{}",
                        title: "{}",
                        body: "{}"
                    }}
                ) {{
                    issue {{
                        title
                        body
                    }}
                }}
            }}"#,
                input.issueId, input.title, input.body
            ));

            // let response = client(query);
            let response = client(query);

            responce_main(response)

            // println!("{:#?}",response);
        }

        async fn addLabelsToLabelable(
            &self,
            input: AddLabelsToLabelable,
        ) -> FieldResult<Result<Json, Error>> {
            dotenv().ok();
            let query = json!(format!(
                r#"mutation {{
                    addLabelsToLabelable(input: {{
    labelableId: "{}",
    labelIds: ["{}"]
  }}) {{
    labelable {{
      ... on Issue {{
        id
        title
        labels(first: 5) {{
          nodes {{
            name
          }}
        }}
      }}
    }}
  }}
}}"#,
                input.issueId, input.labelIds[0]
            ));
            let response = client(query);
            println!("{:#?}", response);
            responce_main(response)
        }
        async fn removeLabelsFromLabelable(
            &self,
            input: RemoveLabelsFromLabelable,
        ) -> FieldResult<Result<Json, Error>> {
            dotenv().ok();
            let query = json!(format!(
                r#"mutation {{
                    removeLabelsFromLabelable(input: {{
                        labelableId: "{}",
    labelIds: ["{}"]
}}) {{
    labelable {{
      ... on Issue {{
        id
        title
        labels(first: 5) {{
          nodes {{
            name
        }}
        }}
      }}
    }}
  }}
}}"#,
                input.issueId, input.labelIds[0]
            ));
            let response = client(query);
            responce_main(response)
        }
    }

    pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
}
