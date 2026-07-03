from concurrent.futures import ThreadPoolExecutor, as_completed
import random
import requests
import argparse

session = requests.Session()

def create_user(base_url, i):
    user_id = f"user_{i:04d}"

    user = {
        "username": user_id,
        "email": f"{user_id}@email.com",
    }

    r = session.post(f"{base_url}/user", json=user)
    r.raise_for_status()

    ios_device = {
        "user_id": user_id,
        "token": f"ios_tok_{user_id}",
        "platform": "ios",
    }

    r = session.post(f"{base_url}/devices", json=ios_device)
    r.raise_for_status()

    if random.random() > 0.5:
        android_device = {
            "user_id": user_id,
            "token": f"android_tok_{user_id}",
            "platform": "android",
        }

        r = session.post(f"{base_url}/devices", json=android_device)
        r.raise_for_status()

    return user_id
    
if __name__ == "__main__":
    p = argparse.ArgumentParser(description="A script to create users.")
    p.add_argument("--target", default="http://localhost:8080",
                   help="Your service base URL (default: http://localhost:8080)")
    p.add_argument("--users", type=int, default=500)
    p.add_argument("--workers", type=int, default=4)
    args = p.parse_args()
    
    with ThreadPoolExecutor(max_workers=args.workers) as executor:
        futures = [
            executor.submit(create_user, args.target, i)
            for i in range(1, args.users + 1)
        ]

        for future in as_completed(futures):
            try:
                print("Created:", future.result())
            except Exception as e:
                print("Failed:", e)

    print("All users were added")