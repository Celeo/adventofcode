def main() -> None:
    with open("2020/day9/input.txt") as f:
        numbers = [int(line.strip()) for line in f.readlines()]
    magic_number = 27911108
    for index in range(0, len(numbers)):
        for back_index in range(1, index + 1):
            check_range = numbers[index - back_index : index]
            if sum(check_range) == magic_number:
                print(min(check_range) + max(check_range))
                return


if __name__ == "__main__":
    main()
