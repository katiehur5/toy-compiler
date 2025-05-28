long boo (long arg5, long arg6, long arg7) {
    long temp1 = 5 * arg5;
    long temp2 = arg5 / arg6;
    long temp3 = temp1 / arg7;
    long temp4 = 5 * temp2;
    return temp4;

}

long bar(long arg3, long arg4) {
    long temp1 = 5 * arg3;
    long temp2 = arg4 + arg3;
    long temp3 = temp1 - 2;
    long temp4 = boo(arg3, arg4, temp3);
    return temp4;
}


 long foo(long arg1, long arg2) {
    long temp1 = 4 * 2; 
    long temp2 = bar(temp1, arg2);
    long temp3 = temp1 + temp2;
    return temp3;
 }


