import sys
import numpy as np
import numpy as np

def find_starting_value_range(destination_starts, source_starts, ranges, target_start, target_range, result_starts = None, result_ranges = None):
    if result_starts is None:
        result_starts = []
        result_ranges = []

    for i in range(len(destination_starts)):

        if (target_start + target_range < source_starts[i]) or (target_start > source_starts[i] + ranges[i]):
            
            continue

        elif target_start < source_starts[i] and target_start + target_range > source_starts[i] and \
        target_start + target_range < source_starts[i] + ranges[i]:

            result_starts.append(destination_starts[i])
            result_ranges.append(target_start + target_range - source_starts[i])

            new_target_start = target_start
            new_target_range = source_starts[i] - target_start

            find_starting_value_range(destination_starts, source_starts, ranges, new_target_start, new_target_range, result_starts, result_ranges)

            return result_starts, result_ranges

        elif target_start >= source_starts[i] and target_start + target_range <= source_starts[i] + ranges[i]:

            result_starts.append(destination_starts[i] + (target_start - source_starts[i]))
            result_ranges.append(target_range)

            return result_starts, result_ranges

        elif target_start > source_starts[i] and target_start + target_range > source_starts[i] + ranges[i] and \
            target_start < source_starts[i] + ranges[i]:

            result_starts.append(destination_starts[i] + (target_start - source_starts[i]))
            result_ranges.append(source_starts[i] + ranges[i] - target_start)

            new_target_start = source_starts[i] + ranges[i]
            new_target_range = target_start + target_range - new_target_start

            find_starting_value_range(destination_starts, source_starts, ranges, new_target_start, new_target_range, result_starts, result_ranges)

            return result_starts, result_ranges

        elif target_start < source_starts[i] and target_start + target_range > source_starts[i] + ranges[i]:

            result_starts.append(destination_starts[i])
            result_ranges.append(ranges[i])

            new_target_start = source_starts[i] + ranges[i]
            new_target_range = target_start + target_range - new_target_start

            find_starting_value_range(destination_starts, source_starts, ranges, new_target_start, new_target_range, result_starts, result_ranges)

            new_target_start = target_start
            new_target_range = source_starts[i] - target_start

            find_starting_value_range(destination_starts, source_starts, ranges, new_target_start, new_target_range, result_starts, result_ranges)

            return result_starts, result_ranges

    result_starts.append(target_start)
    result_ranges.append(target_range)

    return result_starts, result_ranges


#inf = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
inf = sys.argv[1] if len(sys.argv) > 1 else r"D:\Private_projects\Python\advent-of-code-2023\day05\python\tede\input.txt"

lines = [line for line in open(inf).read().split("\n\n")]

source = lines[0].split(' ')[1:]

target_start = [int(item) for index, item in enumerate(source) if index % 2 == 0]
target_range = [int(item) for index, item in enumerate(source) if index % 2 == 1]   

for index, Imap in enumerate(lines[1:]):

    destination_start = [int(item.split(' ')[0]) for item in lines[index+1].split('\n')[1:]]
    source_start = [int(item.split(' ')[1]) for item in lines[index+1].split('\n')[1:]]
    value_range = [int(item.split(' ')[2]) for item in lines[index+1].split('\n')[1:]]

    new_start = []  
    new_range = []

    for Itarget_start, Itarget_range in zip(target_start, target_range):

        temp_start, temp_range = find_starting_value_range(destination_start, source_start, value_range, Itarget_start, Itarget_range)

        for Istart_new, Irange_new in zip(temp_start, temp_range):
            new_start.append(Istart_new)
            new_range.append(Irange_new)
        

    target_start = new_start
    target_range = new_range

closest = np.min(target_start)
print(f"Closest: {closest}")

