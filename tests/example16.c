// Example for deadassign: multiple operand in one expression


 long boo(long arg1, long arg2, long arg3) {
    //testcase: multiple expressions
    long temp1 = 5 * arg1;
    // testcase : const assign
    long temp2 = 5;
    //test case : variable assign
    // test case: expression
    long temp3 = temp2 / temp1;
    long temp4 = 5 * 2;
    
    // test case : dead
    long temp5 = temp4;
    //testcase: constant
    long temp6 = 6;
    return temp3;
 
}

long bar(long arg1, long arg2) {
    long temp1 = arg1 * arg2;
    // testcase: handling usages in function calls
    long temp2 = boo(temp1, arg1, arg1);
    long temp3 = temp1 * arg2;
    return temp2;
 }

long foo(long arg1, long arg2) {
    long temp1 = arg1 * arg1;
    long temp2 = bar(arg2, arg1);
    long temp3 = temp1 * arg1;
    return temp1;
 }
