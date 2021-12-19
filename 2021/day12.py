from helpers.utils import *
from collections import defaultdict
import re

DAY = 12

class Edge:
    def __init__(self, src, dst):
        self.src = src
        self.dst = dst
    def __repr__(self):
        return f"{self.src}-{self.dst}"

class Graph:
    def __init__(self, edges, max_visits):
        self.vertices = set()
        self.graph = list()
        for u, v in edges:
            self.vertices.add(u)
            self.vertices.add(v)
            if u == "start" or v == "end":
                self.add_edge(u, v)
            elif v == "start" or u == "end":
                self.add_edge(v, u)
            else:
                self.add_edge(u, v)
                self.add_edge(v, u)
        self.max_visits = max_visits
        self.paths = []
    
    def add_edge(self, u, v):
        edge = Edge(u, v)
        self.graph.append(edge)

    def find_all_paths(self, cur, dst, visited, path):
        visited[cur] += 1
        path.append(cur)
        if cur == dst:
            self.paths.append(','.join(path.copy()))
        for edge in self.graph:
            if edge.src == cur:
                if edge.dst.isupper():
                    self.find_all_paths(edge.dst, dst, visited, path)
                if edge.dst.islower():
                    if visited[edge.dst] < 1:
                        self.find_all_paths(edge.dst, dst, visited, path)
        path.pop()
        visited[cur] -= 1

    def find_all_paths_v2(self, cur, dst, visited, path, bonus):
        visited[cur] += 1
        path.append(cur)
        if cur == dst:
            self.paths.append(','.join(path.copy()))
        for edge in self.graph:
            if edge.src == cur:
                if edge.dst.isupper():
                    self.find_all_paths_v2(edge.dst, dst, visited, path, bonus)
                if edge.dst.islower():
                    allowed_visits = 1
                    if visited[edge.dst] < allowed_visits:
                        self.find_all_paths_v2(edge.dst, dst, visited, path, bonus)
                    elif bonus:
                        self.find_all_paths_v2(edge.dst, dst, visited, path, False)
        path.pop()
        visited[cur] -= 1
    
    def count_vertice_neighbors(self, vertice):
        return len([edge for edge in self.graph if edge.src == vertice])



def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    edges = [re.findall(r'^(.+)-(.+)$', line)[0] for line in lines]
    graph = Graph(edges, 1)
    visited = {v: 0 for v in graph.vertices}
    graph.find_all_paths("start", "end", visited, [])
    return len(graph.paths)
    

print("== FIRST PART ==")
assert_example(first_part, 10)
get_result(first_part)

def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    edges = [re.findall(r'^(.+)-(.+)$', line)[0] for line in lines]
    graph = Graph(edges, 2)
    visited = {v: 0 for v in graph.vertices}
    graph.find_all_paths_v2("start", "end", visited, [], True)
    return len(graph.paths)


print("== SECOND PART ==")
assert_example(second_part, 36)
assert_example2(second_part, 103)
assert_example3(second_part, 3509)
get_result(second_part)