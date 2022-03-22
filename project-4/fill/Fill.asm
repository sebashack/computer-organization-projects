// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(LOOP)
@SCREEN
D = A

@100 // Pointer
M = D

@8191 // Screen size
D=A
@101
M=D // i

D=0
@KBD
D=M

@BLACK
D;JNE

@WHITE
D;JEQ

@LOOP
0;JMP


// SELECT COLOR
(BLACK)
@102
M=-1
@COLOR_LOOP
0;JMP

(WHITE)
@102
M=0
@COLOR_LOOP
0;JMP


// Color screen black
(COLOR_LOOP)
@102
D=M

@100
A=M
M=D

@100
D=M
@1
D=D+A
@100
M=D

@101
M=M-1
D=M

@COLOR_LOOP
D;JGE

@LOOP
0;JMP