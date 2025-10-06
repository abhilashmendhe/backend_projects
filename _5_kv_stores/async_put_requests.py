import aiohttp
import asyncio
import time
import random
import string

url = "http://localhost:8000"

def random_string(length=8):
    return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))

def generate_dummy_data(n=1000):
    data_list = []
    for _ in range(n):
        key = random_string()
        for _ in range(random.randint(100,200)):
            data = {
                "key": key,
                "value": random_string(12),
                "timestamp": int(time.time() * 1_000_000)  # microsecond precision
            }
            data_list.append(data)
    return data_list

async def put_data(session, data):
    async with session.put(url, json=data) as response:
        resp_text = await response.text()
        print(f"PUT {data['key']} -> Status: {response.status}, Response: {resp_text}")

async def main():
    dummy_data = generate_dummy_data(10)  # generate 10 dummy items
    async with aiohttp.ClientSession() as session:
        tasks = [put_data(session, item) for item in dummy_data]
        await asyncio.gather(*tasks)  # run all PUT requests concurrently

# Run the async main function
asyncio.run(main())
