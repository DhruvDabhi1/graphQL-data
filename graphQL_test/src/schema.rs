use async_graphql::InputObject;
use serde::Serialize;


 #[derive(InputObject, Serialize, Debug)]
 pub struct Createlabels{
     pub repositoryId: String,
     pub name: String,
     pub color: String
    }
    #[derive(InputObject, Serialize, Debug)]
    pub struct Repository {
        pub  owner: String,
        pub  name:String,
        pub issue_need:String
    }
    
    #[derive(InputObject, Serialize, Debug)]
    pub struct GetLables{
        pub owner: String,
        pub name: String,
        pub number: String,
        pub first: String
    }
