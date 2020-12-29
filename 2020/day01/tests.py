from .part1 import main as main1
from .part2 import main as main2


prompt = [1721, 979, 366, 299, 675, 1456]


def test_part1():
    ret = main1(prompt)
    assert ret == 514579


def test_part2():
    ret = main2(prompt)
    assert ret == 241861950
