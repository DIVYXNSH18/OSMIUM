use axum::{extract::State, response::Html, extract::Path};
use crate::dashboard::AppState;

pub async fn landing() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Discord Security Bot</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            text-align: center;
            padding: 2rem;
        }
        h1 {
            font-size: 3rem;
            margin-bottom: 1rem;
        }
        p {
            font-size: 1.2rem;
            margin-bottom: 2rem;
        }
        .btn {
            display: inline-block;
            padding: 1rem 2rem;
            background: white;
            color: #667eea;
            text-decoration: none;
            border-radius: 8px;
            font-weight: bold;
            transition: transform 0.2s;
        }
        .btn:hover {
            transform: scale(1.05);
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🛡️ Discord Security Bot</h1>
        <p>Protect your Discord server with advanced security features</p>
        <a href="/auth/login" class="btn">Login with Discord</a>
    </div>
</body>
</html>
    "#)
}

pub async fn dashboard(State(state): State<AppState>) -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dashboard - Discord Security Bot</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: #1a1a2e;
            color: #eee;
        }
        .header {
            background: #16213e;
            padding: 1rem 2rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.3);
        }
        .container {
            max-width: 1200px;
            margin: 2rem auto;
            padding: 0 2rem;
        }
        .guild-card {
            background: #16213e;
            padding: 1.5rem;
            margin-bottom: 1rem;
            border-radius: 8px;
            cursor: pointer;
            transition: transform 0.2s;
        }
        .guild-card:hover {
            transform: translateY(-2px);
        }
        h1 { margin-bottom: 2rem; }
    </style>
</head>
<body>
    <div class="header">
        <h2>🛡️ Discord Security Bot</h2>
    </div>
    <div class="container">
        <h1>Your Servers</h1>
        <div class="guild-card">
            <h3>Select a server to configure</h3>
            <p>Click on a server to manage its security settings</p>
        </div>
    </div>
</body>
</html>
    "#)
}

pub async fn guild_settings(
    State(state): State<AppState>,
    Path(guild_id): Path<String>,
) -> Html<String> {
    let config = state.database.queries.get_guild_config(&guild_id).await.unwrap_or_default();
    
    let html = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Guild Settings - Discord Security Bot</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: #1a1a2e;
            color: #eee;
        }}
        .header {{
            background: #16213e;
            padding: 1rem 2rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.3);
        }}
        .container {{
            max-width: 1200px;
            margin: 2rem auto;
            padding: 0 2rem;
        }}
        .section {{
            background: #16213e;
            padding: 1.5rem;
            margin-bottom: 1rem;
            border-radius: 8px;
        }}
        .toggle {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem 0;
            border-bottom: 1px solid #0f3460;
        }}
        .toggle:last-child {{
            border-bottom: none;
        }}
        .switch {{
            position: relative;
            display: inline-block;
            width: 60px;
            height: 34px;
        }}
        .switch input {{
            opacity: 0;
            width: 0;
            height: 0;
        }}
        .slider {{
            position: absolute;
            cursor: pointer;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: #ccc;
            transition: .4s;
            border-radius: 34px;
        }}
        .slider:before {{
            position: absolute;
            content: "";
            height: 26px;
            width: 26px;
            left: 4px;
            bottom: 4px;
            background-color: white;
            transition: .4s;
            border-radius: 50%;
        }}
        input:checked + .slider {{
            background-color: #2196F3;
        }}
        input:checked + .slider:before {{
            transform: translateX(26px);
        }}
        h1 {{ margin-bottom: 2rem; }}
        h2 {{ margin-bottom: 1rem; }}
    </style>
</head>
<body>
    <div class="header">
        <h2>🛡️ Discord Security Bot</h2>
    </div>
    <div class="container">
        <h1>Server Configuration</h1>
        
        <div class="section">
            <h2>Security Features</h2>
            <div class="toggle">
                <span>Anti-Nuke Protection</span>
                <label class="switch">
                    <input type="checkbox" {}>
                    <span class="slider"></span>
                </label>
            </div>
            <div class="toggle">
                <span>Anti-Raid Protection</span>
                <label class="switch">
                    <input type="checkbox" {}>
                    <span class="slider"></span>
                </label>
            </div>
            <div class="toggle">
                <span>Anti-Spam Protection</span>
                <label class="switch">
                    <input type="checkbox" {}>
                    <span class="slider"></span>
                </label>
            </div>
            <div class="toggle">
                <span>Anti-Mention Protection</span>
                <label class="switch">
                    <input type="checkbox" {}>
                    <span class="slider"></span>
                </label>
            </div>
        </div>
        
        <div class="section">
            <h2>Verification</h2>
            <div class="toggle">
                <span>Enable Verification</span>
                <label class="switch">
                    <input type="checkbox" {}>
                    <span class="slider"></span>
                </label>
            </div>
        </div>
    </div>
</body>
</html>
    "#,
        if config.antinuke.enabled { "checked" } else { "" },
        if config.antiraid.enabled { "checked" } else { "" },
        if config.antispam.enabled { "checked" } else { "" },
        if config.antimention.enabled { "checked" } else { "" },
        if config.verification.enabled { "checked" } else { "" }
    );
    
    Html(html)
}
