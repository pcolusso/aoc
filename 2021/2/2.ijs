#! /opt/homebrew/bin/jconsole
NB. Found this alternative way to read a file. It's loaded, and then EXECUTED (when called),
NB.   so we can define what to DO for each command, and the numbers are numbers.
NB.   SUPER slick, but i guess it's eval fuckery so probably not a pattern we can take out of J

NB. also I suppose fread can be used as an alternative to 1!:1

NB. Source: https://github.com/Termina1/aoc2021/blob/main/day2/day2.ijs

example =. 'm' fread 'example.txt'
input   =. 'm' fread 'input.txt'
p       =. {{ echo y }} NB. Now i will inject some ruby, hehehe (analog for puts shorthand)

NB. This is going to get confusing, as x & y are used as params.
NB. Horizontal is going to be col 1, vertical is col 2.

NB. y is used to indicate the LHS.
NB. Double curlys is a way to define a lambda I guess.
NB. The brackets are to define a tuple, and represent the change to make.

forward =. {{ (+y, 0) }} 
up      =. {{ (0, -y) }} 
down    =. {{ (0, +y) }} 

NB. Now, we execute the input. The above functions will be used to create a matrix of the changes
NB.  we sum these, to get the final position.

p 'final example coords {x, y}'
p +/ ". example

NB. Then mulitply them for the answer.

p 'example 1 answer'
p */ +/ ". example

p 'part 1 answer'
p */ +/ ". input

NB. ==Part 2==

NB. Guess we could do the same we did in ruby, mix in an aim and then calculate.

forward =. {{ (+y, 0) }} 
up      =. {{ (0, -y) }} 
down    =. {{ (0, +y) }} 

exit''
