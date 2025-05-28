/* Simplest example of constant folding */ 
long bar(long arg3) {
    long temp1 = 5 * arg3;
    return temp1;
 }

long foo(long arg1, long arg2) {
    long temp1 = bar(arg1);
    long temp2 = temp1 + arg2;
    return temp2;
}
 
