
with open("input.txt", "r") as f:
    lines = f.read().strip().split("\n")



def getPower(game_str: str) -> int:
    set_l = [x.split(", ") for x in game_str.split(": ")[1].split("; ")]
    
    sets = []
    for st in set_l:
        # print(st)
        ses = {}
        for cube_info in st:
            ammount, color = cube_info.split(" ")
            ses[color] = int(ammount)
        
        sets.append(ses)


    reds = [sus.get("red", 0) for sus in sets]
    greens = [sus.get("green", 0) for sus in sets]
    blues = [sus.get("blue", 0) for sus in sets]

    # reds = [sus.get("red", -1) for sus in sets]
    # greens = [sus.get("green", -1) for sus in sets]
    # blues = [sus.get("blue", -1) for sus in sets]



    actual_config = {"red": max(reds), "green": max(greens), "blue": max(blues)}

    # print(reds)
    # print(greens)
    # print(blues)

    # print("="*20)
    print("R:", actual_config["red"], "G:", actual_config["green"], "B:", actual_config["blue"])
    print("="*20)

    # print(sets)
    # print(actual_config)
    # print(expected_config)

    # print(set_l)
    # print(ses)

    return actual_config["red"]*actual_config["green"]*actual_config["blue"]

    # return actual_config["red"] <= expected_config["red"] and actual_config["green"] <= expected_config["green"] and actual_config["blue"] <= expected_config["blue"]


def solve(game_str: str):
    set_l = [x.split(", ") for x in game_str.split(": ")[1].split("; ")]
    
    sets = []
    for st in set_l:
        # print(st)
        ses = {}
        for cube_info in st:
            ammount, color = cube_info.split(" ")
            ses[color] = int(ammount)
        
        sets.append(ses)


    reds = [sus.get("red", 0) for sus in sets]
    greens = [sus.get("green", 0) for sus in sets]
    blues = [sus.get("blue", 0) for sus in sets]

    # reds = [sus.get("red", -1) for sus in sets]
    # greens = [sus.get("green", -1) for sus in sets]
    # blues = [sus.get("blue", -1) for sus in sets]



    actual_config = {"red": max(reds), "green": max(greens), "blue": max(blues)}

    # print(reds)
    # print(greens)
    # print(blues)

    # print("="*20)
    # print("R:", actual_config["red"], "G:", actual_config["green"], "B:", actual_config["blue"])
    # print("="*20)

    # print(sets)
    # print(actual_config)
    # print(expected_config)

    # print(set_l)
    # print(ses)

    return actual_config

    # return actual_config["red"] <= expected_config["red"] and actual_config["green"] <= expected_config["green"] and actual_config["blue"] <= expected_config["blue"]




s = 0
for i, line in enumerate(lines):
    out = solve(line)
    print(f"Game{i+1}:{out}")
    # s += power
    s += out["red"]*out["green"]*out["blue"]
    # break

print(s)
