long bar(long arg3, long arg4) {
    long temp1 = 5 * arg3;
    long temp2 = arg4 / arg3;
    long temp3 = temp1 / 2;
    long temp4 = 5 * temp2;
    return temp4;
}


 long foo(long arg1, long arg2) {
    long temp1 = 4 * 2; 
    long temp2 = bar(arg1, arg2);
    long temp3 = temp1 + temp2;
    return temp2;
 }


