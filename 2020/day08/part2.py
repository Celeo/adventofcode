from dataclasses import dataclass
from enum import Enum, auto
from typing import List, Tuple


class TerminationReason(Enum):

    INF_LOOP = auto()
    OOB = auto()


@dataclass
class Machine:

    instructions: List[Tuple[str, int]]
    accumulator: int = 0
    index: int = 0

    @staticmethod
    def load(instructions: List[str]) -> "Machine":
        parsed = [(i.split(" ")[0], int(i.split(" ")[1])) for i in instructions]
        return Machine(instructions=parsed)

    def run(self) -> TerminationReason:
        executed = []
        while True:
            if self.index in executed:
                return TerminationReason.INF_LOOP
            if self.index >= len(self.instructions):
                return TerminationReason.OOB
            executed.append(self.index)
            nxt, val = self.instructions[self.index]
            if nxt == "acc":
                self.accumulator += val
                self.index += 1
            elif nxt == "jmp":
                self.index += val
            elif nxt == "nop":
                self.index += 1
            else:
                raise ValueError(f"Unknown instruction: {nxt}")


def main() -> None:
    with open("2020/day8/input.txt") as f:
        instructions = [e.strip() for e in f.readlines()]
    for index, instr in enumerate(instructions):
        new_instructions = instructions[:]
        if "nop" in instr:
            new_instructions[index] = new_instructions[index].replace("nop", "jmp")
        elif "jmp" in instr:
            new_instructions[index] = new_instructions[index].replace("jmp", "nop")
        m = Machine.load(new_instructions)
        tr = m.run()
        if tr == TerminationReason.OOB:
            print(m.accumulator)
            break


if __name__ == "__main__":
    main()
