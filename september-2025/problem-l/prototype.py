# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "colorama",
#     "seaborn",
#     "numpy",
#     "matplotlib",
#     "tqdm",
# ]
# ///
import time
from contextlib import suppress

import colorama
import matplotlib.pyplot as plt
import seaborn as sns
from tqdm import tqdm

GREEN = colorama.Fore.GREEN
RED = colorama.Fore.RED
RESET = colorama.Fore.RESET


def calc_prefixes(s: str, prefixes: list[int] | None = None) -> list[int]:
    target = {c: i for i, c in enumerate("GEOLYMP")}
    # ["G", "GE", "GEO", "GEOL", "GEOLYM", "GEOLYMP"]
    prefixes = [0] * 7 if prefixes is None else prefixes
    for c in s:
        if c not in target:
            continue
        i = target[c]

        if i == 0:
            prefixes[i] += 1
        else:
            prefixes[i] += prefixes[i - 1]
    return prefixes


def search(target: int, current: str = "", next_prefixes: list[int] = [0] * 7) -> str | None:
    parts = ["GEOLYMP"[i:] for i in range(len("GEOLYMP"))]
    candidates = [
        (next := current + part, calc_prefixes(part, next_prefixes.copy())) for part in parts
    ]
    candidates.sort(key=lambda x: x[1][-1], reverse=True)
    for next, next_prefixes in candidates:
        if len(next) > 1000 or next_prefixes[-1] > target:
            continue
        if next_prefixes[-1] == target:
            return next
        if (result := search(target, next, next_prefixes)) is not None:
            return result


def _doit(n: int) -> tuple[int, str | None, float]:
    total_elapsed = 0.0
    s = None
    for _ in range(10):
        start = time.monotonic()
        try:
            s = search(n)
        except RecursionError:
            s = None
        if s is not None:
            assert calc_prefixes(s)[-1] == n
        total_elapsed += time.monotonic() - start

    return (n, s, total_elapsed / 10)


def main() -> None:
    colorama.init()
    while True:
        s = input("> ")
        print(calc_prefixes(s))



if __name__ == "__main__":
    main()
