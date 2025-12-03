import numpy as np
import math

# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip();
    f.close();

# split on ','
rangeText = [x.strip() for x in allRaw.split(',')];
rangeStartStop = [[int(y) for y in x.split('-')] for x in rangeText];

def gen_sym_nums(N,P,lowerBound,upperBound):
    # N: number of digits in the number
    # P: number of digits in the pattern
    # lowerBound: do not generate numbers below this
    # upperBound: do not generate numbers above this
    if (N % P) != 0:
        return [] # no symetric numbers
    K = int(N//P); # K=number of patterns to fill number
    # clamp lower and upper bound
    lowerBound = min(max(lowerBound,10**(N-1)),(10**N)-1);
    upperBound = max(min(upperBound,(10**N)-1),10**(N-1));
    # get MSP (most significant pattern) of lower bound
    lowerPats = np.floor(lowerBound/(10**(np.array(range(K-1,-1,-1))*P))) % (10**P);
    upperPats = np.floor(upperBound/(10**(np.array(range(K-1,-1,-1))*P))) % (10**P);
    # decide which pattern to start on
    startPat = lowerPats[0]
    for k in range(K):
        if lowerPats[k] > startPat:
            startPat += 1;
            break;
        elif lowerPats[k] < startPat:
            break;
    # decide which pattern to end on
    endPat = upperPats[0]
    for k in range(K):
        if upperPats[k] < endPat:
            endPat -= 1;
            break;
        elif upperPats[k] > endPat:
            break;
    # generate patterns
    pats = list(range(int(startPat),int(endPat)+1));
    outVals = [sum( [x*(10**int(y*P)) for y in list((range(0,K)))]) for x in pats];
    return outVals

# loop through ranges and generate symetric numbers for each
runningTotalPart1 = 0;
runningTotalPart2 = 0;
for r in rangeStartStop:
    startPwr = int(np.floor(np.log10(r[0])))
    stopPwr = int(np.floor(np.log10(r[1])))
    # loop though each decade
    for n in range(min(startPwr,stopPwr),max(startPwr,stopPwr)+1):
        # part 1
        if (n%2)==1:
            valid1 = gen_sym_nums(n+1,(n+1)/2,r[0],r[1])
            add1 = int(sum(valid1))
            runningTotalPart1 += add1
        # part 2
        validList2 = [];
        for p in range(1,n+1):
            valid2 = gen_sym_nums(n+1,p,r[0],r[1])
            validList2 = list(set(validList2).union(valid2)); # prevent double counting
        add2 = int(sum(validList2))
        runningTotalPart2 += add2

print(runningTotalPart1)
print(runningTotalPart2)

