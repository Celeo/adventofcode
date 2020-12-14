import re


MEM_REGEX = re.compile(r"mem\[(\d+)\] = (\d+)")


def apply_mask(mask: str, value: int) -> int:
    ret = ""
    as_bin = bin(value)[2:].zfill(36)
    for index in range(len(mask)):
        if mask[index] == "X":
            ret += as_bin[index]
        else:
            ret += mask[index]
    return int(ret, 2)


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
        new_val = apply_mask(mask, value)
        memory[address] = new_val
    print(sum(memory.values()))


if __name__ == "__main__":
    main()
