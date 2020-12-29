from dataclasses import dataclass
from typing import List


def get_prompt() -> List[str]:
    with open("2020/day02/input.txt") as f:
        return [line.strip() for line in f.readlines()]


@dataclass
class Entry:

    password: str
    letter: str
    a: int
    b: int

    @staticmethod
    def load(raw: str) -> "Entry":
        password = raw.split(": ")[1].strip()
        parts = raw.split(": ")[0].split(" ")
        letter = parts[1]
        a, b = [int(e) for e in parts[0].split("-")]
        return Entry(
            password=password,
            letter=letter,
            a=a,
            b=b,
        )

    def valid(self) -> bool:
        valid_a = self.password[self.a - 1] == self.letter
        valid_b = self.password[self.b - 1] == self.letter
        return (valid_a or valid_b) and valid_a != valid_b


def main(prompt: List[str]) -> int:
    data = [Entry.load(e) for e in prompt]
    valid_count = sum([e.valid() for e in data])
    return valid_count


if __name__ == "__main__":
    print(main(get_prompt()))
