# read file
with open('input.txt','r') as f:
    allRaw = f.read().strip();
    f.close();

# make a list of lists for the bank numbers
banks = [[int(y) for y in x] for x in allRaw.splitlines()]

# function to find best digits in a given list
def find_largest_digits(numbers, N):
    # numbers: list of numbers
    # N: how many digits the desired number is made up of
    
    # base case: N=1, so choose max of input list
    if N==1:
        return [max(numbers)]
    else:
        # search for best first digit, then call on sublist to get the rest
        # need to leave at least N-1 digits at the endswith
        firstMax = max(numbers[:-(N-1)])
        idxFirstmax = numbers.index(firstMax)
        return [firstMax] + find_largest_digits(numbers[idxFirstmax+1:],N-1);

# function to construct base 10s number from list
def b10(inList):
    return sum([10**(len(inList)-1-x)*inList[x] for x in range(len(inList))])

# part one
pt1 = [find_largest_digits(x,2) for x in banks]
print(sum([b10(x) for x in pt1]))

# part two
pt2 = [find_largest_digits(x,12) for x in banks]
print(sum([b10(x) for x in pt2]))

