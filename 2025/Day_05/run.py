import numpy as np

# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip()

# first half is ranges
validRanges = np.array([[int(y) for y in x.split('-')] for x in allRaw.split('\n\n')[0].splitlines()]).astype('longlong')

# second half is test values
testValues = np.array([int(x) for x in allRaw.split('\n\n')[1].splitlines()]).astype('longlong')

# check if test values are within start or end of any ranges
def is_in_range(start,stop,query):
    rightOfStart = (np.reshape(start,(-2,1)) - np.reshape(query,(1,-2))) <= 0
    leftOfEnd = (np.reshape(stop,(-2,1)) - np.reshape(query,(1,-2))) >= 0
    return rightOfStart & leftOfEnd

# part 1
print(sum(np.any(is_in_range(validRanges[:,0],validRanges[:,1],testValues),0)))

# part 2

# if any starting value is itself within a range, move it
# to the end of that range +1
startIsInRange = is_in_range(validRanges[:,0],validRanges[:,1],validRanges[:,0])
newStartingValue = startIsInRange * (np.reshape(validRanges[:,1],(-2,1))+1) 
newStartingValue -= newStartingValue * np.identity(len(newStartingValue)).astype('longlong') # remove diagonal (self range)
newStartingValue = np.max(newStartingValue,0)
validRanges[newStartingValue>0,0] = newStartingValue[newStartingValue>0];

# add up ranges as ints
print(sum([int(validRanges[x,1])-int(validRanges[x,0])+1 for x in range(len(validRanges))]))
