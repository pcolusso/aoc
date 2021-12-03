NB. reading a file into a variable, this creates a string however and not yet usable.
NB. note the filename needs to be boxed.

NB. TODO: Having a tough time getting the files parsed by J itself. For now, just cutting the foiles and manually pasting in.

example =. 1!:1 < 'example.txt'
input =. ". ;._2 (1!:1 < 'input.txt')

NB. Okay okay, found a way to parse.
NB. ". parses strings into ints, and :. is cut. the _2 does TBA

example =. ". ;._2 example

NB. Note this requires the LAST character to be the delimeter, which is a newline at the end of the file.

NB. inspired from jitwit's #1 solution

NB. 2 < example applies less than over each, and all of them are, so we get ones
NB. the reduce operatior is implied, so it's the same as 2 </ example

NB. However, the infix operator allows us to work over a window
NB. which is the other slash.
NB. This is really handy in part two, as the following

3 +\ example 

NB. Shows the windows used, (actually im unsure why the + is needed, we're not summing)
NB. and then using redice, we get the sums

3 +/\ example

NB. Then combine with the previous...

+/ 2 </\ 3 +/\ example

