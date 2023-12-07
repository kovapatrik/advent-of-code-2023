import sys

#inf = sys.argv[1] if len(sys.argv) > 1 else "input_test.txt"

inf = sys.argv[1] if len(sys.argv) > 1 else r"D:\Private_projects\Python\advent-of-code-2023\day07\python\tede\input.txt"

hands = []
values = []
joker_hands = []

trans_table = str.maketrans("AKQTJ", "DCBA1")
reverse_trans_table = (str.maketrans("DCBA1", "AKQTJ"))

hexa_range = [2, 3, 4, 5, 6, 7, 8, 9, 'A', 'B', 'C', 'D']

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
        
def optimize_joker_hand(hand):

    sub_values = []

    for Isub in hexa_range:

        sub_hand = hand.replace('1', str(Isub))
        type = get_type(sub_hand)
        sub_values.append(str(type) + hand)

    sorted_sub_hands, sorted_sub_digit = zip(*sorted(zip(sub_values, hexa_range), key=lambda x: int(x[0], 16)))

    return sorted_sub_hands[-1][0], sorted_sub_digit[-1]


with open(inf) as f:
    for line in f:

        hand_translated = line.split(' ')[0].strip().translate(trans_table)
        value = int(line.split(' ')[1].strip())
        print(hand_translated.translate(reverse_trans_table))
        if 'J' in line:
            
            type, opt_sub = optimize_joker_hand(hand_translated)
            optimal_hand = hand_translated.replace('1', str(opt_sub))
            print(f'after optimization: {optimal_hand.translate(reverse_trans_table)}')

        else:

            type = get_type(hand_translated)

        hands.append(str(type) + hand_translated)
        print(f'hand with type: {hands[-1]}')
        values.append(value)

sorted_hands, sorted_values = zip(*sorted(zip(hands, values), key=lambda x: int(x[0], 16)))

print(sorted_hands)
print(sorted_values)

sum = 0

for index, Ivalue in enumerate(sorted_values):
    sum += Ivalue * (index + 1)
    print(index + 1, Ivalue, Ivalue * (index + 1))

print(sum)