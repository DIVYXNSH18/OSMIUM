import discord
from discord.ext import commands
from discord import app_commands
import asyncio
from datetime import datetime, timedelta
from collections import defaultdict, deque
import os
from dotenv import load_dotenv
from flask import Flask
from threading import Thread

load_dotenv()

# Flask web server to keep Render alive
app = Flask('')

@app.route('/')
def home():
    return "Bot is running!"

def run():
    app.run(host='0.0.0.0', port=8080)

def keep_alive():
    t = Thread(target=run)
    t.start()

intents = discord.Intents.all()
bot = commands.Bot(command_prefix="!", intents=intents)

# Anti-spam tracker
spam_tracker = defaultdict(lambda: deque(maxlen=5))
# Anti-raid tracker
raid_tracker = defaultdict(lambda: deque(maxlen=10))
# Anti-nuke tracker
nuke_tracker = defaultdict(lambda: {"bans": deque(maxlen=5), "kicks": deque(maxlen=5), "deletes": deque(maxlen=5), "roles": deque(maxlen=5)})
# Whitelist
whitelist = set()
# Log channels storage
log_channels = {}
# Server lockdown status
lockdown_status = {}
# Minimum account age (days)
MIN_ACCOUNT_AGE = 7

@bot.event
async def on_ready():
    print(f'✅ Bot is ready! Logged in as {bot.user}')
    try:
        synced = await bot.tree.sync()
        print(f'✅ Synced {len(synced)} commands')
    except Exception as e:
        print(f'❌ Failed to sync commands: {e}')

# ==================== ANTI-SPAM ====================
@bot.event
async def on_message(message):
    if message.author.bot or message.author.id in whitelist:
        return
    
    if message.guild:
        user_id = message.author.id
        now = datetime.now()
        
        # Track messages
        spam_tracker[user_id].append(now)
        
        # Check if spam (5 messages in 5 seconds)
        if len(spam_tracker[user_id]) >= 5:
            time_diff = (spam_tracker[user_id][-1] - spam_tracker[user_id][0]).total_seconds()
            if time_diff < 5:
                try:
                    await message.delete()
                    await message.channel.send(f"⚠️ {message.author.mention} Stop spamming!", delete_after=5)
                    await message.author.timeout(timedelta(minutes=5), reason="Spam detected")
                    print(f"🚨 Anti-Spam: Muted {message.author} for spamming")
                    
                    # Log to security channel
                    if message.guild.id in log_channels and 'security' in log_channels[message.guild.id]:
                        channel = bot.get_channel(log_channels[message.guild.id]['security'])
                        if channel:
                            embed = discord.Embed(title="🚨 Anti-Spam Triggered", color=0xED4245, timestamp=datetime.now())
                            embed.add_field(name="User", value=f"{message.author.mention} ({message.author.id})", inline=False)
                            embed.add_field(name="Channel", value=message.channel.mention, inline=True)
                            embed.add_field(name="Action", value="Muted for 5 minutes", inline=True)
                            await channel.send(embed=embed)
                except:
                    pass
    
    await bot.process_commands(message)

# ==================== ANTI-RAID ====================
@bot.event
async def on_member_join(member):
    now = datetime.now()
    raid_tracker[member.guild.id].append(now)
    
    # Check account age
    account_age = (now - member.created_at.replace(tzinfo=None)).days
    is_new_account = account_age < MIN_ACCOUNT_AGE
    
    # Log to join channel
    if member.guild.id in log_channels and 'join' in log_channels[member.guild.id]:
        channel = bot.get_channel(log_channels[member.guild.id]['join'])
        if channel:
            embed = discord.Embed(
                title="👋 Member Joined", 
                color=0xED4245 if is_new_account else 0x57F287, 
                timestamp=datetime.now()
            )
            embed.set_thumbnail(url=member.display_avatar.url)
            embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
            embed.add_field(name="Account Age", value=f"{account_age} days", inline=True)
            if is_new_account:
                embed.add_field(name="⚠️ Warning", value="New account detected!", inline=True)
            embed.set_footer(text=f"Total Members: {member.guild.member_count}")
            await channel.send(embed=embed)
    
    # Auto-kick new accounts during raid
    if is_new_account and len(raid_tracker[member.guild.id]) >= 5:
        try:
            await member.kick(reason="New account during raid")
            if member.guild.id in log_channels and 'security' in log_channels[member.guild.id]:
                channel = bot.get_channel(log_channels[member.guild.id]['security'])
                if channel:
                    embed = discord.Embed(title="🚨 Auto-Kick: New Account", color=0xED4245, timestamp=datetime.now())
                    embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
                    embed.add_field(name="Reason", value=f"Account age: {account_age} days (min: {MIN_ACCOUNT_AGE})", inline=False)
                    await channel.send(embed=embed)
            return
        except:
            pass
    
    # Check if raid (10 joins in 10 seconds)
    if len(raid_tracker[member.guild.id]) >= 10:
        time_diff = (raid_tracker[member.guild.id][-1] - raid_tracker[member.guild.id][0]).total_seconds()
        if time_diff < 10:
            try:
                await member.kick(reason="Raid detected")
                print(f"🚨 Anti-Raid: Kicked {member} - Raid detected")
                
                # Log to security channel
                if member.guild.id in log_channels and 'security' in log_channels[member.guild.id]:
                    channel = bot.get_channel(log_channels[member.guild.id]['security'])
                    if channel:
                        embed = discord.Embed(title="🚨 Anti-Raid Triggered", color=0xED4245, timestamp=datetime.now())
                        embed.add_field(name="User Kicked", value=f"{member.mention} ({member.id})", inline=False)
                        embed.add_field(name="Reason", value="Mass join detected (10+ joins in 10s)", inline=False)
                        await channel.send(embed=embed)
            except:
                pass

@bot.event
async def on_member_remove(member):
    # Log to join channel
    if member.guild.id in log_channels and 'join' in log_channels[member.guild.id]:
        channel = bot.get_channel(log_channels[member.guild.id]['join'])
        if channel:
            embed = discord.Embed(title="👋 Member Left", color=0xED4245, timestamp=datetime.now())
            embed.set_thumbnail(url=member.display_avatar.url)
            embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
            embed.set_footer(text=f"Total Members: {member.guild.member_count}")
            await channel.send(embed=embed)

# ==================== ANTI-NUKE ====================
@bot.event
async def on_member_ban(guild, user):
    async for entry in guild.audit_logs(limit=1, action=discord.AuditLogAction.ban):
        if entry.user.id not in whitelist:
            now = datetime.now()
            nuke_tracker[entry.user.id]["bans"].append(now)
            
            if len(nuke_tracker[entry.user.id]["bans"]) >= 3:
                time_diff = (nuke_tracker[entry.user.id]["bans"][-1] - nuke_tracker[entry.user.id]["bans"][0]).total_seconds()
                if time_diff < 10:
                    try:
                        await guild.ban(entry.user, reason="Anti-Nuke: Mass banning detected")
                        print(f"🚨 Anti-Nuke: Banned {entry.user} for mass banning")
                    except:
                        pass

@bot.event
async def on_guild_channel_delete(channel):
    async for entry in channel.guild.audit_logs(limit=1, action=discord.AuditLogAction.channel_delete):
        if entry.user.id not in whitelist:
            now = datetime.now()
            nuke_tracker[entry.user.id]["deletes"].append(now)
            
            if len(nuke_tracker[entry.user.id]["deletes"]) >= 3:
                time_diff = (nuke_tracker[entry.user.id]["deletes"][-1] - nuke_tracker[entry.user.id]["deletes"][0]).total_seconds()
                if time_diff < 10:
                    try:
                        await channel.guild.ban(entry.user, reason="Anti-Nuke: Mass channel deletion")
                        print(f"🚨 Anti-Nuke: Banned {entry.user} for mass channel deletion")
                    except:
                        pass

# ==================== MODERATION COMMANDS ====================
@bot.tree.command(name="ban", description="Ban a user")
@app_commands.describe(member="User to ban", reason="Reason for ban")
async def ban(interaction: discord.Interaction, member: discord.Member, reason: str = "No reason provided"):
    if not interaction.user.guild_permissions.ban_members:
        await interaction.response.send_message("❌ You don't have permission to ban members!", ephemeral=True)
        return
    
    try:
        await member.ban(reason=reason)
        await interaction.response.send_message(f"✅ Banned {member.mention} | Reason: {reason}")
        print(f"🔨 {interaction.user} banned {member} - Reason: {reason}")
        
        # Log to mod channel
        if interaction.guild.id in log_channels and 'mod' in log_channels[interaction.guild.id]:
            channel = bot.get_channel(log_channels[interaction.guild.id]['mod'])
            if channel:
                embed = discord.Embed(title="🔨 Member Banned", color=0xED4245, timestamp=datetime.now())
                embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
                embed.add_field(name="Moderator", value=interaction.user.mention, inline=True)
                embed.add_field(name="Reason", value=reason, inline=True)
                await channel.send(embed=embed)
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to ban: {e}", ephemeral=True)

@bot.tree.command(name="kick", description="Kick a user")
@app_commands.describe(member="User to kick", reason="Reason for kick")
async def kick(interaction: discord.Interaction, member: discord.Member, reason: str = "No reason provided"):
    if not interaction.user.guild_permissions.kick_members:
        await interaction.response.send_message("❌ You don't have permission to kick members!", ephemeral=True)
        return
    
    try:
        await member.kick(reason=reason)
        await interaction.response.send_message(f"✅ Kicked {member.mention} | Reason: {reason}")
        print(f"👢 {interaction.user} kicked {member} - Reason: {reason}")
        
        # Log to mod channel
        if interaction.guild.id in log_channels and 'mod' in log_channels[interaction.guild.id]:
            channel = bot.get_channel(log_channels[interaction.guild.id]['mod'])
            if channel:
                embed = discord.Embed(title="👢 Member Kicked", color=0xFFA500, timestamp=datetime.now())
                embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
                embed.add_field(name="Moderator", value=interaction.user.mention, inline=True)
                embed.add_field(name="Reason", value=reason, inline=True)
                await channel.send(embed=embed)
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to kick: {e}", ephemeral=True)

@bot.tree.command(name="mute", description="Timeout a user")
@app_commands.describe(member="User to mute", minutes="Duration in minutes", reason="Reason for mute")
async def mute(interaction: discord.Interaction, member: discord.Member, minutes: int, reason: str = "No reason provided"):
    if not interaction.user.guild_permissions.moderate_members:
        await interaction.response.send_message("❌ You don't have permission to timeout members!", ephemeral=True)
        return
    
    try:
        await member.timeout(timedelta(minutes=minutes), reason=reason)
        await interaction.response.send_message(f"✅ Muted {member.mention} for {minutes} minutes | Reason: {reason}")
        print(f"🔇 {interaction.user} muted {member} for {minutes}m - Reason: {reason}")
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to mute: {e}", ephemeral=True)

@bot.tree.command(name="unmute", description="Remove timeout from a user")
@app_commands.describe(member="User to unmute")
async def unmute(interaction: discord.Interaction, member: discord.Member):
    if not interaction.user.guild_permissions.moderate_members:
        await interaction.response.send_message("❌ You don't have permission to timeout members!", ephemeral=True)
        return
    
    try:
        await member.timeout(None)
        await interaction.response.send_message(f"✅ Unmuted {member.mention}")
        print(f"🔊 {interaction.user} unmuted {member}")
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to unmute: {e}", ephemeral=True)

@bot.tree.command(name="clear", description="Delete messages")
@app_commands.describe(amount="Number of messages to delete")
async def clear(interaction: discord.Interaction, amount: int):
    if not interaction.user.guild_permissions.manage_messages:
        await interaction.response.send_message("❌ You don't have permission to manage messages!", ephemeral=True)
        return
    
    try:
        await interaction.response.defer(ephemeral=True)
        deleted = await interaction.channel.purge(limit=amount)
        await interaction.followup.send(f"✅ Deleted {len(deleted)} messages", ephemeral=True)
        print(f"🗑️ {interaction.user} cleared {len(deleted)} messages")
    except Exception as e:
        await interaction.followup.send(f"❌ Failed to clear: {e}", ephemeral=True)

@bot.tree.command(name="lock", description="Lock a channel")
async def lock(interaction: discord.Interaction):
    if not interaction.user.guild_permissions.manage_channels:
        await interaction.response.send_message("❌ You don't have permission to manage channels!", ephemeral=True)
        return
    
    try:
        await interaction.channel.set_permissions(interaction.guild.default_role, send_messages=False)
        await interaction.response.send_message("🔒 Channel locked!")
        print(f"🔒 {interaction.user} locked {interaction.channel}")
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to lock: {e}", ephemeral=True)

@bot.tree.command(name="unlock", description="Unlock a channel")
async def unlock(interaction: discord.Interaction):
    if not interaction.user.guild_permissions.manage_channels:
        await interaction.response.send_message("❌ You don't have permission to manage channels!", ephemeral=True)
        return
    
    try:
        await interaction.channel.set_permissions(interaction.guild.default_role, send_messages=True)
        await interaction.response.send_message("🔓 Channel unlocked!")
        print(f"🔓 {interaction.user} unlocked {interaction.channel}")
    except Exception as e:
        await interaction.response.send_message(f"❌ Failed to unlock: {e}", ephemeral=True)

@bot.tree.command(name="whitelist", description="Add user to whitelist")
@app_commands.describe(member="User to whitelist")
async def add_whitelist(interaction: discord.Interaction, member: discord.Member):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    whitelist.add(member.id)
    await interaction.response.send_message(f"✅ Added {member.mention} to whitelist")
    print(f"✅ {interaction.user} whitelisted {member}")

@bot.tree.command(name="status", description="Show bot status")
async def status(interaction: discord.Interaction):
    embed = discord.Embed(title="🛡️ Security Bot Status", color=0x00ff00)
    embed.add_field(name="Servers", value=len(bot.guilds), inline=True)
    embed.add_field(name="Users", value=len(bot.users), inline=True)
    embed.add_field(name="Whitelisted", value=len(whitelist), inline=True)
    embed.add_field(name="Features", value="✅ Anti-Spam\n✅ Anti-Raid\n✅ Anti-Nuke\n✅ Moderation", inline=False)
    await interaction.response.send_message(embed=embed)

@bot.tree.command(name="enableverification", description="Setup verification system")
@app_commands.describe(channel="Channel to send verification message")
async def enableverification(interaction: discord.Interaction, channel: discord.TextChannel):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    # Create verified role if it doesn't exist
    verified_role = discord.utils.get(interaction.guild.roles, name="Verified")
    if not verified_role:
        verified_role = await interaction.guild.create_role(
            name="Verified",
            color=discord.Color.green(),
            reason="Verification system"
        )
    
    # Create verification embed
    embed = discord.Embed(
        title="Server Verification",
        description="This server requires you to verify yourself to get access to other channels. You can simply verify by clicking on the verify button.",
        color=0x5865F2
    )
    embed.set_image(url="https://i.imgur.com/verification_banner.png")  # You can replace with custom image
    
    # Create verify button
    class VerifyButton(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)
        
        @discord.ui.button(label="Verify", style=discord.ButtonStyle.primary, custom_id="verify_button", emoji="✅")
        async def verify(self, interaction: discord.Interaction, button: discord.ui.Button):
            member = interaction.user
            verified_role = discord.utils.get(interaction.guild.roles, name="Verified")
            
            if verified_role in member.roles:
                await interaction.response.send_message("✅ You are already verified!", ephemeral=True)
                return
            
            try:
                await member.add_roles(verified_role)
                await interaction.response.send_message("✅ You have been verified! Welcome to the server!", ephemeral=True)
                
                # Log to mod channel
                if interaction.guild.id in log_channels and 'join' in log_channels[interaction.guild.id]:
                    log_channel = bot.get_channel(log_channels[interaction.guild.id]['join'])
                    if log_channel:
                        log_embed = discord.Embed(title="✅ Member Verified", color=0x57F287, timestamp=datetime.now())
                        log_embed.add_field(name="User", value=f"{member.mention} ({member.id})", inline=False)
                        await log_channel.send(embed=log_embed)
            except Exception as e:
                await interaction.response.send_message(f"❌ Failed to verify: {e}", ephemeral=True)
    
    # Send verification message
    await channel.send(embed=embed, view=VerifyButton())
    await interaction.response.send_message(f"✅ Verification system enabled in {channel.mention}\n\n**Setup Instructions:**\n1. Remove view permissions from @everyone for other channels\n2. Give view permissions to {verified_role.mention} for other channels\n3. Members will get the role after clicking Verify", ephemeral=True)
    
    print(f"✅ {interaction.user} enabled verification in {interaction.guild.name}")

@bot.tree.command(name="logging", description="Setup logging channels")
async def logging(interaction: discord.Interaction):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    await interaction.response.defer()
    
    try:
        guild = interaction.guild
        
        # Check if logging channels already exist
        if guild.id in log_channels:
            existing_channels = []
            for channel_type, channel_id in log_channels[guild.id].items():
                channel = bot.get_channel(channel_id)
                if channel:
                    existing_channels.append(f"{channel_type}: {channel.mention}")
            
            if existing_channels:
                embed = discord.Embed(
                    title="⚠️ Logging Channels Already Exist",
                    description="Logging channels are already set up for this server.",
                    color=0xFFA500
                )
                embed.add_field(name="Existing Channels", value="\n".join(existing_channels), inline=False)
                embed.set_footer(text="Delete the channels manually if you want to recreate them")
                await interaction.followup.send(embed=embed, ephemeral=True)
                return
        
        # Create category for logs
        overwrites = {
            guild.default_role: discord.PermissionOverwrite(view_channel=False),
            guild.me: discord.PermissionOverwrite(view_channel=True, send_messages=True)
        }
        
        # Add permission for administrators
        for role in guild.roles:
            if role.permissions.administrator:
                overwrites[role] = discord.PermissionOverwrite(view_channel=True, send_messages=False)
        
        category = await guild.create_category("📊 Security Logs", overwrites=overwrites)
        
        # Create log channels
        mod_logs = await guild.create_text_channel("mod-logs", category=category, topic="Moderation actions log")
        join_logs = await guild.create_text_channel("join-logs", category=category, topic="Member join/leave log")
        message_logs = await guild.create_text_channel("message-logs", category=category, topic="Deleted/edited messages log")
        security_logs = await guild.create_text_channel("security-logs", category=category, topic="Anti-spam/raid/nuke alerts")
        
        # Store channel IDs
        log_channels[guild.id] = {
            'mod': mod_logs.id,
            'join': join_logs.id,
            'message': message_logs.id,
            'security': security_logs.id
        }
        
        embed = discord.Embed(
            title="✅ Logging Channels Created",
            description="Security logging channels have been set up!",
            color=0xE89A7C
        )
        embed.add_field(name="📋 Mod Logs", value=mod_logs.mention, inline=True)
        embed.add_field(name="👋 Join Logs", value=join_logs.mention, inline=True)
        embed.add_field(name="💬 Message Logs", value=message_logs.mention, inline=True)
        embed.add_field(name="🛡️ Security Logs", value=security_logs.mention, inline=True)
        embed.set_footer(text="Only administrators can view these channels")
        
        await interaction.followup.send(embed=embed)
        print(f"📊 {interaction.user} created logging channels in {guild.name}")
        
    except Exception as e:
        await interaction.followup.send(f"❌ Failed to create logging channels: {e}", ephemeral=True)

if __name__ == "__main__":
    TOKEN = os.getenv("DISCORD_TOKEN")
    if not TOKEN:
        print("❌ DISCORD_TOKEN not found in .env file!")
    else:
        keep_alive()  # Start web server
        bot.run(TOKEN)


# ==================== ROLE MONITORING ====================
@bot.event
async def on_guild_role_delete(role):
    async for entry in role.guild.audit_logs(limit=1, action=discord.AuditLogAction.role_delete):
        if entry.user.id not in whitelist:
            now = datetime.now()
            nuke_tracker[entry.user.id]["roles"].append(now)
            
            if len(nuke_tracker[entry.user.id]["roles"]) >= 3:
                time_diff = (nuke_tracker[entry.user.id]["roles"][-1] - nuke_tracker[entry.user.id]["roles"][0]).total_seconds()
                if time_diff < 10:
                    try:
                        await role.guild.ban(entry.user, reason="Anti-Nuke: Mass role deletion")
                        
                        if role.guild.id in log_channels and 'security' in log_channels[role.guild.id]:
                            channel = bot.get_channel(log_channels[role.guild.id]['security'])
                            if channel:
                                embed = discord.Embed(title="🚨 NUKE ATTEMPT BLOCKED", color=0xED4245, timestamp=datetime.now())
                                embed.add_field(name="Attacker", value=f"{entry.user.mention} ({entry.user.id})", inline=False)
                                embed.add_field(name="Action", value="Mass role deletion detected", inline=False)
                                embed.add_field(name="Response", value="User banned automatically", inline=False)
                                await channel.send(embed=embed)
                    except:
                        pass

@bot.event
async def on_guild_role_update(before, after):
    # Detect dangerous permission grants
    dangerous_perms = ['administrator', 'manage_guild', 'manage_roles', 'manage_channels', 'ban_members', 'kick_members']
    
    before_perms = {perm: value for perm, value in before.permissions}
    after_perms = {perm: value for perm, value in after.permissions}
    
    granted_perms = []
    for perm in dangerous_perms:
        if not before_perms.get(perm, False) and after_perms.get(perm, False):
            granted_perms.append(perm)
    
    if granted_perms:
        async for entry in after.guild.audit_logs(limit=1, action=discord.AuditLogAction.role_update):
            if entry.user.id not in whitelist and after.guild.id in log_channels and 'security' in log_channels[after.guild.id]:
                channel = bot.get_channel(log_channels[after.guild.id]['security'])
                if channel:
                    embed = discord.Embed(title="⚠️ Dangerous Permission Granted", color=0xFFA500, timestamp=datetime.now())
                    embed.add_field(name="Role", value=after.mention, inline=False)
                    embed.add_field(name="Modified By", value=entry.user.mention, inline=True)
                    embed.add_field(name="Permissions Granted", value=", ".join(granted_perms), inline=False)
                    await channel.send(embed=embed)

# ==================== ADVANCED COMMANDS ====================
@bot.tree.command(name="lockdown", description="Emergency server lockdown")
async def lockdown(interaction: discord.Interaction):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    await interaction.response.defer()
    
    try:
        guild = interaction.guild
        lockdown_status[guild.id] = True
        
        # Lock all text channels
        locked_count = 0
        for channel in guild.text_channels:
            try:
                await channel.set_permissions(guild.default_role, send_messages=False)
                locked_count += 1
            except:
                pass
        
        embed = discord.Embed(
            title="🔒 SERVER LOCKDOWN ACTIVATED",
            description=f"All channels have been locked. Only administrators can send messages.",
            color=0xED4245,
            timestamp=datetime.now()
        )
        embed.add_field(name="Locked Channels", value=str(locked_count), inline=True)
        embed.add_field(name="Activated By", value=interaction.user.mention, inline=True)
        
        await interaction.followup.send(embed=embed)
        
        if guild.id in log_channels and 'security' in log_channels[guild.id]:
            channel = bot.get_channel(log_channels[guild.id]['security'])
            if channel:
                await channel.send(embed=embed)
        
        print(f"🔒 {interaction.user} activated lockdown in {guild.name}")
    except Exception as e:
        await interaction.followup.send(f"❌ Failed to lockdown: {e}", ephemeral=True)

@bot.tree.command(name="unlockdown", description="Remove server lockdown")
async def unlockdown(interaction: discord.Interaction):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    await interaction.response.defer()
    
    try:
        guild = interaction.guild
        lockdown_status[guild.id] = False
        
        # Unlock all text channels
        unlocked_count = 0
        for channel in guild.text_channels:
            try:
                await channel.set_permissions(guild.default_role, send_messages=True)
                unlocked_count += 1
            except:
                pass
        
        embed = discord.Embed(
            title="🔓 SERVER LOCKDOWN DEACTIVATED",
            description=f"All channels have been unlocked.",
            color=0x57F287,
            timestamp=datetime.now()
        )
        embed.add_field(name="Unlocked Channels", value=str(unlocked_count), inline=True)
        embed.add_field(name="Deactivated By", value=interaction.user.mention, inline=True)
        
        await interaction.followup.send(embed=embed)
        
        if guild.id in log_channels and 'security' in log_channels[guild.id]:
            channel = bot.get_channel(log_channels[guild.id]['security'])
            if channel:
                await channel.send(embed=embed)
        
        print(f"🔓 {interaction.user} deactivated lockdown in {guild.name}")
    except Exception as e:
        await interaction.followup.send(f"❌ Failed to unlock: {e}", ephemeral=True)

@bot.tree.command(name="accountage", description="Set minimum account age requirement")
@app_commands.describe(days="Minimum account age in days")
async def accountage(interaction: discord.Interaction, days: int):
    if not interaction.user.guild_permissions.administrator:
        await interaction.response.send_message("❌ You need Administrator permission!", ephemeral=True)
        return
    
    global MIN_ACCOUNT_AGE
    MIN_ACCOUNT_AGE = days
    
    await interaction.response.send_message(f"✅ Minimum account age set to **{days} days**\\nNew accounts will be auto-kicked during raids.", ephemeral=True)
    print(f"⚙️ {interaction.user} set min account age to {days} days")

@bot.tree.command(name="security", description="View security status and statistics")
async def security(interaction: discord.Interaction):
    guild = interaction.guild
    
    embed = discord.Embed(
        title="🛡️ Security Status",
        description=f"Security overview for **{guild.name}**",
        color=0xE89A7C,
        timestamp=datetime.now()
    )
    
    # Lockdown status
    is_locked = lockdown_status.get(guild.id, False)
    embed.add_field(name="Lockdown", value="🔒 Active" if is_locked else "🔓 Inactive", inline=True)
    
    # Whitelist count
    embed.add_field(name="Whitelisted Users", value=str(len(whitelist)), inline=True)
    
    # Account age requirement
    embed.add_field(name="Min Account Age", value=f"{MIN_ACCOUNT_AGE} days", inline=True)
    
    # Logging status
    has_logging = guild.id in log_channels
    embed.add_field(name="Logging", value="✅ Enabled" if has_logging else "❌ Disabled", inline=True)
    
    # Recent activity
    recent_spam = sum(1 for tracker in spam_tracker.values() if len(tracker) > 0)
    embed.add_field(name="Active Spam Trackers", value=str(recent_spam), inline=True)
    
    recent_raids = len(raid_tracker.get(guild.id, []))
    embed.add_field(name="Recent Joins", value=str(recent_raids), inline=True)
    
    embed.set_footer(text="Use /lockdown for emergency protection")
    
    await interaction.response.send_message(embed=embed)
