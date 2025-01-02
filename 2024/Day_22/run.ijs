NB. load data
filePath =: '~Projects/adventofcode/2024/day_22/data.txt'
data =: ; ". each cutopen toJ 1!:1 (< jpath filePath)
NB. custom function to calculate next iteration
prn =: 0 (+ m. 16777216) ] NB. add 0 modulo that number
mix =: 22 b. NB. 22=16+6 NB. 6=0b0110=XOR truth table
pt1 =: verb : 'prn (64*y) mix y'
pt2 =: verb : 'prn (<.y%32) mix y'
pt3 =: verb : 'prn (2048*y) mix y'
inc =: verb : 'pt3 pt2 pt1 y'
incN =: {{
for. i. x do.
 y =. inc y
end.
]y
}}
NB. run 2000 times and sum result
+/ 2000 incN data
NB. part two
NB. construct array of prices
appN =: {{
out =. y
for. i. x do.
 out =. out , inc {:out
]out
end.
}}
tabl =: 0 (+ m.10) 2000 (appN"0) data
NB. construct base 20 number of 4 diffs+10
tablDiff =: (2 -~/\ ])"1 tabl
encDiff =: verb : '+/ (y+10) * (20^i.#y)'
diffCode =: (4 encDiff\ ])"1 tablDiff
seqPrice =: 0 ,~"1 (4 }."1 tabl) NB. append 0s so that non-matches return 0
NB. go through each possible code and check what the result is
prcIdx =: diffCode (i."1) i.(encDiff 9 9 9 9)
sllPrc =: prcIdx {"(1 1) seqPrice
>./ +/ sllPrc