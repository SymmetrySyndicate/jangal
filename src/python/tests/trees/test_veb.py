from src.trees import vEB


def test_veb() -> None:
    tree = vEB(16)
    for i in range(16):
        tree.insert(i)
        assert tree.get(tree.high(i), tree.low(i)) == i

    for i in range(16):
        assert tree.isin(i)
        tree.delete(i)
        assert not tree.isin(i)
