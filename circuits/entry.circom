pragma circom 2.1.9;

include "gmul.circom";

template GhashMulFoldEntry() {
    signal input step_in[1];
    signal input X[1];
    signal input Y[1];
    // signal output out[1];
    signal output step_out[1];

    // out <== _GhashMul()(X,Y);

    step_out[0] <== step_in[0] ;
}


template _GhashMul(){
    signal input X;
    signal input Y;
    signal output out;

    out <== [0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0];
}

// component main = GhashMul();
component main { public [step_in] } = GhashMulFoldEntry();
