// zkML simulation: y = x * 2

template Double() {
    signal input x;
    signal output y;

    y <== x * 2;
}

component main = Double();
