NB. load data
filePath =: '~Projects/adventofcode/2024/day_06/data.txt'
dataS =: > cutopen toJ 1!:1 (< jpath filePath)
NB. compute distance to next turning point (or edge)
rTable =: (<:&('#'i.~]))\."1 dataS
lTable =: - |."1 (<:&('#'i.~]))\."1 |."1 dataS NB. reverse
dTable =: |: (<:&('#'i.~]))\."1 |: dataS NB. transpose
uTable =: - |: |."1 (<:&('#'i.~]))\."1 |."1 |: dataS NB. transpose and reverse
NB. create brick of distance tables
dLut =: ((uTable,:rTable),dTable),lTable
NB. get starting index
cIdx =: ('^'i:~])"1 dataS
pos =: (I. cIdx<#0{dataS) , (I. cIdx<#0{dataS){cIdx
NB. function to check whether the current position is at the edge
isEdge =: verb : '-. (0<0{y) *. ((<:#dataS)>0{y) *. (0<1{y) *. ((<:#0{dataS)>1{y)'
NB. walk around until we hit an edge
runSim =: {{
dirIdx =. 0
addMod4 =. +m.4
pUpdateIdx =. 0 1 0 1 NB. up/right/down/left
posList =. y NB. for debugging
maxIts =. 1e4 NB. for safety
while. (-. (isEdge y)) *. maxIts>0 do.
  NB. find new position
  newPos =. (((<dirIdx,y){dLut)+((dirIdx{pUpdateIdx){y)) ((dirIdx{pUpdateIdx)})y
  NB. udpate path to be 'X'
  x =. 'X' (<(<"1 newPos + (*y-newPos) * i."0 y - newPos))} x
  NB. update loop variables
  dirIdx =. dirIdx addMod4 1 NB. increment mod 4
  y =. newPos
  posList =. posList , y
  maxIts=. <: maxIts
end.
x
]x
}}
NB. turn '^' to 'X' as the iteration skips current location
dataS =: 'X' (<pos)} dataS
NB. run iteration
dataX =: dataS runSim pos
NB. count Xs
+/ +/&('X'=[)"1 dataX
