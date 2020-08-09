#[allow(unused_imports)]
pub mod test_helpers;

use test_helpers::*;

// mod follows;
mod login;
// mod posting_tweets;
mod user_creation;
// mod timeline;
// mod users;
// mod logout;

#[async_std::test]
async fn creating_a_user() {
    let mut server = test_setup().await;

}