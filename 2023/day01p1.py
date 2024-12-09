
with open("input.txt") as f:
    lines = f.read().strip().split("\n")

s = 0 
for line in lines:
    digits = [c for c in line if c.isdigit()]
    s += int(digits[0] + digits[-1])

print(s)