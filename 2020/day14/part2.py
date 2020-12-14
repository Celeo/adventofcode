import re
from typing import Iterator, List


MEM_REGEX = re.compile(r"mem\[(\d+)\] = (\d+)")


# Thanks to https://www.reddit.com/r/adventofcode/comments/kcr1ct/2020_day_14_solutions/gfsc1yj/ for this.
#
# I attempted to use `itertools.permutations` based on the number of "X"s in `apply_mask::base`, but
# when the number of "X"s was 9, it ... didn't finish within 10 or so minutes. Given the AoC guidelines,
# that was clearly not the correct approach.
#
# There does seem to be another approach using powersets, but this seems cleaner.
def permutations(mask: str) -> Iterator[str]:
    if not mask:
        yield ""
        return
    for m in permutations(mask[1:]):
        if mask[0] == "X":
            yield "0" + m
            yield "1" + m
        else:
            yield mask[0] + m


def apply_mask(mask: str, value: int) -> List[int]:
    base = ""
    as_bin = bin(value)[2:].zfill(36)
    for index in range(len(mask)):
        if mask[index] == "0":
            base += as_bin[index]
        else:
            base += mask[index]
    bins = list(permutations(base))
    return [int(b, 2) for b in bins]


def main() -> None:
    with open("2020/day14/input.txt") as f:
        data = [l.strip() for l in f.readlines()]
    memory = {}
    mask = ""
    for line in data:
        if "mask" in line:
            mask = line.split()[2]
            continue
        address, value = map(int, MEM_REGEX.findall(line)[0])
        new_addresses = apply_mask(mask, address)
        for address in new_addresses:
            memory[address] = value
    print(sum(memory.values()))


if __name__ == "__main__":
    main()
