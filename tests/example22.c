long boo(long arg5, long arg6) {
    long temp1 = 5 * arg5;
    long temp2 = arg6 / arg5;
    long temp3 = temp1 / 2;
    long temp4 = 5 * temp2;
    return temp4;
}

long bar(long arg3, long arg4) {
    long temp3 = boo(arg3, arg4);
    return temp3;
}


 long foo(long arg1, long arg2) {
    long temp2 = bar(arg1, arg2);
    return temp2;
 }


