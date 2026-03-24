mod common;

#[tokio::test]
async fn smoke_test_postgrest_is_reachable() {
    let (client, config) = common::setup().await;
    assert!(client.get(&config.supabase_url).send().await.is_ok());
}

#[tokio::test]
async fn smoke_test_auth_is_reachable() {
    let (client, config) = common::setup().await;

    let response = client
        .get(format!("{}/health", config.auth_url))
        .send()
        .await
        .expect("Could not connect to GoTrue");

    assert!(
        response.status().is_success(),
        "GoTrue returned unexpected status: {}",
        response.status()
    );

    log::info!("✓ GoTrue responded at {}", config.auth_url);
}

#[tokio::test]
async fn smoke_test_anon_key_is_accepted() {
    let (client, config) = common::setup().await;

    let response = client
        .get(&config.supabase_url)
        .header("apikey", &config.anon_key)
        .header("Authorization", format!("Bearer {}", config.anon_key))
        .send()
        .await
        .expect("Request failed");

    assert_ne!(
        response.status().as_u16(),
        401,
        "The anon key was rejected - check JWT_SECRET in .env.test"
    );

    log::info!("✓ Anon key accepted by PostgREST");
}
