import sys
import numpy as np
import numpy as np

#inf = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
inf = sys.argv[1] if len(sys.argv) > 1 else r"D:\Private_projects\Python\advent-of-code-2023\day05\python\tede\input.txt"

lines = [line for line in open(inf).read().split("\n\n")]

source = lines[0].split(' ')[1:]
source = [int(item) for item in source]

def find_starting_value_range(destination_start, source_start, ranges, target_number):
    for i in range(len(destination_start)):
        if source_start[i] <= target_number <= source_start[i] + ranges[i]:
            return destination_start[i] + (target_number - source_start[i])
    return target_number

for index, Imap in enumerate(lines[1:]):

    destination_start = [int(item.split(' ')[0]) for item in lines[index+1].split('\n')[1:]]
    source_start = [int(item.split(' ')[1]) for item in lines[index+1].split('\n')[1:]]
    value_range = [int(item.split(' ')[2]) for item in lines[index+1].split('\n')[1:]]

    #print(source)

    new_source = []
    for Isource in source:

        destination = find_starting_value_range(destination_start, source_start, value_range, Isource)
 

        new_source.append(destination)  

    source = new_source


closest = np.min(source)
print(f"Closest: {closest}")