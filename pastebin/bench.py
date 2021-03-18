"""
To test the collision rate.
"""

from django.utils.crypto import get_random_string

data = {}
all_tries = {4: 0, 5: 0, 6: 0, 7: 0}
collision_tries = {4: 0, 5: 0, 6: 0, 7: 0}

while all_tries[4] < 1e7:
    key_len = 4
    all_tries[key_len] += 1
    key = get_random_string(key_len)
    while data.get(key, '') is None:
        collision_tries[key_len] += 1
        key_len += 1
        all_tries[key_len] += 1
        key = get_random_string(key_len)
    data[key] = None

    if all_tries[4] % 1e6 == 0:
        print(f"{all_tries[4]} tries:")
        for i in range(4, 8):
            collision_rate = collision_tries[i] / all_tries[i] if all_tries[i] else 0
            print(
                f"length {i}: all tries: {all_tries[i]}, "
                f"collision tries: {collision_tries[i]}, "
                f"collision rate: {collision_rate:.2%}"
            )
        print()
