import sys

inf = sys.argv[1] if len(sys.argv) > 1 else "input.txt"

hands = []
values = []

trans_table = str.maketrans("AKQJT", "FDCBA")

def get_type(hand):

    hand_elements = {}

    for card in hand:
        if card in hand_elements:
            hand_elements[card] += 1
        else:
            hand_elements[card] = 1

    sorted_hand = dict(sorted(hand_elements.items(), key=lambda x: x[1], reverse=True))
    
    for key in sorted_hand:
        if sorted_hand[key] == 5:
            return 6
        elif sorted_hand[key] == 4:
            return 5
        elif sorted_hand[key] == 3 and len(sorted_hand) == 2:
            return 4
        elif sorted_hand[key] == 3 and len(sorted_hand) == 3:
            return 3
        elif sorted_hand[key] == 2 and len(sorted_hand) == 3:
            return 2
        elif sorted_hand[key] == 2 and len(sorted_hand) == 4:
            return 1
        else:
            return 0


with open(inf) as f:
    for line in f:
        type = get_type(line.split(' ')[0].strip())

        hands.append(str(type) + line.split(' ')[0].strip().translate(trans_table))
        values.append(int(line.split(' ')[1].strip()))


sorted_hands, sorted_values = zip(*sorted(zip(hands, values), key=lambda x: int(x[0], 16)))

print(sorted_hands)
print(sorted_values)

sum = 0

for index, Ivalue in enumerate(sorted_values):
    sum += Ivalue * (index + 1)
    print(index, Ivalue, Ivalue * (index + 1))

print(sum)

