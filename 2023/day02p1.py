
with open("input.txt", "r") as f:
    lines = f.read().strip().split("\n")



def isPossible(game_str: str, expected_config: dict) -> bool:
    set_l = [x.split(", ") for x in game_str.split(": ")[1].split("; ")]
    
    sets = []
    for st in set_l:
        # print(st)
        ses = {}
        for cube_info in st:
            amount, color = cube_info.split(" ")
            ses[color] = int(amount)
        
        sets.append(ses)


    reds = [sus.get("red", 0) for sus in sets]
    greens = [sus.get("green", 0) for sus in sets]
    blues = [sus.get("blue", 0) for sus in sets]

    actual_config = {"red": max(reds), "green": max(greens), "blue": max(blues)}


    # print(sets)
    # print(actual_config)
    # print(expected_config)

    # print(set_l)
    # print(ses)

    return actual_config["red"] <= expected_config["red"] and actual_config["green"] <= expected_config["green"] and actual_config["blue"] <= expected_config["blue"]





s = 0
for i, line in enumerate(lines):
    if isPossible(line, {"red": 12, "green": 13, "blue": 14}):
        print(line)
        s += i+1
    # break

print(s)
