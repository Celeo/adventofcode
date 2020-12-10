from dataclasses import dataclass


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
        return self.a <= self.password.count(self.letter) <= self.b


def main():
    with open("2020/day2/input.txt") as f:
        data = [Entry.load(line) for line in f.readlines()]
    valid_count = sum([e.valid() for e in data])
    print(valid_count)


if __name__ == "__main__":
    main()
