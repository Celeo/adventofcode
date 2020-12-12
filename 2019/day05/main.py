from dataclasses import dataclass
from enum import Enum
from typing import List


ARGUMENT_COUNT = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 99: 0}


class ArgMode(Enum):

    Position = 0
    Immediate = 1


@dataclass
class OpCode:

    code: int
    arg_count: int
    arg_modes: List[ArgMode]


def parse_opcode(code: int) -> OpCode:
    s = str(code).zfill(4)
    opcode = int(s[-2:])
    arg_modes = [ArgMode(int(a)) for a in s[:-2][::-1]]
    arg_count = ARGUMENT_COUNT[opcode]
    while len(arg_modes) < arg_count:
        arg_modes.append(ArgMode.Position)
    return OpCode(code=opcode, arg_count=arg_count, arg_modes=arg_modes)


def get_at(
    instructions: List[int], position: int, offset: int, arg_mode: ArgMode
) -> int:
    if arg_mode == ArgMode.Immediate:
        return instructions[position + offset]
    return instructions[instructions[position + offset]]


def process(instructions: List[int], position: int) -> int:
    instruction = instructions[position]
    opcode = parse_opcode(instruction)
    if opcode.code == 99:
        return -1
    elif opcode.code == 1:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        instructions[instructions[position + 3]] = arg_1 + arg_2
    elif opcode.code == 2:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        instructions[instructions[position + 3]] = arg_1 * arg_2
    elif opcode.code == 3:
        val = int(input("Enter value: "))
        if opcode.arg_modes[0] == ArgMode.Immediate:
            instructions[position + 1] = val
        else:
            instructions[instructions[position + 1]] = val
    elif opcode.code == 4:
        val = 0
        if opcode.arg_modes[0] == ArgMode.Immediate:
            val = instructions[position + 1]
        else:
            val = instructions[instructions[position + 1]]
        print(val)
    elif opcode.code == 5:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        if arg_1 != 0:
            return arg_2
    elif opcode.code == 6:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        if arg_1 == 0:
            return arg_2
    elif opcode.code == 7:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        val = 1 if arg_1 < arg_2 else 0
        if opcode.arg_modes[2] == ArgMode.Immediate:
            instructions[position + 3] = val
        else:
            instructions[instructions[position + 3]] = val
    elif opcode.code == 8:
        arg_1 = get_at(instructions, position, 1, opcode.arg_modes[0])
        arg_2 = get_at(instructions, position, 2, opcode.arg_modes[1])
        val = 1 if arg_1 == arg_2 else 0
        if opcode.arg_modes[2] == ArgMode.Immediate:
            instructions[position + 3] = val
        else:
            instructions[instructions[position + 3]] = val
    else:
        raise ValueError(opcode.code)
    return position + opcode.arg_count + 1


def process_all(instructions: List[int]) -> List[int]:
    position = 0
    while position != -1:
        position = process(instructions, position)
    return instructions


def main() -> None:
    with open("2019/day5/input.txt") as f:
        instructions = [int(e) for e in f.read().split(",")]
    process_all(instructions[:])


if __name__ == "__main__":
    main()
