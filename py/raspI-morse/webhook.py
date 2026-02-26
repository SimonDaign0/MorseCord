import requests
import json

def sendDiscordMsg(webhook_url, message_content, username=None, avatar_url=None):
    data = {
        "content": message_content
    }

    if username:
        data["username"] = username
    if avatar_url:
        data["avatar_url"] = avatar_url

    # Headers to specify the content type is JSON
    headers = {
        "Content-Type": "application/json"
    }

    result = requests.post(webhook_url, data=json.dumps(data), headers=headers)

    try:
        result.raise_for_status()
        print(f"Message sent successfully! Status code: {result.status_code}")
    except requests.exceptions.HTTPError as err:
        print(f"Failed to send message: {err}")
        print(result.json())