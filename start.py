import threading
import subprocess
import sys

def run_bot():
    subprocess.run([sys.executable, 'bot.py'])

def run_web():
    subprocess.run([sys.executable, 'web_dashboard.py'])

if __name__ == '__main__':
    print("Starting OSMIUM Bot and Web Dashboard...")
    
    bot_thread = threading.Thread(target=run_bot, daemon=True)
    web_thread = threading.Thread(target=run_web, daemon=True)
    
    bot_thread.start()
    web_thread.start()
    
    bot_thread.join()
    web_thread.join()
