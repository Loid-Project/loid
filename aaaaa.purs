

HIIII bye

Take in words: strings between spaces excluding whitespaces
A line break \n is also a word

Give a word to a lexer function

it'll see what it is and lex it (return a Token[] type)

the Token types are then constructed into an array

slower, but easier to debug and use.


-13 -> [Token of -13]

21 -> [Token of 21]

12-32 -> [Token of 12, Token of subtract, Token of 32]



for negative numbers

a -b == a-b == a - b | this is math 

 - | NEG op 

3 tokens


- 123412343215



NEG 12345566724

-a 

a-b  

if (a - 1) 

NUMBER NUMBER


11 - 10

advance(){
    skip whitespaces
    cursor++
}

is_last_token_number = true/false

11 -10  | number neg number


f(5) -f(4)

if char == -
	if digit
else
	

number number











