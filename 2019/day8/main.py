from typing import List


WIDTH = 25
HEIGHT = 6
PER_LAYER = WIDTH * HEIGHT


def part_1(layers: List[str]) -> int:
    layer_with_fewest = []
    fewest = 2 ** 32
    for layer in layers:
        zeroes = layer.count("0")
        if zeroes < fewest:
            layer_with_fewest = layer[:]
            fewest = zeroes
    return layer_with_fewest.count("1") * layer_with_fewest.count("2")


def part_2(layers: List[str]) -> str:
    ret = ""
    image = layers[0]
    for layer in layers[1:]:
        replacement = ""
        for pixel in range(0, PER_LAYER):
            if image[pixel] == "2":
                replacement += layer[pixel]
            else:
                replacement += image[pixel]
        image = replacement
    for i in range(0, HEIGHT):
        ret += (
            image[(i * WIDTH) : ((i + 1) * WIDTH)].replace("0", " ").replace("1", "x")
            + "\n"
        )
    return ret


def main() -> None:
    with open("2019/day8/input.txt") as f:
        data = f.read().strip()
    layers = []
    for i in range(0, len(data) // PER_LAYER):
        layers.append(data[(i * PER_LAYER) : ((i + 1) * PER_LAYER)])
    print(part_1(layers))
    print(part_2(layers))


if __name__ == "__main__":
    main()
