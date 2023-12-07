time = 46689866
Distance = 358105418071080

#[ms for ms in range(limit) if ms * (time - ms) > Distance]
results = []
for ms in range(time):
    if ms * (time -ms) > Distance:
        results.append(ms)
        break
        
for ms in range(time, 0, -1):
    if ms * (time -ms) > Distance:
        results.append(ms)
        break

print(results)