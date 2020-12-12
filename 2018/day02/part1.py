from string import ascii_lowercase


def main() -> None:
    with open("2018/day2/input.txt") as f:
        ids = [l.strip() for l in f.readlines()]
    counts = {2: 0, 3: 0}
    for i in ids:
        found_2, found_3 = False, False
        for letter in ascii_lowercase:
            if i.count(letter) == 3:
                found_3 = True
            elif i.count(letter) == 2:
                found_2 = True
        counts[2] += 1 if found_2 else 0
        counts[3] += 1 if found_3 else 0
    print(counts[2] * counts[3])


if __name__ == "__main__":
    main()
