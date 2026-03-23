use salvo_oapi::endpoint;

#[endpoint(
    tags("Health"),
    summary = "This endpoint allows you to check the health of the API",
    description = "This endpoint returns a simple 'OK' message if the API is healthy. It can be used for monitoring and health checks."
)]
pub async fn health() -> &'static str {
    "OK"
}