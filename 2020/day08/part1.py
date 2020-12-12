from dataclasses import dataclass
from typing import List, Tuple


@dataclass
class Machine:

    instructions: List[Tuple[str, int]]
    accumulator: int = 0
    index: int = 0

    @staticmethod
    def load(instructions: List[str]) -> "Machine":
        parsed = [(i.split(" ")[0], int(i.split(" ")[1])) for i in instructions]
        return Machine(instructions=parsed)

    def run(self) -> None:
        executed = []
        while True:
            if self.index in executed:
                break
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
        machine = Machine.load([e.strip() for e in f.readlines()])
    machine.run()
    print(machine.accumulator)


if __name__ == "__main__":
    main()
