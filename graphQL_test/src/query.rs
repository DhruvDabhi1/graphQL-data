use async_graphql::{Object, FieldResult, Context, ID};
 use async_graphql::parser::Error;
use serde_json::json;
use serde_json::Value as Json;


use crate::Planet;
use crate::schema::GetLables;
use crate::{schema::Repository, all_functions::function_handle::{client, responce_main}};

#[derive(Default)]
    pub struct Query;
    
    #[Object]
    impl Query {
        async fn date(&self) -> FieldResult<String> {
            Ok("hello".to_string())
        }

        async fn getIssue(&self, _ctx: &Context<'_>,input: Repository) -> FieldResult<Result<Json, Error>> {
         let query = json!(format!(
                r#"query {{
  repository(owner: "{}", name: "{}") {{
    issues(first: {}) {{
      nodes {{
        id
        title
        body
        state
      }}
    }}
  }}
}}
    "#,input.owner,input.name,input.issue_need));

         let response = client(query);
          println!("{:#?}", response);
            responce_main(response)
        }
        async fn getlabe(&self, _ctx: &Context<'_>,input: GetLables) -> FieldResult<Result<Json, Error>> {
         let query = json!(format!(
                r#"query {{
  repository(owner: "{}", name: "{}") {{
    issue(number: {}) {{
      id
      title
      labels(first: {}) {{
        nodes {{
          name
          color
        }}
      }}
    }}
  }}
}}
    "#,input.owner,input.name,input.number,input.first));

         let response = client(query);
          println!("{:#?}", response);
            responce_main(response)
        }

          
          #[graphql(entity)]
    async fn get_planet_by_id(&self, #[graphql(key)] id: ID) -> Planet {
        Planet { id }
    }
    }