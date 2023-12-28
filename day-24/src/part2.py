#!/usr/bin/env python3

import sys
import sympy

input = open(sys.argv[1], "r").read().splitlines()

hailstones = []
for line in input:
    parts = line.split(" @ ")
    (xh,yh,zh) = parts[0].split(", ")
    (vxh,vyh,vzh) = parts[1].split(", ")
    hailstones.append((int(xh), int(yh), int(zh), int(vxh), int(vyh), int(vzh)))

print("Loaded " + str(len(hailstones)) + " hailstones")

# Define unknown symbols
xr, yr, zr, vxr, vyr, vzr = sympy.symbols('xr yr zr vxr vyr vzr')

# Define equations
equations = []

for (xh,yh,zh,vxh,vyh,vzh) in hailstones[0:5]:
    equations.append((xh - xr) * (vyr - vyh) - (vxr - vxh) * (yh - yr))
    equations.append((yh - yr) * (vzr - vzh) - (vyr - vyh) * (zh - zr))

# Solve equations
solutions = sympy.solve(equations)
if len(solutions) == 0:
    print("No solutions found")
    sys.exit(1)

if len(solutions) > 1:
    print("Multiple solutions found")
    sys.exit(1)

solution = solutions[0]
print("Solution: " + str(solution.values()))

result = solution[xr] + solution[yr] + solution[zr]
print("Result: " + str(result))
