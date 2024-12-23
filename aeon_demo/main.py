from biodivine_aeon import AsynchronousGraph, BooleanNetwork

from aeon_demo import *

boolean_network = BooleanNetwork.from_file("edition-2022-aeon/001.aeon")
print(boolean_network)

graph = AsynchronousGraph(boolean_network)
print(graph)
