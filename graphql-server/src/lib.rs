use wasmcloud_component::http;
wit_bindgen::generate!({ generate_all });
use crate::wasi::logging::logging::*;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, 
    GraphQLEnum, Variables, 
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    // Note, that the enum value will be automatically converted to the
    // `SCREAMING_SNAKE_CASE` variant, just as GraphQL conventions imply.
    NewHope,
    Empire,
    Jedi,
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
}

// mutation {
//     action(id: "1de601200e7e42559952df0b37c150ad", input: $input)
//   }

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn gql() -> String {
    // Create a context.
    let ctx = Ctx(Episode::NewHope);
    
    // let result = bettyblocks::runtime_cloud::action_runner::execute();
    log(Level::Info, "", &format!("Data received: {:?}", "hit"));

    // Run the execution.
    let (res, _errors) = juniper::execute_sync(
        "query { favoriteEpisode }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();

    format!("{}", res)
}

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let str = format!("Hellos, {}!", gql());
        Ok(http::Response::new(str))
    }
}
