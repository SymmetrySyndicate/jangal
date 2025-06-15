from typing import Any, Optional, TypeAlias, Union

NUMBER: TypeAlias = Union[int, float]


class Node:
    def __init__(self, value: Any, node_id: Optional[NUMBER] = None):
        self.value = value
        self.id = node_id if node_id is not None else id(self)

        # undirected
        self._edges = set()
        # directed
        self._incoming = set()
        self._outgoing = set()

        # tree stuff
        self._parent: "Node" = None
        self._children: set["Node"] = set()

    def add_edge(
        self,
        node: "Node",
        weight: Optional[NUMBER] = 0.0,
        directed: Optional[bool] = False,
        bidirectional: Optional[bool] = False,
    ) -> None:
        # TODO: do something with weight: another class ??
        assert isinstance(node, Node), (
            f"provide valid Node objects got type {type(node)}"
        )

        if directed:
            self._outgoing.add(node)
            node._incoming.add(self)
        else:
            if bidirectional:
                # undirected edge
                self._edges.add(node)
                node._edges.add(self)
            else:
                # excuse me ??
                self._edges.add(node)

    def add_child(self, child_node: "Node") -> None:
        assert isinstance(child_node, Node), (
            f"provide valid Node objects got type {type(child_node)}"
        )

        # clean-up old links
        if child_node._parent:
            child_node._parent._children.discard(child_node)

        child_node._parent = self
        self._children.add(child_node)

        # NOTE: self.add_edge() ??

    @property
    def parent(self) -> "Node":
        return self._parent

    @property
    def children(self) -> list["Node"]:
        return list(self._children)

    def dfs(self, visited=None):
        if visited is None:
            visited = set()

        if self in visited:
            return

        visited.add(self)
        yield self

        children = self._children

        for node in children:
            if node not in visited:
                yield from node.dfs(visited)

    def bfs(self):
        from collections import deque

        visited = set()
        queue = deque([self])
        visited.add(self)

        while queue:
            current = queue.popleft()
            yield current

            children = current._children

            for node in children:
                if node not in visited:
                    visited.add(node)
                    queue.append(node)

    def preorder(self):
        yield self
        for child in self._children:
            yield from child.preorder()

    def postorder(self):
        children = list(self._children)

        if not children:
            yield self
        elif len(children) == 1:
            yield from children[0].postorder()
            yield self
        else:
            for child in children:
                yield from child.postorder()
            yield self

    @property
    def is_root(self) -> bool:
        return self.parent is None

    @property
    def is_leaf(self) -> bool:
        return len(self._children) == 0

    @property
    def height(self) -> int:
        if self.is_leaf:
            return 0
        return 1 + max(child.height for child in self._children)

    @property
    def depth(self) -> int:
        if self.is_root:
            return 0
        return 1 + self.parent.depth

    @property
    def num_leaves(self) -> int:
        if self.is_leaf:
            return 1
        return sum(child.num_leaves for child in self._children)

    @property
    def num_nodes(self) -> int:
        return 1 + sum(child.num_nodes for child in self._children)

    @property
    def diameter(self) -> int:
        if self.is_leaf:
            return 0

        children_list = list(self._children)

        if len(children_list) == 0:
            return 0
        elif len(children_list) == 1:
            return 1 + children_list[0].height
        else:
            heights = [child.height for child in children_list]
            heights.sort(reverse=True)
            root_diameter = 2 + heights[0] + heights[1]  # path through root
            child_diameters = [child.diameter for child in children_list]

            return max(root_diameter, *child_diameters)

    @property
    def is_balanced(self) -> bool:
        if self.is_leaf:
            return True
        else:
            heights = [child.height for child in self._children]
            heights.sort(reverse=True)
            return all(height <= 1 for height in heights)

    def __hash__(self) -> int:
        return hash(self.id)

    def __str__(self) -> str:
        return f"Node(value={self.value})"

    def __repr__(self) -> str:
        return f"Node(value={self.value}, id={self.id})"

    # TODO: Implement a proper check
    # def __eq__(self, other: "Node") -> bool:
    #     pass
