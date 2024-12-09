use wasmcloud_component::http;
wit_bindgen::generate!({ generate_all });
// use crate::wasi::logging::logging::*;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, Variables,
};

use juniper::FieldResult;

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    // Note, that the enum value will be automatically converted to the
    // `SCREAMING_SNAKE_CASE` variant, just as GraphQL conventions imply.
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: i32,
    name: String,
    // appears_in: Vec<Episode>,
    home_planet: String,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object]
#[graphql(context = Ctx)]
impl Query {
    fn favorite_episode(context: &Ctx) -> Episode {
        context.0
    }
    fn all_human() -> FieldResult<Human> {
        Ok(Human {
            id: 1,
            name: "chris".to_string(),
            home_planet: "tatooine".to_string(),
        })
    }
}

// mutation {
//     action(id: "1de601200e7e42559952df0b37c150ad", input: $input)
//   }

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn gql(query: &str) -> String {
    // Create a context.
    let ctx = Ctx(Episode::NewHope);

    // Run the execution.
    let (res, _errors) = juniper::execute_sync(
        query,
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    format!("{}", res)
}

fn get_config() -> String {
    wasi::config::runtime::get("bb_runtime_cloud-gql_config")
        .expect("Unable to fetch value")
        .unwrap_or_else(|| "config value not set".to_string())
}

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let str = bettyblocks::runtime_cloud::action_runner::execute();
        let str = format!(
            "Hallos, {} {} {}!",
            gql("query { favoriteEpisode }"),
            str,
            get_config()
        );
        Ok(http::Response::new(str))
    }
}

#[cfg(test)]
mod tests {
    // use wasi::http::outgoing_handler::handle;

    use super::*;

    #[test]
    fn it_works() {
        let res = gql("query { allHuman { id \n name }}");
        assert_eq!(res, "{\"favoriteEpisode\": \"NEW_HOPE\"}")
    }

    // fn test_handle() {
    // use http::{Request, Response};
    // let request: http::IncomingRequest = nil;
    // let mut request = Request::builder()
    //     .uri("https://www.rust-lang.org/")
    //     .header("User-Agent", "my-awesome-agent/1.0");
    // let res = handle(request);
    // let response = Ok(http::Response::new(""));
    // assert_eq!(res, response);
    // }
}
