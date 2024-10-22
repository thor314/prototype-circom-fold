pragma circom 2.1.9;

include "circomlib/circuits/comparators.circom"; // isZero
include "utils.circom"; // bitwise right shift
include "circomlib/circuits/mux1.circom"; // multiplexer

// Algorithm 1: X •Y
// Input:
// blocks X, Y.
// Output:
// block X •Y.
// multiplication of two blocks in the binary extension field defined by the irreducible polynomial
// 1 + X + X^2 + X^7 + X^128
// computes a “product” block, denoted X •Y
template GhashMul() {
    signal input X[16];
    signal input Y[16];
    signal output out[16];

    // Let R be the bit string 11100001 || 0120. Given two blocks X and Y
    // byte 0xE1 is 11100001 in binary
    // var R[16] = [0xE1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    // 1. Let x0, x1...x127 denote the sequence of bits in X.
    // 2. Let Z0 = 0128 and V0 = Y.
    signal Z[129][16];
    Z[0] <== [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// State accumulator. ie. V[i] is V0 holding 16 bytes
    signal V[129][16];
    V[0] <== Y;
    //       ⎧ Zi               if xi = 0;
    //  Zi+1 ⎨ 
    //       ⎩ Zi ⊕Vi           if xi =1.
    //
    // The V update is isomorphic to multiplying binary polynomial by x
    //
    //       ⎧ Vi >>1           if LSB1(Vi) = 0;
    //  Vi+1 ⎨ 
    //       ⎩ (Vi >>1) ⊕ R     if LSB1(Vi) =1.
    //  
    component bit[16];
    // component z_i_update[128];
    // component mulx[128];
    component single_bit_mul[128];
    component bytesToBits = BytesToBits(16);
    bytesToBits.in <== X;
    signal bitsX[16*8];
    bitsX <== bytesToBits.out;
    for (var i = 0; i < 128; i++) {
        single_bit_mul[i] = single_bit_mul();
        single_bit_mul[i].z_prev <== Z[i];
        single_bit_mul[i].v_prev <== V[i];
        single_bit_mul[i].bit_val <== bitsX[i];
        Z[i + 1] <== single_bit_mul[i].z_new;
        V[i + 1] <== single_bit_mul[i].v_new;
    }
    // 4. Return Z128. 
    out <== Z[128];
}


// if bit value is 0, then Z_new = Z
// if bit value is 1, then Z_new = Z xor V
template Z_UPDATE(n_bytes) {
    signal input Z[n_bytes]; // this is Zero block in first itteration
    signal input V[n_bytes]; // this is Y in first itteration
    signal input bit_val;
    signal output Z_new[n_bytes];

    component mux = ArrayMux(n_bytes);
    mux.sel <== bit_val;
    mux.a <== Z;
    component xorBlock = XORBLOCK(n_bytes);
    xorBlock.a <== Z;
    xorBlock.b <== V;
    mux.b <== xorBlock.out;
    Z_new <== mux.out;
}



// right shift by one bit. If msb is 1:
// then we xor the first byte with 0xE1 (11100001: 1 + X + X^2 + X^7)
// this is the irreducible polynomial used in AES-GCM
template Mulx(n_bytes) {
    signal input in[n_bytes];
    signal output out[n_bytes];

    signal intermediate[n_bytes];

    component blockRightShift = BlockRightShift(n_bytes);
    blockRightShift.in <== in;
    intermediate <== blockRightShift.out;

    component xorByte = XorByte();
    xorByte.a <== intermediate[0];
    xorByte.b <== 0xE1; // 11100001

    // if msb is 1, then we xor the first byte with R[0]
    component mux = Mux1();
    mux.s <== blockRightShift.msb;
    mux.c[0] <== intermediate[0];
    mux.c[1] <== xorByte.out;

    for (var i = 1; i < n_bytes; i++) {
        out[i] <== intermediate[i];
    }
    out[0] <== mux.out;
}

// right shifts 16 bytes by one bit and returns the msb before the shift
template BlockRightShift(n_bytes) {
    signal input in[n_bytes];
    signal output out[n_bytes];
    signal output msb;
    
    signal shiftedbits[n_bytes*8];
    component bytesToBits = BytesToBits(n_bytes);
    for (var i = 0; i < n_bytes; i++) {
        bytesToBits.in[i] <== in[i];
    }
    msb <== bytesToBits.out[n_bytes*8 - 1];

    component BitwiseRightShift = BitwiseRightShift(n_bytes*8, 1);
    BitwiseRightShift.in <== bytesToBits.out;
    shiftedbits <== BitwiseRightShift.out;

    component bitsToBytes = BitsToBytes(n_bytes);
    bitsToBytes.in <== shiftedbits;
    out <== bitsToBytes.out;
}

template single_bit_mul() {
    // previous state
    signal input z_prev[16];
    signal input v_prev[16];
    signal input bit_val;

    // new state
    signal output z_new[16];
    signal output v_new[16];

    component z_update = Z_UPDATE(16);
    component v_update = Mulx(16);
    z_update.Z <== z_prev;
    z_update.V <== v_prev;
    z_update.bit_val <== bit_val;
    z_new <== z_update.Z_new;

    v_update.in <== v_prev;
    v_new <== v_update.out;
}

