import sys

inf = sys.argv[1] if len(sys.argv) > 1 else "input"

lines = [line for line in open(inf).read().split("\n")]

sum = 0

for l in lines:
    winning_numbers = l.split('|')[0].split(':')[1].split(' ')
    winning_numbers = list(filter(lambda x: x != '', winning_numbers))

    my_numbers = l.split('|')[1].split(' ')
    my_numbers = list(filter(lambda x: x != '', my_numbers))

    line_val = 0
    counter = 0

    for Inum in winning_numbers:
        if Inum in my_numbers:
            if counter != 0:
                line_val *= 2
            else:
                line_val = 1
            counter += 1

    sum += line_val

print(sum)


