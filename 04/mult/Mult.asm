// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[3], respectively.)

// Put your code here.
// r2 = r0 * r1
// var r2 = r1;
// var i = 1;
// while(i < r0){
//   r2 += r1;
//   i++;
// }
// @ -> stored A register
// M -> memory in A or RAM[A]
// D -> just another register
    @0
    D=A
    @2
    M=D
    @i
    M=1
(LOOP)
    @1
    D=M
    @i
    D=M-D
    @END
    D;JGT
    @0
    D=M
    @2
    M=M+D
    @i
    M=M+1
    @LOOP
    0;JMP
(END)
    @END
    0;JMP
