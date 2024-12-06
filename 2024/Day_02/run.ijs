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
+/ (>okChg)*.(>okDec)+.(>okInc)
