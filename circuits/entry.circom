pragma circom 2.1.9;

include "gmul.circom";

template GhashMulFoldEntry() {
    signal input step_in;
    signal input X[16];
    signal input Y[16];
    signal output out[16];
    signal output step_out;

    out <== GhashMul()(X,Y);

    step_out <== step_in ;
}

// component main = GhashMul();
component main { public [step_in] } = GhashMulFoldEntry();
