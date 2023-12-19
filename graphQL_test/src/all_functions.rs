    pub mod function_handle{
    use std::env;

    use async_graphql::{FieldResult, parser::Error};
    use isahc::{ReadResponseExt, RequestExt};
    use serde_json::{Value as Json, json};
     use dotenv::dotenv;

    use crate::api_try::try_agin::GITHUB_GRAPHQL_URL;

      pub fn responce_main(response :  Result<isahc::Response<isahc::Body>, isahc::Error> ) -> FieldResult<Result<Json, Error>> {
           match response {
            Ok(mut res) => {
                let body = res.text().unwrap();
                let json: Json = serde_json::from_str(&body).unwrap();
                println!("{:#?}", json);
                Ok(Ok(json))
                }
                Err(err) => {
                Err("not able to create".to_string().into())
               
            }
        }
    }

     pub fn client(query: Json) -> Result<isahc::Response<isahc::Body>, isahc::Error> {
        dotenv().ok();
        let github_token = env::var("GITHUB_TOKEN").expect("not able to dound");
        let response: Result<isahc::Response<isahc::Body>, isahc::Error> =
            isahc::http::Request::post(GITHUB_GRAPHQL_URL)
            .header("Authorization", format!("Bearer {}", github_token))
                .body(json!({ "query": query }).to_string())
                .unwrap()
                .send();
        response
    }
}   