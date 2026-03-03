# Discord Security Bot (Python)

Simple, working Discord security bot with essential features.

## Features

✅ **Anti-Spam** - Detects and mutes spammers (5 messages in 5 seconds)
✅ **Anti-Raid** - Kicks mass joiners (10 joins in 10 seconds)
✅ **Anti-Nuke** - Bans users doing mass bans/deletions (3 actions in 10 seconds)
✅ **Moderation Commands** - Ban, kick, mute, unmute, clear, lock, unlock
✅ **Whitelist System** - Protect trusted users from security measures

## Setup

1. **Install Python 3.8+**
   - Download: https://www.python.org/downloads/

2. **Install Dependencies**
   ```bash
   pip install -r requirements.txt
   ```

3. **Configure Bot**
   - Edit `.env` file
   - Add your Discord bot token

4. **Run Bot**
   ```bash
   python bot.py
   ```

## Commands

- `/ban @user [reason]` - Ban a user
- `/kick @user [reason]` - Kick a user
- `/mute @user <minutes> [reason]` - Timeout a user
- `/unmute @user` - Remove timeout
- `/clear <amount>` - Delete messages
- `/lock` - Lock channel
- `/unlock` - Unlock channel
- `/whitelist @user` - Add to whitelist
- `/status` - Show bot status

## Bot Permissions

Invite with Administrator permission:
```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

## Features Explained

### Anti-Spam
- Tracks messages per user
- Mutes if 5+ messages in 5 seconds
- Auto-deletes spam messages

### Anti-Raid
- Tracks member joins
- Kicks if 10+ joins in 10 seconds
- Protects against raid attacks

### Anti-Nuke
- Tracks bans, kicks, channel deletions
- Bans if 3+ actions in 10 seconds
- Protects against server nuking

### Whitelist
- Whitelisted users bypass all security
- Use `/whitelist @user` to add
- Requires Administrator permission

## Support

This is a simplified version that actually works. For advanced features, consider using established bots like Wick or Beemo.
