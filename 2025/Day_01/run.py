import numpy as np
import math

# read file
f = open('input.txt','r');
rawLines = [x.strip() for x in f];
f.close();

# parse lines
turnAmount = np.array([(x.startswith('R')*2-1)*int(x[1:]) for x in rawLines]);

# start at 50
turnAmount = np.insert(turnAmount,0,50);

# cumulative sum, starting at 50
cSum = np.cumsum(turnAmount);

# count when sum mod 100 == 0
exactZero = (cSum % 100) == 0;
print(sum(exactZero))

# count wrap-around 0s
wrapZeros = np.floor(np.abs(turnAmount)/100);
turnAmountUnambiguous = turnAmount - wrapZeros*100*np.sign(turnAmount); # all turns are 0-99 now

# if diff between the two values mod 100 doesn't equal diff mod 100, a wrap occured
# exclude turns to or from 0, which cause a double-count half of the time
modDiff = ((cSum[1:] % 100) - (cSum[:-1] % 100));
diffMod = turnAmountUnambiguous[1:]
diffWrap = modDiff != diffMod;
diffWrap = np.logical_and(diffWrap, np.invert(exactZero[1:])); # moving to zero
diffWrap = np.logical_and(diffWrap, np.invert(exactZero[:-1])); # moving from zero

# final count
print(sum(exactZero) + sum(wrapZeros) + sum(diffWrap));