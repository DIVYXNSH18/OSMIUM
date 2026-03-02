# Discord Security Bot

A production-ready Discord security bot built in Rust with 14 comprehensive security features, advanced moderation system, verification, and web dashboard.

## Features

### 🛡️ Security Systems
- **Anti-Nuke**: Prevents mass bans, kicks, channel/role deletions
- **Anti-Raid**: Detects and stops raid attacks with score-based detection
- **Anti-Spam**: Blocks message spam, duplicate messages, emoji spam
- **Anti-Mention**: Prevents @everyone/@here abuse and mass mentions
- **Anti-Ghost-Ping**: Exposes deleted messages with mentions
- **Anti-Vanity**: Protects server vanity URL from changes
- **Anti-Rename**: Prevents unauthorized server/role/channel renames
- **Anti-Emoji**: Detects and prevents emoji deletions
- **Anti-Invite**: Protects server invites from deletion
- **Beast Mode**: Emergency lockdown mode with auto-deactivation

### 🔐 Verification System
- Button-based verification
- Image CAPTCHA verification
- Configurable verified/unverified roles
- Attempt tracking and logging

### ⚙️ Moderation Commands
- `/ban` - Ban users with reason
- `/kick` - Kick users with reason
- `/mute` - Mute users temporarily
- `/unmute` - Unmute users
- `/clear` - Bulk delete messages
- `/lock` - Lock channels
- `/unlock` - Unlock channels
- `/slowmode` - Set channel slowmode
- `/warn` - Warn users
- `/warnings` - View user warnings
- `/clearwarnings` - Clear user warnings
- `/nickname` - Change user nicknames
- `/timeout` - Timeout users
- `/untimeout` - Remove timeouts

### 📊 Web Dashboard
- Discord OAuth2 authentication
- Real-time configuration management
- Security logs viewer
- Whitelist management
- Statistics and analytics

### 🔍 Security Scanner
- Server security audit
- Permission analysis
- Vulnerability detection
- Security score calculation

## Prerequisites

### Required Software
- **Rust** 1.70+ ([Install](https://rustup.rs/))
- **MongoDB** 4.4+ ([Install](https://www.mongodb.com/try/download/community))
- **Redis** 6.0+ ([Install](https://redis.io/download))
- **Visual Studio Build Tools** (Windows only)
  - Download from [Visual Studio](https://visualstudio.microsoft.com/downloads/)
  - Install "Desktop development with C++" workload

### Discord Bot Setup
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Navigate to "Bot" section and create a bot
4. Enable these Privileged Gateway Intents:
   - Server Members Intent
   - Message Content Intent
   - Presence Intent
5. Copy the bot token
6. Navigate to "OAuth2" section and copy Client ID and Client Secret

## Installation

### 1. Clone Repository
```bash
git clone <repository-url>
cd discord-security-bot
```

### 2. Configure Environment
```bash
cp .env.example .env
```

Edit `.env` with your configuration:
```env
# Discord Bot Configuration
DISCORD_TOKEN=your_bot_token_here
DISCORD_CLIENT_ID=your_client_id_here
DISCORD_CLIENT_SECRET=your_client_secret_here

# Database Configuration
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=discord_security_bot

# Redis Configuration
REDIS_URL=redis://localhost:6379

# Dashboard Configuration
DASHBOARD_HOST=0.0.0.0
DASHBOARD_PORT=3000
DASHBOARD_URL=http://localhost:3000
JWT_SECRET=your_random_jwt_secret_here

# OAuth2 Configuration
OAUTH2_REDIRECT_URI=http://localhost:3000/auth/callback
```

### 3. Start Services

#### MongoDB
```bash
# Linux/macOS
mongod --dbpath /path/to/data

# Windows
mongod --dbpath C:\data\db

# Docker
docker run -d -p 27017:27017 --name mongodb mongo:latest
```

#### Redis
```bash
# Linux/macOS
redis-server

# Windows
redis-server.exe

# Docker
docker run -d -p 6379:6379 --name redis redis:latest
```

### 4. Build and Run

#### Development
```bash
cargo run
```

#### Production
```bash
cargo build --release
./target/release/discord-security-bot
```

## Bot Invitation

Invite the bot with these permissions:
```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

Required Permissions:
- Administrator (or specific permissions: Manage Server, Manage Roles, Manage Channels, Ban Members, Kick Members, Manage Messages, View Audit Log)

## Configuration

### Initial Setup Commands

1. **Enable Security Features**
```
/antinuke enable
/antiraid enable
/antispam enable
/antimention enable
/antivanity enable
```

2. **Configure Verification**
```
/verification setup channel:#verify role:@Verified unverified:@Unverified
/verification enable
```

3. **Add Whitelisted Users/Roles**
```
/whitelist add user:@Admin
/whitelist add role:@Moderator
```

4. **Configure Logging**
```
/config logging channel:#security-logs
```

### Security Thresholds

Default thresholds (configurable in database):
- **Anti-Nuke**: 3 actions in 10 seconds
- **Anti-Raid**: Score > 50 in 60 seconds
- **Anti-Spam**: 5 messages in 5 seconds
- **Anti-Mention**: 5+ mentions in one message

## Web Dashboard

### Access
1. Start the bot
2. Navigate to `http://localhost:3000`
3. Click "Login with Discord"
4. Authorize the application
5. Select your server

### Features
- Toggle security features
- Configure thresholds
- View security logs
- Manage whitelist
- View statistics

## Docker Deployment

### Using Docker Compose
```yaml
version: '3.8'

services:
  bot:
    build: .
    env_file: .env
    depends_on:
      - mongodb
      - redis
    restart: unless-stopped

  mongodb:
    image: mongo:latest
    volumes:
      - mongodb_data:/data/db
    restart: unless-stopped

  redis:
    image: redis:latest
    restart: unless-stopped

volumes:
  mongodb_data:
```

Run with:
```bash
docker-compose up -d
```

## Production Deployment

### Systemd Service (Linux)

Create `/etc/systemd/system/discord-bot.service`:
```ini
[Unit]
Description=Discord Security Bot
After=network.target mongodb.service redis.service

[Service]
Type=simple
User=botuser
WorkingDirectory=/opt/discord-security-bot
ExecStart=/opt/discord-security-bot/target/release/discord-security-bot
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable discord-bot
sudo systemctl start discord-bot
sudo systemctl status discord-bot
```

### Environment Variables for Production
```env
RUST_LOG=info
DASHBOARD_URL=https://yourdomain.com
OAUTH2_REDIRECT_URI=https://yourdomain.com/auth/callback
```

### Reverse Proxy (Nginx)
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

## Monitoring

### Logs
```bash
# View logs
tail -f logs/bot.log

# Systemd logs
journalctl -u discord-bot -f
```

### Health Check
```bash
curl http://localhost:3000/health
```

## Troubleshooting

### Bot Not Starting
- Verify `.env` configuration
- Check MongoDB/Redis are running
- Ensure bot token is valid
- Check privileged intents are enabled

### Commands Not Working
- Verify bot has Administrator permission
- Check bot role is above managed roles
- Ensure slash commands are registered (wait 1 hour or restart bot)

### Compilation Errors (Windows)
- Install Visual Studio Build Tools with C++ workload
- Restart terminal after installation
- Run `cargo clean` then `cargo build`

### Database Connection Failed
- Verify MongoDB is running: `mongosh`
- Check MONGODB_URI in `.env`
- Ensure firewall allows port 27017

### Redis Connection Failed
- Verify Redis is running: `redis-cli ping`
- Check REDIS_URL in `.env`
- Ensure firewall allows port 6379

## Security Best Practices

1. **Keep Token Secret**: Never commit `.env` to version control
2. **Use Strong JWT Secret**: Generate with `openssl rand -hex 32`
3. **Enable 2FA**: Require 2FA for server administrators
4. **Regular Updates**: Keep dependencies updated with `cargo update`
5. **Monitor Logs**: Regularly check security logs for suspicious activity
6. **Backup Database**: Schedule regular MongoDB backups
7. **Whitelist Trusted Users**: Add trusted admins to whitelist

## Performance Tuning

### MongoDB Indexes
```javascript
db.guild_configs.createIndex({ guild_id: 1 })
db.moderation_logs.createIndex({ guild_id: 1, timestamp: -1 })
db.warnings.createIndex({ guild_id: 1, user_id: 1 })
```

### Redis Memory
```bash
# Set max memory (e.g., 256MB)
redis-cli CONFIG SET maxmemory 256mb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

### Bot Optimization
- Adjust `RUST_LOG=warn` for production (less verbose)
- Use `cargo build --release` for optimized binary
- Enable MongoDB connection pooling (already configured)

## API Documentation

### REST API Endpoints

#### Authentication
- `GET /auth/login` - Initiate OAuth2 flow
- `GET /auth/callback` - OAuth2 callback
- `POST /auth/logout` - Logout user

#### Guild Management
- `GET /api/guilds` - List user's guilds
- `GET /api/guilds/:id` - Get guild details
- `GET /api/guilds/:id/config` - Get guild configuration
- `PUT /api/guilds/:id/config` - Update guild configuration

#### Whitelist
- `GET /api/guilds/:id/whitelist` - Get whitelist
- `POST /api/guilds/:id/whitelist` - Add to whitelist
- `DELETE /api/guilds/:id/whitelist/:entry_id` - Remove from whitelist

#### Logs
- `GET /api/guilds/:id/logs` - Get moderation logs

## Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -am 'Add feature'`
4. Push to branch: `git push origin feature-name`
5. Submit pull request

## License

This project is licensed under the MIT License - see LICENSE file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/discord-security-bot/issues)
- **Discord**: [Support Server](https://discord.gg/your-invite)
- **Documentation**: [Wiki](https://github.com/yourusername/discord-security-bot/wiki)

## Acknowledgments

- Built with [Serenity](https://github.com/serenity-rs/serenity)
- Command framework: [Poise](https://github.com/serenity-rs/poise)
- Web framework: [Axum](https://github.com/tokio-rs/axum)
- Database: [MongoDB](https://www.mongodb.com/)
- Cache: [Redis](https://redis.io/)

---

**Note**: This bot requires proper configuration and monitoring. Always test in a development server before deploying to production.
