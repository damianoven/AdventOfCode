import numpy as np

# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip().split('\n')

# create np array
numPoints = len(allRaw)
loc = np.array([[int(y) for y in x.split(',')] for x in allRaw]).astype('longlong')
# reshape so dim0=index, dim1=empty, dim2=coordinate
loc = np.reshape(loc,(-1,1,3))
# create copy with dim0=empty, dim1=index, dim2=coordinate
locT = np.reshape(loc,(1,-1,3))
# subtract, square, sum to get matrix of squared distances
distSq = np.sum((loc-locT)**2,axis=2)
# remove upper triangular since they are repeats
distSq = np.triu(distSq)
# indices corresponding to each distance
i1,i2 = np.meshgrid(range(numPoints),range(numPoints),indexing='ij')
# flatten distances and indices
distSq = np.reshape(distSq,(-1))
i1 = np.reshape(i1,(-1))
i2 = np.reshape(i2,(-1))
# sort in ascending order
idxSort = np.argsort(distSq)
distSq = distSq[idxSort]
i1 = i1[idxSort]
i2 = i2[idxSort]
# the first real connection is after the lower triangular of zeros
ptr = (numPoints+1)*numPoints//2

# connection function
def make_connection(circuitList,a,b,currentCircuitId):
    # makes a connection between index a and b in circuitList
    # returns (circuitList,currentCircuitId)
    aConn = circuitList[a]>0
    bConn = circuitList[b]>0
    if aConn and bConn:
        # both are already in a circuit, need to join them
        circuitList[np.logical_or(circuitList==circuitList[a], circuitList==circuitList[b])] = currentCircuitId
        currentCircuitId += 1
    elif aConn or bConn:
        # one is not connected, add items
        thisCircuitId = circuitList[a]+circuitList[b] # one of these is 0
        circuitList[a] = thisCircuitId
        circuitList[b] = thisCircuitId
    else:
        # neither are in a circuit, create a new one
        circuitList[a] = currentCircuitId
        circuitList[b] = currentCircuitId
        currentCircuitId += 1
    return (circuitList, currentCircuitId)

# part 1
circuitList = np.zeros(numPoints).astype('longlong')
currentCircuitId = 1
numIts = 1000;
for i in range(ptr,ptr+numIts):
    (circuitList, currentCircuitId) = make_connection(circuitList,i1[i],i2[i],currentCircuitId)
circuitSizes = [sum(circuitList==x) for x in range(1,currentCircuitId)]
circuitSizes = np.sort(circuitSizes)
print(np.prod(circuitSizes[-3:]))

# part 2
circuitList = np.zeros(numPoints).astype('longlong')
currentCircuitId = 1
ii = ptr
while True:
    (circuitList, currentCircuitId) = make_connection(circuitList,i1[ii],i2[ii],currentCircuitId)
    # check if we're done
    if len(np.unique(circuitList))==1:
        break
    ii += 1

print(loc[i1[ii],0,0]*loc[i2[ii],0,0])
    
