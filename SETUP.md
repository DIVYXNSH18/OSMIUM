# OSMIUM Dashboard - Quick Setup

## Step 1: Get Client Secret (30 seconds)

1. Open: https://discord.com/developers/applications/1477950780781625457/oauth2
2. Click "Reset Secret" button
3. Copy the secret that appears
4. Open `.env` file and replace `get_this_from_discord_developer_portal` with your secret

## Step 2: Add Redirect URL (10 seconds)

On the same page, scroll down to "Redirects":
- Click "Add Redirect"
- Paste: `http://localhost:5000/callback`
- Click "Save Changes"

## Step 3: Run Dashboard (5 seconds)

```bash
pip install -r requirements.txt
python web_dashboard.py
```

Open: http://localhost:5000

**That's it!** Click "Login" and you'll see your dashboard.

---

## For Railway Deployment

1. In Discord Developer Portal, add another redirect:
   - `https://your-railway-domain.railway.app/callback`

2. In Railway, set environment variables (copy from `.env`)

3. Change Procfile to:
   ```
   web: python start.py
   ```

4. Push to GitHub - done!
