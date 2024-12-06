NB. load data and convert from string to number
dataS =: toJ 1!:1 (< jpath '~Projects/adventofcode/2024/day_02/data.txt')
NB. keep data boxed since different number of elements per row
dataB =: _ ". each cutopen dataS
NB. diff between each element
diff =: (}: - }.) each dataB
diffAbs =: diff *each (*each diff)
rowLen =: $ each diff
NB. check part 1 rules and sum
okDec =:        rowLen  = each +/ each *each diff
okInc =: (-each rowLen) = each +/ each *each diff
okChg =: 4>each >./ each diffAbs
okPt1 =: (#diff) $, (>okChg)*.(>okDec)+.(>okInc)
+/ okPt1
NB. part 2
NB. remove 1 item from each location in each list
dataT =: 1]\. each dataB
diffT =: (}: - }.)"1 each dataT
diffAbsT =: diffT *each (*each diffT)
rowLenT =: 1{ each $ each diffT
okIncT =:        rowLenT  = each +/"1 each *each diffT
okDecT =: (-each rowLenT) = each +/"1 each *each diffT
okChgT =: 4>each >./"1 each diffAbsT
okPt2 =: > >./ each okChgT *. each okIncT +. each okDecT
+/ okPt1 +. okPt2