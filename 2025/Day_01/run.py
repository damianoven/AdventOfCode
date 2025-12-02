import numpy as np

# read file
f = open('input.txt','r');
rawLines = [x.strip() for x in f];
f.close();

# parse lines
turnAmount = [(x.startswith('R')*2-1)*int(x[1:]) for x in rawLines];

# cumulative sum, starting at 50
cSum = 50 + np.cumsum(turnAmount);

# count when sum mod 100 == 0
numZeros = sum((cSum % 100) == 0);
print(numZeros)
