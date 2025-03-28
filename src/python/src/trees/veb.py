import math

from typing import Optional

__all__ = ["vEB"]

# TODO: Add support for floating point numbers


class vEB:
    def __init__(self, size: int):
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

    def high(self, x) -> int:
        """
        get the high order bits of x

        Args:
            x: The element to get the high order bits from

        Returns:
            int: high order bits of x
        """

        return x // int(math.sqrt(self.size))

    def low(self, x) -> int:
        """
        get the low order bits of x

        Args:
            x: The element to get the low order bits from

        Returns:
            int: low order bits of x
        """

        return x % int(math.sqrt(self.size))

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

    def insert(self, x) -> None:
        """
        insert x into the vEB tree

        Args:
            x: element to insert
        """

        if self.min is None:
            self.min = x
            self.max = x
        else:
            if x < self.min:
                x, self.min = self.min, x
            if x > self.max:
                self.max = x
            if self.size > 2:
                if self.cluster[self.high(x)].min is None:
                    self.summary.insert(self.high(x))
                    self.cluster[self.high(x)].min = self.low(x)
                    self.cluster[self.high(x)].max = self.low(x)
                else:
                    self.cluster[self.high(x)].insert(self.low(x))

    def delete(self, x) -> None:
        """
        delete x from the vEB tree

        Args:
            x: element to delete
        """

        if self.min == self.max:
            self.min = None
            self.max = None

        elif self.size == 2:
            if x == 0:
                self.min = 1
            else:
                self.min = 0
            self.max = self.min

        else:
            if x == self.min:
                first_cluster = self.summary.min
                x = self.get(first_cluster, self.cluster[first_cluster].min)
                self.min = x
            self.cluster[self.high(x)].delete(self.low(x))
            if self.cluster[self.high(x)].min is None:
                self.summary.delete(self.high(x))
                if x == self.max:
                    summary_max = self.summary.max
                    if summary_max is None:
                        self.max = self.min
                    else:
                        self.max = self.get(summary_max, self.cluster[summary_max].max)
            elif x == self.max:
                self.max = self.get(self.high(x), self.cluster[self.high(x)].max)

    def successor(self, x) -> Optional[int]:
        """
        returns the successor of x in the vEB tree

        Args:
            x: element to find the successor of

        Returns:
            Optional[int]: successor of x, None if x is the maximum element
        """

        if self.size == 2:
            if x == 0 and self.max == 1:
                return 1
            else:
                return None

        elif self.min is not None and x < self.min:
            return self.min

        else:
            max_low = self.cluster[self.high(x)].max
            if max_low is not None and self.low(x) < max_low:
                offset = self.cluster[self.high(x)].successor(self.low(x))
                return self.get(self.high(x), offset)
            else:
                succ_cluster = self.summary.successor(self.high(x))
                if succ_cluster is None:
                    return None
                else:
                    offset = self.cluster[succ_cluster].min
                    return self.get(succ_cluster, offset)

    def predecessor(self, x) -> Optional[int]:
        """
        returns the predecessor of x in the vEB tree

        Args:
            x: element to find the predecessor of

        Returns:
            Optional[int]: predecessor of x, None if x is the minimum element
        """

        if self.size == 2:
            if x == 1 and self.min == 0:
                return 0
            else:
                return None

        elif self.max is not None and x > self.max:
            return self.max

        else:
            min_low = self.cluster[self.high(x)].min
            if min_low is not None and self.low(x) > min_low:
                offset = self.cluster[self.high(x)].predecessor(self.low(x))
                return self.get(self.high(x), offset)
            else:
                pred_cluster = self.summary.predecessor(self.high(x))
                if pred_cluster is None:
                    if self.min is not None and x > self.min:
                        return self.min
                    else:
                        return None
                else:
                    offset = self.cluster[pred_cluster].max
                    return self.get(pred_cluster, offset)

    def isin(self, x) -> bool:
        """
        check if x is in the vEB tree

        Args:
            x: element to check

        Returns:
            bool: True if x is in the vEB tree, False otherwise
        """

        if x == self.min or x == self.max:
            return True
        elif self.size == 2:
            return False
        else:
            return self.cluster[self.high(x)].isin(self.low(x))
