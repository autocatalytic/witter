use crate::tests::test_helpers::*;

#[async_std::test]
async fn posting_a_valid_tweet() {
    let mut server = test_setup().await;

    let token = create_user_and_authenticate(&mut server, None).await.token;

    let (json, status, _) = post(
        "/tweets",
        Some(CreateTweetPayload {
            text: "Hello, World!".to_string(),
        },
    ))
    .header("Authorization", format!("Bearer {}", token))
    .send(&mut server)
    .await;
    assert_eq!(status, 201);

    assert_json_include!(
        actual: json,
        expected: json!({
            "data": {
                "text": "Hello, World!"
            }
        })
    );
}

#[async_std::test]
async fn posting_a_tweet_that_is_too_long() {
    use shared::MAX_TWEET_LENGTH;

    let mut server = test_setup().await;

    let token = create_user_and_authenticate(&mut server, None).await.token;

    let text = std::iter::repeat('a').take(1000).collect::<String>();
    let (json, status, _) = post("/tweets", Some(CreateTweetPayload { text }))
        .header("Authorization", format!("Bearer {}", token))
        .send(&mut server)
        .await;
    assert_eq!(status, 422);

    assert_json_include!(
        actual: json,
        expected: json!({
            "error": {
                "message": format!("Tweet is too long. Max then is {}", MAX_TWEET_LENGTH)
            }
        })
    );
}

#[async_std::test]
async fn posting_a_tweet_that_is_exactly_the_max_length() {
    use shared::MAX_TWEET_LENGTH;

    let mut server = test_setup().await;

    let token = create_user_and_authenticate(&mut server, None).await.token;

    let text = std::iter::repeat('a').take(MAX_TWEET_LENGTH).collect::<String>();
    let (json, status, _) = post("/tweets", Some(CreateTweetPayload { text }))
        .header("Authorization", format!("Bearer {}", token))
        .send(&mut server)
        .await;
    assert_eq!(status, 201);

    assert_json_include!(
        actual: json,
        expected: json!({
            "data": {}
        })
    );
}