import math

from typing import Optional

from .base import Tree

__all__ = ["vEB"]


class vEB(Tree):
    def __init__(self, size: int) -> None:
        """
        Args:
            size (int): The size of the vEB tree
        """

        self.size = size
        self.min = None
        self.max = None
        self.summary = None
        self.cluster = {}

        # trivial case
        if self.size == 2:
            self.summary = None
            self.cluster = None
        else:
            self.summary = vEB(int(math.sqrt(size)))
            self.cluster = [
                vEB(int(math.sqrt(size))) for i in range(int(math.sqrt(size)))
            ]

    def high(self, element) -> int:
        """
        get the high order bits of some element

        Args:
            element: The element to get the high order bits from

        Returns:
            int: high order bits of element
        """

        return element // int(math.sqrt(self.size))

    def low(self, element) -> int:
        """
        get the low order bits of element

        Args:
            element: The element to get the low order bits from

        Returns:
            int: low order bits of element
        """

        return element % int(math.sqrt(self.size))

    def get(self, x, y) -> int:
        """
        get the element from the high and low order bits

        Args:
            x: high order bits
            y: low order bits

        Returns:
            int: element
        """

        return x * int(math.sqrt(self.size)) + y

    def insert(self, element) -> None:
        """
        insert some element into the vEB tree

        Args:
            element: element to insert
        """

        if self.min is None:
            self.min = element
            self.max = element
        else:
            if element < self.min:
                element, self.min = self.min, element
            if element > self.max:
                self.max = element
            if self.size > 2:
                if self.cluster[self.high(element)].min is None:
                    self.summary.insert(self.high(element))
                    self.cluster[self.high(element)].min = self.low(element)
                    self.cluster[self.high(element)].max = self.low(element)
                else:
                    self.cluster[self.high(element)].insert(self.low(element))

    def delete(self, element) -> None:
        """
        delete element from the vEB tree

        Args:
            element: element to delete
        """

        if self.min == self.max:
            self.min = None
            self.max = None

        elif self.size == 2:
            if element == 0:
                self.min = 1
            else:
                self.min = 0
            self.max = self.min

        else:
            if element == self.min:
                first_cluster = self.summary.min
                x = self.get(first_cluster, self.cluster[first_cluster].min)
                self.min = x
            self.cluster[self.high(x)].delete(self.low(x))
            if self.cluster[self.high(x)].min is None:
                self.summary.delete(self.high(x))
                if element == self.max:
                    summary_max = self.summary.max
                    if summary_max is None:
                        self.max = self.min
                    else:
                        self.max = self.get(summary_max, self.cluster[summary_max].max)
            elif element == self.max:
                self.max = self.get(self.high(x), self.cluster[self.high(x)].max)

    def successor(self, element) -> Optional[int]:
        """
        returns the successor of some element in the vEB tree

        Args:
            element: element to find the successor of

        Returns:
            Optional[int]: successor of element, None if x is the maximum element
        """

        if self.size == 2:
            if element == 0 and self.max == 1:
                return 1
            else:
                return None

        elif self.min is not None and element < self.min:
            return self.min

        else:
            max_low = self.cluster[self.high(element)].max
            if max_low is not None and self.low(element) < max_low:
                offset = self.cluster[self.high(element)].successor(self.low(element))
                return self.get(self.high(element), offset)
            else:
                succ_cluster = self.summary.successor(self.high(element))
                if succ_cluster is None:
                    return None
                else:
                    offset = self.cluster[succ_cluster].min
                    return self.get(succ_cluster, offset)

    def predecessor(self, element) -> Optional[int]:
        """
        returns the predecessor of some element in the vEB tree

        Args:
            element: element to find the predecessor of

        Returns:
            Optional[int]: predecessor of element, None if element is the minimum element
        """

        if self.size == 2:
            if element == 1 and self.min == 0:
                return 0
            else:
                return None

        elif self.max is not None and element > self.max:
            return self.max

        else:
            min_low = self.cluster[self.high(element)].min
            if min_low is not None and self.low(element) > min_low:
                offset = self.cluster[self.high(element)].predecessor(self.low(element))
                return self.get(self.high(element), offset)
            else:
                pred_cluster = self.summary.predecessor(self.high(element))
                if pred_cluster is None:
                    if self.min is not None and element > self.min:
                        return self.min
                    else:
                        return None
                else:
                    offset = self.cluster[pred_cluster].max
                    return self.get(pred_cluster, offset)

    def isin(self, element) -> bool:
        """
        check if some element is in the vEB tree

        Args:
            element: element to check

        Returns:
            bool: True if the element is in the vEB tree, False otherwise
        """

        if element == self.min or element == self.max:
            return True
        elif self.size == 2:
            return False
        else:
            return self.cluster[self.high(element)].isin(self.low(element))
