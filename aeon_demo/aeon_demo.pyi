from typing import List

# class Node:
#     index: int
#     label: str
#
#     def __init__(self) -> None: ...
#     def __repr__(self) -> str: ...
#
# class Edge:
#     source: int
#     target: int
#
#     def __init__(self) -> None: ...
#     def __repr__(self) -> str: ...

class Graph:
    def __init__(self) -> None: ...
    def add_node(self, label: str) -> None: ...
    def add_edge(self, source: int, target: int) -> None: ...
    def node_count(self) -> int: ...
    def edge_count(self) -> int: ...
    def get_node_label(self, index: int) -> str: ...
    def dfs(self, start_index: int) -> List[int]: ...
