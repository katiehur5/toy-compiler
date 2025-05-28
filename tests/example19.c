long bar(long arg1, long arg2)
{
    long temp1 = 7 + 5;
    long temp2 = 4 * 6;
    long temp3 = arg1 + temp1;
    long temp4 = arg1 * temp2;
    long temp5 = temp4 - temp3;
    return temp5;
}

long foo(long arg1, long arg2) 
{
    long temp1 = arg2 + 2;
    long temp2 = bar(arg1, arg2);
    long temp3 = temp1 - temp2;
    return temp1;
}
