async function saveConfig() {
    const config = {
        anti_spam: {
            enabled: document.getElementById('anti-spam-enabled').checked,
            max_messages: parseInt(document.getElementById('spam-max-messages').value),
            time_window: parseInt(document.getElementById('spam-time-window').value)
        },
        anti_raid: {
            enabled: document.getElementById('anti-raid-enabled').checked,
            max_joins: parseInt(document.getElementById('raid-max-joins').value),
            time_window: parseInt(document.getElementById('raid-time-window').value)
        },
        anti_nuke: {
            enabled: document.getElementById('anti-nuke-enabled').checked,
            max_actions: parseInt(document.getElementById('nuke-max-actions').value),
            time_window: parseInt(document.getElementById('nuke-time-window').value)
        },
        log_channel: document.getElementById('log-channel').value || null
    };

    try {
        const response = await fetch(`/api/config/${guildId}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(config)
        });

        const result = await response.json();
        
        if (result.success) {
            alert('Configuration saved successfully!');
        } else {
            alert('Failed to save configuration');
        }
    } catch (error) {
        console.error('Error:', error);
        alert('Error saving configuration');
    }
}
