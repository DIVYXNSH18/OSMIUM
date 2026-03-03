import discord
from discord.ext import commands
from discord import app_commands
import asyncio
from datetime import datetime, timedelta
from collections import defaultdict, deque
import os
from dotenv import load_dotenv

load_dotenv()

intents = discord.Intents.all()
bot = commands.Bot(command_prefix="!", intents=intents)

# Anti-spam tracker
spam_tracker = defaultdict(lambda: deque(maxlen=5))
# Anti-raid tracker
raid_tracker = defaultdict(lambda: deque(maxlen=10))
# Anti-nuke tracker
nuke_tracker = defaultdict(lambda: {"bans": deque(maxlen=5), "kicks": deque(maxlen=5), "deletes": deque(maxlen=5)})
# Whitelist
whitelist = set()

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
                except:
                    pass
    
    await bot.process_commands(message)

# ==================== ANTI-RAID ====================
@bot.event
async def on_member_join(member):
    now = datetime.now()
    raid_tracker[member.guild.id].append(now)
    
    # Check if raid (10 joins in 10 seconds)
    if len(raid_tracker[member.guild.id]) >= 10:
        time_diff = (raid_tracker[member.guild.id][-1] - raid_tracker[member.guild.id][0]).total_seconds()
        if time_diff < 10:
            try:
                await member.kick(reason="Raid detected")
                print(f"🚨 Anti-Raid: Kicked {member} - Raid detected")
            except:
                pass

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

if __name__ == "__main__":
    TOKEN = os.getenv("DISCORD_TOKEN")
    if not TOKEN:
        print("❌ DISCORD_TOKEN not found in .env file!")
    else:
        bot.run(TOKEN)
