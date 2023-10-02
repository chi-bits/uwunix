import re
import sys
from enum import Enum # average python L

# lexer

# regexes for things
hex = r"$[0-9a-fA-F]+"
bin = r"%[0-1]+"

class Instr(Enum): # what the fuck is this
    LD = 1 # this is literally so ass
    ST = 2 # what the fuck python
    WRITE = 3 # so youre telling me
    READ = 4 # i have to do " = [number]"
    # oh looky here i need to insert something
    # well i fucking cant
    # unless i shift every single subsequent one

    ADD = 5 # every
    SUB = 6 # single
    MUL = 7 # fucking
    DIV = 8 # time?
    INC = 9 # what the absolute
    DEC = 10 # fuck
    
    AND = 11 # are these enums???
    OR = 12
    XOR = 13 # i could literally
    NOT = 14 # write a fucking script

    FUNC = 15 # that generates these
    CALL = 16 # faster
    JUMP = 17 # than actually doing this
    
    CMP = 18 # by hand
    HALT = 19


class Token:
    def __init__(self, type, val):
        self.type = type
        self.val = val

    def __repr__(self):
        return f"Token({type}, {val})" 

def lex(inp):

    for l in inp:
        instr = l.split(" ")[0]
        
        if instr == "ld":
            


if __name__ == "__main__":
    with open(sys.argv[1], "r") as f:
        infile = f.readlines()

    lex(infile)


