NB. load data and convert from string to number
dataS =: toJ 1!:1 (< jpath '~Projects/adventofcode/2024/day_01/data.txt')
dataN =: > (_ ". each (cutopen dataS) )
NB. swap rows and columns
dataL =: |: dataN
NB. sort each row
dataI =: /:"1~ dataL
NB. diff the two rows. Root of square bc I can't find abs() function
dataD =: %: (-/ dataI) ^ 2
NB. sum diffs
+/ dataD
NB. count occurances of each number in 2nd column
NB. I think this is creating an NxN array but I don't care
valCounts =: +/"1 (i. >: >./0{dataL) =/ 1{dataL
NB. multiply first column by element at Ith index in second column, then sum
+/ (0{dataL) * ( (0{dataL) { valCounts)
