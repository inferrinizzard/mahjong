from typing import Dict


def take(d: Dict[str, int], key: str):
    if key not in d:
        return
    if d[key] > 1:
        d[key] -= 1
    else:
        del d[key]

    return key
