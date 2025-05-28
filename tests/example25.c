long goo(long arg9, long arg10) {
    long temp1 = 5 * arg9;
    long temp2 = arg10 / temp1;
    long temp3 = temp1 / temp2;
    long temp4 = 5 * temp3;
    return temp4;
}

long boo(long arg6, long arg7, long arg8) {
    long temp1 = 5 * arg6;
    long temp2 = arg7 / arg8;
    long temp3 = temp1 / 2;
    long temp4 = goo(temp1, temp2);
    return temp4;
}

long bar(long arg3, long arg4, long arg5, long arg6, long arg7, long arg8, long arg9) {
    long temp3 = boo(arg3, arg4, arg5);
    long temp4 = temp3 + arg6;
    long temp5 = temp3 * arg7;
    long temp6 = temp5 - arg8;
    long temp7 = boo(temp6, arg5, arg4);
    return temp7;
}


 long foo(long arg1, long arg2) {
    long temp1 = arg1 + arg2;	 
    long temp2 = bar(arg1, arg2, arg1, temp1, arg1, arg2, arg1);
    return temp2;
 }


