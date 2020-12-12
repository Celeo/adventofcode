from dataclasses import dataclass
import re
from typing import Optional


@dataclass(init=False)
class PassportInfo:

    byr: Optional[str]
    iyr: Optional[str]
    eyr: Optional[str]
    hgt: Optional[str]
    hcl: Optional[str]
    ecl: Optional[str]
    pid: Optional[str]
    cid: Optional[str]

    def __init__(self) -> None:
        self.byr = None
        self.iyr = None
        self.eyr = None
        self.hgt = None
        self.hcl = None
        self.ecl = None
        self.pid = None
        self.cid = None

    @staticmethod
    def from_buffer(text: str) -> "PassportInfo":
        new_info = PassportInfo()
        for part in text.split():
            prefix, value = part.split(":")
            if prefix == "byr":
                new_info.byr = value
            elif prefix == "iyr":
                new_info.iyr = value
            elif prefix == "eyr":
                new_info.eyr = value
            elif prefix == "hgt":
                new_info.hgt = value
            elif prefix == "hcl":
                new_info.hcl = value
            elif prefix == "ecl":
                new_info.ecl = value
            elif prefix == "pid":
                new_info.pid = value
            elif prefix == "cid":
                new_info.cid = value
            else:
                raise ValueError(f"Unknown prefix: {prefix}")
        return new_info

    def valid(self) -> bool:
        hgt_match = self.hgt and re.match(r"^\d+(in|cm)$", self.hgt) is not None
        if hgt_match:
            if "in" in self.hgt:
                hgt_match = 59 <= int(self.hgt[:-2]) <= 76
            else:
                hgt_match = 150 <= int(self.hgt[:-2]) <= 193
        conditions = [
            (
                self.byr
                and re.match(r"^[0-9]{4}$", self.byr) is not None
                and 1920 <= int(self.byr) <= 2002
            ),
            (
                self.iyr
                and re.match(r"^[0-9]{4}$", self.iyr) is not None
                and 2010 <= int(self.iyr) <= 2020
            ),
            (
                self.eyr
                and re.match(r"^[0-9]{4}$", self.eyr) is not None
                and 2020 <= int(self.eyr) <= 2030
            ),
            (hgt_match),
            (self.hcl and re.match(r"^#[0-9a-f]{6}$", self.hcl) is not None),
            (
                self.ecl
                and re.match(r"^(amb|blu|brn|gry|grn|hzl|oth)$", self.ecl) is not None
            ),
            (self.pid and re.match(r"^\d{9}$", self.pid) is not None),
        ]
        return all(conditions)


def main() -> None:
    passports = []
    with open("2020/day4/input.txt") as f:
        buffer = ""
        for line in f.readlines():
            if not line.strip():
                passports.append(PassportInfo.from_buffer(buffer))
                buffer = ""
                continue
            buffer += "\n" + line
        passports.append(PassportInfo.from_buffer(buffer))
    valid_count = sum([p.valid() for p in passports])
    print(valid_count)


if __name__ == "__main__":
    main()
