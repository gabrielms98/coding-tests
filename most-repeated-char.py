from typing import Dict


def solution(s: str) -> str:
    compute: Dict[str, int] = {}

    for c in s:
        if c.isnumeric():
            continue

        if c not in compute:
            compute[c] = 0
        compute[c] += 1

    return max(compute, key=compute.get)


if __name__ == "__main__":
    print(solution("abcddefda131312313111"))
    print(solution("AA0AB0BB0ccc0aa0aw00woOBBBw123123"))
