import sys

inf = sys.argv[1] if len(sys.argv) > 1 else "input"

lines = [line for line in open(inf).read().split("\n")]


copies = {line: 1 for line in range(1,len(lines) + 1)}


sum = 0

for index, l in enumerate(lines):
    winning_numbers = l.split('|')[0].split(':')[1].split(' ')
    winning_numbers = list(filter(lambda x: x != '', winning_numbers))

    my_numbers = l.split('|')[1].split(' ')
    my_numbers = list(filter(lambda x: x != '', my_numbers))

    matching = 0

    for Inum in winning_numbers:
        if Inum in my_numbers:
            matching += 1

    for Icopy in range(copies[index + 1]):
        for Imatch in range(1, matching+1):
            if index + 1 + Imatch < len(lines):
                copies[index + 1 + Imatch] += 1

for _, Ivalue in copies.items():
    sum += Ivalue



print(sum)


