#!/usr/bin/env python

ANSWER=None

with open("input.txt", "r") as f:
    lines = f.read().strip().split("\n")


cards = []

def count_matches(card):
    count = 0
    for num in card[1]:
        if num in card[0]:
            count += 1
    return count


for line in lines:
    winning_numbers, personal_numbers = [x.split() for x in line.split(": ")[1].split(" | ")]

    cards.append((winning_numbers, personal_numbers))



m = {i+1: count_matches(card) for i, card in enumerate(cards)}

scratchcards = {}
for i in range(len(cards)):
    scratchcards[i+1] = 0

def reco(i):
    for j in range(i+1, i+1+m[i]):
        new_count = m[j]
        # print(f"Card {j+1} has {new_count}")
        scratchcards[j] += 1 #count_matches(cards[j])
        
        if new_count > 0:
            reco(j)



for i in range(1, len(cards)+1):
    scratchcards[i] += 1
    reco(i)


print(scratchcards)
ANSWER = sum(scratchcards.values())

print(ANSWER)