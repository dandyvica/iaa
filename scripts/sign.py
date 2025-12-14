import sys
x = sys.argv[1]
value = ''.join([ f"\\x{s}" for s in x.split() ])
print(f"\"{value}\"")