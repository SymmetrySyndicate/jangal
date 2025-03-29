from src import vEB


def test_veb() -> None:
    tree = vEB(16)
    for i in range(16):
        tree.insert(i)
        assert tree.get(tree.high(i), tree.low(i)) == i

    assert tree.min == 0
    assert tree.max == 15

    for i in range(16):
        if i + 1 < 16:
            assert tree.successor(i) == i + 1

    for i in range(15, -1, -1):
        if i - 1 >= 0:
            assert tree.predecessor(i) == i - 1

    for i in range(16):
        assert tree.isin(i)
        tree.delete(i)
        assert not tree.isin(i)

    assert tree.min is None
    assert tree.max is None
