
with open("input.txt", "r") as f:
    lines = f.read().strip().split("\n")


numstr_to_digit = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

s = 0
for line in lines:
    digits = []

    i = 0
    while (i < len(line)):
        c = line[i]

        if c.isdigit():
            digits.append(c)
        else:
            for numstr, digit in numstr_to_digit.items():
                if line[i:i+len(numstr)] == numstr:
                    digits.append(digit)
                    break

        i += 1

    s += int(digits[0] + digits[-1])

print(s)