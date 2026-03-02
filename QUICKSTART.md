# Quick Start Guide

Get your Discord Security Bot running in 5 minutes!

## Prerequisites

- Rust installed ([rustup.rs](https://rustup.rs))
- MongoDB running locally or remote
- Redis running locally or remote
- Discord Bot Token

## Step 1: Clone and Setup (1 minute)

```bash
cd discord-security-bot
cp .env.example .env
```

Edit `.env` with your Discord bot token:
```env
DISCORD_TOKEN=your_bot_token_here
DISCORD_CLIENT_ID=your_client_id
DISCORD_CLIENT_SECRET=your_client_secret
```

## Step 2: Start Services (1 minute)

### MongoDB
```bash
# Ubuntu/Debian
sudo systemctl start mongodb

# macOS
brew services start mongodb-community

# Docker
docker run -d -p 27017:27017 mongo:latest
```

### Redis
```bash
# Ubuntu/Debian
sudo systemctl start redis

# macOS
brew services start redis

# Docker
docker run -d -p 6379:6379 redis:latest
```

## Step 3: Build and Run (3 minutes)

```bash
# Build (first time takes 2-3 minutes)
cargo build --release

# Run
cargo run --release
```

Or use the build script:
```bash
chmod +x build.sh
./build.sh
./target/release/discord-security-bot
```

## Step 4: Invite Bot to Server

Use this URL (replace YOUR_CLIENT_ID):
```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

## Step 5: Initial Configuration

In your Discord server:

```
/setlogchannel #logs
/security antinuke enabled:true
/security antiraid enabled:true
/security antispam enabled:true
/whitelist add role:@Admin
```

## Done! 🎉

Your bot is now protecting your server!

## Quick Commands Reference

### Security
- `/scan` - Run security audit
- `/config` - View configuration
- `/security antinuke enabled:true` - Enable anti-nuke

### Moderation
- `/ban @user reason:Spamming` - Ban user
- `/kick @user` - Kick user
- `/mute @user duration:60 reason:Timeout` - Mute for 60 minutes
- `/warn @user reason:Breaking rules` - Warn user
- `/clear amount:50` - Delete 50 messages
- `/lock` - Lock current channel
- `/lockall` - Lock all channels

### Whitelist
- `/whitelist add user:@TrustedUser` - Add to whitelist
- `/whitelist add role:@Moderator` - Add role to whitelist
- `/whitelist list` - View whitelist

### Verification
- `/setup_verification enabled:true method:button` - Setup verification
- `/send_verify_button channel:#verify` - Send verify button

## Web Dashboard

Access at: `http://localhost:3000`

1. Click "Login with Discord"
2. Authorize the application
3. Select your server
4. Configure settings via web interface

## Troubleshooting

### Bot not responding?
- Check `DISCORD_TOKEN` in `.env`
- Verify bot has Administrator permission
- Check all intents are enabled in Discord Developer Portal

### Database errors?
- Ensure MongoDB is running: `systemctl status mongodb`
- Check connection string in `.env`

### Redis errors?
- Ensure Redis is running: `systemctl status redis`
- Test with: `redis-cli ping` (should return PONG)

## Next Steps

- Read [README.md](README.md) for detailed documentation
- Check [FEATURES.md](FEATURES.md) for complete feature list
- Configure thresholds via web dashboard
- Set up verification system
- Run `/scan` to audit your server

## Support

- GitHub Issues: Report bugs and request features
- Documentation: See README.md for comprehensive guide

## Production Deployment

For production, see README.md sections on:
- Docker deployment
- Systemd service
- Environment security
- Backup strategies
