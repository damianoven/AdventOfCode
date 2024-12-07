NB. load data
filePath =: '~Projects/adventofcode/2024/day_04/data.txt'
dataS =: > cutopen toJ 1!:1 (< jpath filePath)
NB. find different xmas strings
load 'regex'
rxFunc =: verb define
(# 'SAMX'&rxall y) + (# 'XMAS'&rxall y)
)
horzMatch =: rxFunc"1 dataS
vertMatch =: rxFunc"1 |: dataS
diagMatchA =: > rxFunc each </. dataS
diagMatchB =: > rxFunc each </. |. dataS
+/ horzMatch,vertMatch,diagMatchA,diagMatchB
NB. part 2
isMas =: verb define
0 < # '(SAM|MAS)' rxall y
)
xDashMas =: verb define
(isMas 2{[/. y) *. (isMas 2{[/. |. y)
)
pt2 =: (1 1,:3 3) xDashMas;._3 dataS
+/ , pt2