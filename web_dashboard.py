from flask import Flask, render_template, redirect, url_for, session, request, jsonify
from requests_oauthlib import OAuth2Session
import os
from dotenv import load_dotenv
import requests
from functools import wraps

load_dotenv()

app = Flask(__name__)
app.secret_key = os.getenv('FLASK_SECRET_KEY', 'your-secret-key-change-this')

# Discord OAuth2 Config
CLIENT_ID = os.getenv('DISCORD_CLIENT_ID')
CLIENT_SECRET = os.getenv('DISCORD_CLIENT_SECRET')
REDIRECT_URI = os.getenv('REDIRECT_URI', 'http://localhost:5000/callback')
BOT_TOKEN = os.getenv('DISCORD_TOKEN')

OAUTH2_URL = 'https://discord.com/api/oauth2/authorize'
TOKEN_URL = 'https://discord.com/api/oauth2/token'
API_BASE = 'https://discord.com/api/v10'
SCOPE = ['identify', 'guilds']

os.environ['OAUTHLIB_INSECURE_TRANSPORT'] = '1'  # For local dev

def token_updater(token):
    session['oauth2_token'] = token

def make_session(token=None, state=None):
    return OAuth2Session(
        client_id=CLIENT_ID,
        token=token,
        state=state,
        scope=SCOPE,
        redirect_uri=REDIRECT_URI,
        auto_refresh_kwargs={'client_id': CLIENT_ID, 'client_secret': CLIENT_SECRET},
        auto_refresh_url=TOKEN_URL,
        token_updater=token_updater
    )

def login_required(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        if 'oauth2_token' not in session:
            return redirect(url_for('login'))
        return f(*args, **kwargs)
    return decorated_function

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/login')
def login():
    discord = make_session()
    authorization_url, state = discord.authorization_url(OAUTH2_URL)
    session['oauth2_state'] = state
    return redirect(authorization_url)

@app.route('/callback')
def callback():
    if request.values.get('error'):
        return redirect(url_for('index'))
    
    discord = make_session(state=session.get('oauth2_state'))
    token = discord.fetch_token(
        TOKEN_URL,
        client_secret=CLIENT_SECRET,
        authorization_response=request.url,
        include_client_id=True
    )
    session['oauth2_token'] = token
    return redirect(url_for('dashboard'))

@app.route('/logout')
def logout():
    session.clear()
    return redirect(url_for('index'))

@app.route('/dashboard')
@login_required
def dashboard():
    discord = make_session(token=session.get('oauth2_token'))
    user = discord.get(f'{API_BASE}/users/@me').json()
    guilds = discord.get(f'{API_BASE}/users/@me/guilds').json()
    
    # Filter guilds where user has admin permissions
    admin_guilds = [g for g in guilds if int(g['permissions']) & 0x8]
    
    # Get bot guilds
    bot_guilds = get_bot_guilds()
    bot_guild_ids = [g['id'] for g in bot_guilds]
    
    # Mark which guilds have the bot
    for guild in admin_guilds:
        guild['has_bot'] = guild['id'] in bot_guild_ids
    
    return render_template('dashboard.html', user=user, guilds=admin_guilds)

@app.route('/server/<guild_id>')
@login_required
def server_config(guild_id):
    discord = make_session(token=session.get('oauth2_token'))
    user = discord.get(f'{API_BASE}/users/@me').json()
    guilds = discord.get(f'{API_BASE}/users/@me/guilds').json()
    
    # Check if user has access to this guild
    guild = next((g for g in guilds if g['id'] == guild_id and int(g['permissions']) & 0x8), None)
    if not guild:
        return redirect(url_for('dashboard'))
    
    # Get guild config from bot (placeholder - connect to MongoDB)
    config = get_guild_config(guild_id)
    
    return render_template('server.html', user=user, guild=guild, config=config)

@app.route('/api/config/<guild_id>', methods=['GET', 'POST'])
@login_required
def api_config(guild_id):
    if request.method == 'POST':
        config = request.json
        save_guild_config(guild_id, config)
        return jsonify({'success': True})
    else:
        config = get_guild_config(guild_id)
        return jsonify(config)

def get_bot_guilds():
    headers = {'Authorization': f'Bot {BOT_TOKEN}'}
    try:
        response = requests.get(f'{API_BASE}/users/@me/guilds', headers=headers)
        return response.json() if response.status_code == 200 else []
    except:
        return []

def get_guild_config(guild_id):
    # Placeholder - connect to MongoDB
    return {
        'anti_spam': {'enabled': True, 'max_messages': 5, 'time_window': 5},
        'anti_raid': {'enabled': True, 'max_joins': 10, 'time_window': 10},
        'anti_nuke': {'enabled': True, 'max_actions': 3, 'time_window': 10},
        'log_channel': None,
        'whitelist': []
    }

def save_guild_config(guild_id, config):
    # Placeholder - save to MongoDB
    pass

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000, debug=True)
