from target.debug.wrapper import HashSet

set = HashSet()

set.add(1)

print(f"has 0: {0 in set}")
print(f"has 1: {1 in set}")

set.add(2)

for i in set:
    print(f"found {i}")

print(len(set))