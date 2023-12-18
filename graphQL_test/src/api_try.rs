pub mod try_agin {
    use async_graphql::{
        Context, EmptySubscription, FieldResult, InputObject, Object, Response, Schema,
        SimpleObject, Value,
    };
    use async_graphql_rocket::{GraphQLRequest as OtherGraphQLRequest, GraphQLResponse};
    use isahc::{ReadResponseExt, RequestExt};
    use rocket::State;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value as Json};
    use std::env;
    use dotenv::dotenv;

    use crate::api_try;

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
        repositoryName:String,
        issueNumber: String
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
    pub struct AddLabelsToLabelable{
        issueId: String,
        labelIds: Vec<String>
    }
    #[derive(InputObject, Serialize, Debug)]
    pub struct RemoveLabelsFromLabelable{
        issueId: String,
        labelIds: Vec<String>
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
                issueNumber:String::new(),
                repositoryOwner: String::new(),
                repositoryName:String::new()
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

    impl AddLabelsToLabelable{
      pub fn init()->Self{
            AddLabelsToLabelable{
               issueId: String::new(),
                labelIds: Vec::new()
            }
        }
    }
    impl RemoveLabelsFromLabelable{
      pub fn init()->Self{
            RemoveLabelsFromLabelable{
               issueId: String::new(),
                labelIds: Vec::new()
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
            dotenv().ok();
            let github_token = env::var("GITHUB_TOKEN").expect("not able to dound");
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
                .header("Authorization", format!("Bearer {}", github_token))
                .body(json!({ "query": query }).to_string())
                .unwrap()
                .send();

            println!("{:#?}", response);
            match response {
                Ok(mut res) => {
                    let body = res.text().unwrap();
                    let json: Json = serde_json::from_str(&body).unwrap();
                    println!("{:#?}", json);
                    let data = CreateIssueResponse {
                        issue: Some(Issue {
                            number: Some("created succes fully".to_string()),
                            url: Some("all set and done".to_string()),
                            title: Some("all set and done".to_string()),
                        }),
                    };
                    //Issue {
                    //     number: Some("created succes fully".to_string()),
                    //     url: Some("all set and done".to_string()),
                    // };

                    Ok(data)
                }
                Err(_) => Ok(api_try::try_agin::CreateIssueResponse { issue: None }),
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

        async fn Deleteissue(&self, _ctx: &Context<'_>, input: DeleteIssue) -> FieldResult<String> {
            dotenv().ok();
            let GITHUB_TOKEN: String = env::var("GITHUB_TOKEN").expect("not able to dound");
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
            let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
                isahc::http::Request::post(GITHUB_GRAPHQL_URL)
                    .header("Authorization", format!("Bearer {}", GITHUB_TOKEN))
                    .body(json!({ "query": query }).to_string())
                    .unwrap()
                    .send();
            println!("{:#?}", response);

            // Ok(ClientMutationId {
            //     data: Some("issue id deleted".to_string()),
            // })
            Ok("deleted".to_string())
        }

        async fn updateIssue(&self, input: UpdateIssue) -> FieldResult<String> {
            dotenv().ok();
            let github_token = env::var("GITHUB_TOKEN").expect("not able to dound");
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
            let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
                isahc::http::Request::post(GITHUB_GRAPHQL_URL)
                    .header("Authorization", format!("Bearer {}", github_token))
                    .body(json!({ "query": query }).to_string())
                    .unwrap()
                    .send();
                
                match response {
                Ok(mut res) => {
                    // Assuming you want to print the response for debugging
                    let body = res.text().unwrap();
                    let json: Json = serde_json::from_str(&body).unwrap();
                    println!("{:#?}", json);
                    
                    // Extract and print the user's name and repositories
                    if let Some(viewer) = json["data"]["viewer"].as_object() {
                        if let Some(login) = viewer.get("login") {
                            println!("User's login: {}", login);
                        }
                        
                        if let Some(repositories) = viewer.get("repositories") {
                            println!("User's repositories:");
                            if let Some(nodes) = repositories["nodes"].as_array() {
                                for node in nodes {
                                    if let Some(name) = node["name"].as_str() {
                                        println!("{}", name);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {:#?}", err);
                }
            }
            
            // println!("{:#?}",response);
            Ok("updated done".to_string())
        }

        async fn addLabelsToLabelable(&self,input: AddLabelsToLabelable) -> FieldResult<String>{
            dotenv().ok();
            let github_token = env::var("GITHUB_TOKEN").expect("not able to dound");
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
               input.issueId,input.labelIds[0]
            ));
            let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
                isahc::http::Request::post(GITHUB_GRAPHQL_URL)
                    .header("Authorization", format!("Bearer {}", github_token))
                    .body(json!({ "query": query }).to_string())
                    .unwrap()
                    .send();
                println!("{:#?}",response);
                Ok("lables added".to_string())
        }
        async fn removeLabelsFromLabelable(&self,input:RemoveLabelsFromLabelable) -> FieldResult<String>{
            dotenv().ok();
            let github_token = env::var("GITHUB_TOKEN").expect("not able to dound");
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
               input.issueId,input.labelIds[0]
            ));
            let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
                isahc::http::Request::post(GITHUB_GRAPHQL_URL)
                    .header("Authorization", format!("Bearer {}", github_token))
                    .body(json!({ "query": query }).to_string())
                    .unwrap()
                    .send();
                println!("{:#?}",response);
                Ok("lables removed".to_string())
        }
    }
    
    pub fn client(query: Json) -> Result<isahc::Response<isahc::Body>, isahc::Error> {
        dotenv().ok();
        let GITHUB_TOKEN: String = env::var("GITHUB_TOKEN").expect("not able to dound");
        let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
            isahc::http::Request::post(GITHUB_GRAPHQL_URL)
            .header("Authorization", format!("Bearer {}", GITHUB_TOKEN))
                .body(json!({ "query": query }).to_string())
                .unwrap()
                .send();
        response
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
/*
mutation {
  deleteIssue(input: {
    issueId: "ISSUE_ID"
  }) {
    clientMutationId
  }
}
 */

/*
 Object {
    "data": Object {
        "createIssue": Object {
            "issue": Object {
                "number": Number(11),
                "url": String("https://github.com/DhruvDabhi1/graphQL-repo-practice/issues/11"),
            },
        },
    },
} */

/*
mutation {
  updateIssue(
    input: {
      issueId: "ISSUE_ID"
      title: "New Title"
      body: "New Body"
    }
  ) {
    issue {
      title
      body
    }
  }
}
 */
/*
mutation UpdateIssue($repositoryOwner: String!, $repositoryName: String!, $issueNumber: Int!, $title: String, $body: String) {
  updateIssue(input: {
    repositoryOwner: $repositoryOwner,
    repositoryName: $repositoryName,
    issueNumber: $issueNumber,
    title: $title,
    body: $body
  }) {
    issue {
      title
      body
    }
  }
} */

//lables
/*
mutation {
  addLabelsToLabelable(input: {
    labelableId: "ISSUE_ID",
    labelIds: ["LABEL_ID_1", "LABEL_ID_2"]
  }) {
    labelable {
      ... on Issue {
        id
        title
        labels(first: 5) {
          nodes {
            name
          }
        }
      }
    }
  }
}

 */