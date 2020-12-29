from .part1 import main as main1
from .part2 import main as main2


prompt = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]


def test_part1():
    ret = main1(prompt)
    assert ret == 2


def test_part2():
    ret = main2(prompt)
    assert ret == 1
