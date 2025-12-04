import numpy as np
from scipy import signal

# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip()
    f.close()

# make into 0/1 array
grid = np.array([[int(y=='@') for y in x] for x in allRaw.splitlines()])

# function to mark rolls for removal
def find_valid_rolls(grid):
    # convolve with donut kernal
    kernal = np.array([[1,1,1],[1,0,1],[1,1,1]])
    result = signal.convolve2d(grid,kernal,boundary='fill',fillvalue=0,mode='same')
    return (result<4) & grid

# part one
print(sum(sum(find_valid_rolls(grid))))

# part two
totalRemoved = 0
while True:
    nextRemove = find_valid_rolls(grid)
    amountRemove = sum(sum(nextRemove))
    if amountRemove == 0:
        break
    # remove these rolls and iterate again
    grid -= nextRemove
    totalRemoved += amountRemove
    print(f"Total removed: {totalRemoved}")
