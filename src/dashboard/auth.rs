use axum::{extract::{Query, State}, response::Redirect};
use serde::{Deserialize, Serialize};
use crate::dashboard::AppState;

#[derive(Deserialize)]
pub struct AuthQuery {
    code: Option<String>,
}

pub async fn login(State(state): State<AppState>) -> Redirect {
    let redirect_uri = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify%20guilds",
        state.config.discord_client_id,
        urlencoding::encode(&state.config.oauth_redirect_uri)
    );
    Redirect::to(&redirect_uri)
}

pub async fn callback(
    State(state): State<AppState>,
    Query(query): Query<AuthQuery>,
) -> Redirect {
    if let Some(code) = query.code {
        match exchange_code(&code, &state).await {
            Ok(token) => {
                return Redirect::to("/dashboard");
            }
            Err(e) => {
                tracing::error!("OAuth error: {}", e);
            }
        }
    }
    
    Redirect::to("/")
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
}

async fn exchange_code(code: &str, state: &AppState) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    
    let params = [
        ("client_id", state.config.discord_client_id.as_str()),
        ("client_secret", state.config.discord_client_secret.as_str()),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", state.config.oauth_redirect_uri.as_str()),
    ];
    
    let response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await?;
    
    let token_response: TokenResponse = response.json().await?;
    
    Ok(token_response.access_token)
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_jwt(user_id: &str, secret: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use jsonwebtoken::{encode, Header, EncodingKey};
    
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
    
    Ok(token)
}
