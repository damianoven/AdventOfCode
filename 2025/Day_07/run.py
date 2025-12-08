import numpy as np

# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip().split('\n')

# starting beam location
startingLine = np.array([x!='.' for x in allRaw[0]]).astype('longlong')

# every other line has splitters
splitterLines = np.array([[x=='^' for x in y] for y in allRaw[2::2]]).astype('bool')

# splitting evaluation
def get_split(tachOrder,isSplitter):
    # retuns a tuple of (tachOrder,numSplit)
    # evaluate splits
    splitOccured = np.array((tachOrder>0) & isSplitter).astype('longlong')
    splitResult = np.roll(splitOccured*tachOrder,1) + np.roll(splitOccured*tachOrder,-1)
    tachOrder += splitResult
    # splitters block tachyon flow directly beneath
    tachOrder[splitOccured.astype('bool')] = 0;
    # return
    return(tachOrder,np.sum(splitOccured))

# run
currentLine = startingLine
pt1Total = 0
for i in range(len(splitterLines)):
    tmpResult = get_split(currentLine,splitterLines[i])
    currentLine = tmpResult[0]
    pt1Total += tmpResult[1]

# part 1
print(pt1Total)
# part 2
print(np.sum(currentLine))
