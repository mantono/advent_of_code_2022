from collections import Counter, defaultdict, deque
from functools import cache, reduce
from heapq import heapify, heappop, heappush
from itertools import combinations, permutations, product
from helpers import chunks, chunks_with_overlap, columns, digits, distance, distance_sq, eight_neighs, eight_neighs_bounded, grouped_lines, ints, manhattan, multall, n_neighs, neighs, neighs_bounded


def solve(lines):
    def letval(c):
        base, comp = (27, 'A') if c.isupper() else (1, 'a')
        return base + ord(c)-ord(comp)

    def common(group):
        return [c for c in group[0] if c in group[1] and c in group[2]][0]
        
    def groupval(group):
        return letval(common(group))
    
    return sum(groupval(chunk) for chunk in chunks(lines, 3))


def main():
    lines = []

    with open('3.txt') as f:
        for line in f.readlines():
            lines.append(line)
            
    return solve(lines)


if __name__ == '__main__':
    print(main())
