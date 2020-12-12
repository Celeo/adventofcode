from typing import List, Tuple


def process(instructions: List[int], position: int) -> int:
    code = instructions[position]
    if code == 99:
        return -1
    arg_1, arg_2, arg_3 = instructions[position + 1 : position + 4]
    if code == 1:
        instructions[arg_3] = instructions[arg_1] + instructions[arg_2]
    elif code == 2:
        instructions[arg_3] = instructions[arg_1] * instructions[arg_2]
    return position + 4


def process_all(instructions: List[int], noun: int = 12, verb: int = 2) -> List[int]:
    instructions[1] = noun
    instructions[2] = verb
    position = 0
    while position != -1:
        position = process(instructions, position)
    return instructions


def guess_input(instructions: List[int], desired: int) -> Tuple[int, int]:
    noun, verb = 0, 0
    while True:
        result = process_all(instructions[:], noun, verb)
        if result[0] == desired:
            return (noun, verb)
        verb += 1
        if verb == 99:
            verb = 0
            noun += 1


def main() -> None:
    with open("2019/day2/input.txt") as f:
        instructions = [int(e) for e in f.read().split(",")]
    part_1 = process_all(instructions[:])
    print(part_1[0])
    part_2 = guess_input(instructions[:], 19690720)
    print(100 * part_2[0] + part_2[1])


if __name__ == "__main__":
    main()
