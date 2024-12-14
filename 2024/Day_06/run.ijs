NB. load data
filePath =: '~Projects/adventofcode/2024/day_06/data.txt'
dataS =: > cutopen toJ 1!:1 (< jpath filePath)
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
visited =. (4,$x) $ 0
infLoop =. 0
NB. compute distance to next turning point (or edge)
rTable =. (<:&('#'i.~]))\."1 x
lTable =. - |."1 (<:&('#'i.~]))\."1 |."1 x NB. reverse
dTable =. |: (<:&('#'i.~]))\."1 |: x NB. transpose
uTable =. - |: |."1 (<:&('#'i.~]))\."1 |."1 |: x NB. transpose and reverse
NB. create brick of distance tables
dLut =. ((uTable,:rTable),dTable),lTable
NB. iterate
while. (-. (isEdge y)) *. (maxIts>0) *. (-. infLoop) do.
  NB. find new position
  newPos =. (((<dirIdx,y){dLut)+((dirIdx{pUpdateIdx){y)) ((dirIdx{pUpdateIdx)})y
  NB. udpate path to be 'X'
  x =. 'X' (<(<"1 newPos + (*y-newPos) * i."0 y - newPos))} x
  NB. check if we've been here before facing the same direction
  infLoop =. (<(dirIdx,newPos)){visited
  NB. update loop variables
  visited =. 1 (<(dirIdx,newPos))} visited
  dirIdx =. dirIdx addMod4 1 NB. increment mod 4
  y =. newPos
  posList =. posList , y
  maxIts=. <: maxIts
end.
](<x),(<infLoop),(<maxIts)
}}
NB. run iteration
dataX =: 'X' (<pos)} dataS
result =: dataX runSim pos
NB. count Xs
+/ +/&('X'=[)"1 >0{result
NB. part two
idxTest =: |: (,(i.0{$dataS) +/ (1{$dataS)$0) ,: ,((0{$dataS)$0) +/ i.1{$dataS
isInf =: {{>1{(('#' (<y)} dataX) runSim pos)}}
infList =: isInf"1 idxTest
(+/ infList) - (isInf pos)
NB.isOF =: {{>2{(('#' (<y)} dataX) runSim pos)}}
NB.ofList =: isOF"1 idxTest
NB.+/ 0=ofList