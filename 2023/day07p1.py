#!/usr/bin/env python

import re
from collections import defaultdict


with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]

card_to_value = {card: i for i, card in enumerate("A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2".split(", ")[::-1])}
hand_to_type = {
        count_list: i for i, count_list in enumerate([
            "1 1 1 1 1",
            "1 1 1 2",
            "1 2 2",
            "1 1 3",
            "2 3",
            "1 4",
            "5"
        ])
    }

class Hand:

    def __init__(self, cards: list, bid: int):
        self.cards = cards
        self.bid = bid
        self.type = self.find_type(cards)

    # def __eq__(self, other: Hand):
    #     return self.type == other.type

    # def __lt__(self, other: Hand):
    #     if self != other:
            
    #         return self.type < other.type

    def compare_to(self, other):
        result = self.type - other.type
        if result == 0:
            for c1, c2 in zip(self.cards, other.cards):
                result = card_to_value[c1] - card_to_value[c2]
                if result != 0:
                    break

        return result

    def find_type(self, cards):
        m = {}
        unique_cards = list(set(cards))
        for card in unique_cards:
            m[card] = cards.count(card)

        return hand_to_type[" ".join(map(str, sorted(m.values())))]

hands = []
for line in lines:
    hand, bid = line.split()
    cards = [card for card in hand]
    bid = int(bid)
    hands.append(Hand(cards, bid))

# for hand in hands:
#     print(hand.cards, hand.bid)

i = 0
while i < len(hands)-1:
    min_idx = i

    j = i+1
    while j < len(hands):
        if hands[j].compare_to(hands[min_idx]) < 0:
            min_idx = j

        j += 1    

    hands[i], hands[min_idx] = hands[min_idx], hands[i]    
    
    i += 1 


# print("sorted:")
# i = 0
# for hand in hands:
#     print("RANK:", i+1, "".join(hand.cards), hand.bid)
#     i += 1

ANSWER = sum([hand.bid*(i+1) for i, hand in enumerate(hands)])

print("ANSWER:", ANSWER)