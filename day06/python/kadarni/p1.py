import numpy as np

content = open('./day06/input.txt').readlines()

def find_options(t,d):
    first = np.ceil((-t+np.sqrt(t**2-4*d))/-2)
    options = t-first-first+1
    return options

ts = np.asarray(content[0].split()[1:]).astype(float)
ds = np.asarray(content[1].split()[1:]).astype(float)

mult_options = 1
for i,t in enumerate(ts):
    mult_options *=find_options(t,ds[i])

print(mult_options)

ts = np.asarray(content[0].split(':')[1].replace(' ','')).astype(float)
ds = np.asarray(content[1].split(':')[1].replace(' ','')).astype(float)

print(find_options(ts,ds))