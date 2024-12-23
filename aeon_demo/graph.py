from aeon_demo import *

graph = Graph()
graph.add_node("A")
graph.add_node("B")
graph.add_node("C")
graph.add_edge(0, 1)
graph.add_edge(1, 2)
graph.add_edge(2, 0)

visited_nodes = graph.dfs(1)
print(visited_nodes)
