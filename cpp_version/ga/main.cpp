#include <iostream>
#include <string_view>
#include "gray.hpp"

using namespace std;
int main()
{
	long n = 1028;
	long res = toGray(n);
	printBin(n);
	printBin(res);
	printBin(fromGray(res));
	return 0;
}



