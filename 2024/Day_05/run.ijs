NB. load data
filePath =: '~Projects/adventofcode/2024/day_05/data.txt'
dataB =: cutopen toJ 1!:1 (< jpath filePath)
list1 =: (> +/ each '|' = each dataB) # dataB
list1 =: ". each ('|',',')&stringreplace each list1
list2 =: ". each (-. > +/ each '|' = each dataB) # dataB
NB. NxN truth table as vector
tSize =: >./ , > list1
tt =: (tSize^2) $ 0
getIdx =: 3 : '_2 +/\ (((# y)$(tSize,1)) * (<: y))'
tt =: 1 (> getIdx each list1)} tt
combos =: ((<&{.&])\. each list2) ,: each ((<&}.&])\. each list2)
combos =: |: each each (_1}. ((0{[) ,:/ each (1{[))) each combos
comboIdx =: getIdx&, each each combos
comboVld =: (tt{~]) each each comboIdx
rowVld =: > <./ each > each <./ each each comboVld
list2Ctr =: > ((<.&(2%~#)){]) each list2
+/ rowVld # list2Ctr
NB. part 2
]list2
]comboVld
NB.swapIdx =: |. each > each (-. each) each comboVld