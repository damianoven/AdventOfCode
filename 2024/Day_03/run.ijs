NB. load data
dataS =: toJ 1!:1 (< jpath '~Projects/adventofcode/2024/day_03/data.txt')
NB. remove LF
dataS =: (LF,' ') stringreplace dataS
NB. get uncorrupted matches with regular expression
load 'regex'
rxMatch =: '(?<=mul)\(\d*,\d*\)' rxall dataS
+/ */"1 > ". each rxMatch
NB. part 2
NB. split into section starting with do()
doSections =: '(?<=do\(\)|\A).*?(?=don''t\(\)|\Z)' rxall dataS
doString =: (*/ $ >doSections) $, >doSections
rxMatch2 =: '(?<=mul)\(\d*,\d*\)' rxall doString
+/ */"1 > ". each rxMatch2