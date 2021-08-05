"""
To test the collision rate.
"""

import os
import random


def get_random_string(
    length, allowed_chars=('abcdefghijklmnopqrstuvwxyz' 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789')
):
    """
    This function is similar to django.utils.crypto.get_random_string,
    only replace `secrets` to `random`.
    """
    r = ''.join(random.choice(allowed_chars) for i in range(length))
    return r


print(f"This process has PID: {os.getpid()}")

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
