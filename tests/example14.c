// dont include this gcc does not do an optimization 
long bar(long arg2, long arg3) {
    long temp1 = 5 + 2;
    long temp2 = arg2 / 3;
    long temp3 = temp2 / temp1;
    return temp3;
 }
 
 long foo(long arg1, long arg2) {
    long temp1 = arg1 * arg2;
    long temp2 = temp1 * arg2;
    long temp3 = 6 - 5;
    //testcase : dead assignment elimination: temp4 and temp5 will not be deleted because temp6 has a use.
    // but temp6 will be deleted as that has no use. in that case temp4 and temp5 also should be deleted.
    // iterate untill no changes. this should be handled.
    long temp4 = 6 * 5;
    long temp5 = 6/3;
    long temp6 = bar(temp4, temp5);
    return temp2;
 }
