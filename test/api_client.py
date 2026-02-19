# api_client.py
import os
import requests

def call_api(prompt):
    # 獲取 API 密鑰
    api_key = os.environ.get('ANTHROPIC_AUTH_TOKEN')
    base_url = os.environ.get('ANTHROPIC_BASE_URL', 'https://api.anthropic.com')

    if not api_key:
        raise ValueError(
            '❌ 密鑰未加載！\n'
            '請執行: eval "$(keychain-cli load --format export)"\n'
            '或使用: ~/start-claude.sh python api_client.py'
        )

    headers = {
        'x-api-key': api_key,
        'content-type': 'application/json'
    }

    payload = {
        'model': 'claude-haiku-4-5-20251001',
        'max_tokens': 1024,
        'messages': [{'role': 'user', 'content': prompt}]
    }

    api_url = f"{base_url}/v1/messages"
    print(f"📡 使用 API 端點: {api_url}")

    response = requests.post(
        api_url,
        headers=headers,
        json=payload
    )

    return response.json()

if __name__ == '__main__':
    try:
        result = call_api('What is 2+2?')
        print('✅ Success:', result)
    except Exception as e:
        print('❌ Error:', str(e))
