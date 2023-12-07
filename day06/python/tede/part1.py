from functools import reduce

time = [46,     68,     98,     66]
distance = [358,   1054,   1807,   1080]

reduce(lambda x, y: x * y, [len([ms for ms in range(Itime) if ms * (Itime - ms) > Idistance]) for (Itime, Idistance) in zip(time, distance)])

