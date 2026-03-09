# OSMIUM Web Dashboard Setup

## Setup Instructions

### 1. Get Discord OAuth2 Credentials

1. Go to https://discord.com/developers/applications/1477950780781625457
2. Click on "OAuth2" in the left sidebar
3. Copy your **Client Secret** (keep this private!)
4. Add redirect URLs:
   - For local testing: `http://localhost:5000/callback`
   - For production: `https://your-railway-app.railway.app/callback`

### 2. Update .env File

Add these to your `.env` file:
```
DISCORD_CLIENT_ID=1477950780781625457
DISCORD_CLIENT_SECRET=your_client_secret_from_step_1
REDIRECT_URI=https://your-railway-app.railway.app/callback
FLASK_SECRET_KEY=generate_random_string_here
```

Generate a random secret key:
```bash
python -c "import secrets; print(secrets.token_hex(32))"
```

### 3. Run Locally

```bash
pip install -r requirements.txt
python web_dashboard.py
```

Visit: http://localhost:5000

### 4. Deploy to Railway

1. Push to GitHub
2. In Railway dashboard, add environment variables:
   - `DISCORD_CLIENT_ID`
   - `DISCORD_CLIENT_SECRET`
   - `REDIRECT_URI` (use your Railway domain)
   - `FLASK_SECRET_KEY`
   - `DISCORD_TOKEN`

3. Update Procfile to run web dashboard:
```
web: python web_dashboard.py
```

### 5. Run Both Bot and Dashboard

Create a new file `start.py`:
```python
import threading
import subprocess

def run_bot():
    subprocess.run(['python', 'bot.py'])

def run_web():
    subprocess.run(['python', 'web_dashboard.py'])

if __name__ == '__main__':
    bot_thread = threading.Thread(target=run_bot)
    web_thread = threading.Thread(target=run_web)
    
    bot_thread.start()
    web_thread.start()
    
    bot_thread.join()
    web_thread.join()
```

Update Procfile:
```
web: python start.py
```

## Features

- **Discord OAuth2 Login** - Users login with their Discord account
- **Server Selection** - Shows all servers where user has admin permissions
- **Configuration Panel** - Adjust anti-spam, anti-raid, anti-nuke settings
- **Real-time Updates** - Changes saved to database and applied instantly
- **Modern UI** - Dark theme matching Discord's design

## Dashboard Pages

- `/` - Landing page with features and commands
- `/login` - Discord OAuth2 login
- `/dashboard` - Server selection
- `/server/<guild_id>` - Server configuration panel
- `/api/config/<guild_id>` - API endpoint for saving settings
