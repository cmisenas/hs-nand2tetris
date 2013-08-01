// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, the
// program clears the screen, i.e. writes "white" in every pixel.

// Put your code here.
(INFINITE_LOOP)
    @SCREEN
    D=A
    @addr
    M=D

    @147456
    D=A
    @counter
    M=D

    @KBD
    D=M
    @IF_TRUE
    D;JNE

        (ERASE)
            @addr
            A=M
            M=0
            @addr
            D=M
            @32
            D=D+A
            @addr
            M=D
            @counter
            MD=M-1
        @ERASE
        D;JGT

    @END
    0;JMP
    (IF_TRUE)

        (DRAW)
            @addr
            A=M
            M=-1
            @addr
            D=M
            @32
            D=D+A
            @addr
            M=D
            @counter
            MD=M-1
        @DRAW
        D;JGT

    (END)
@INFINITE_LOOP
0;JMP
