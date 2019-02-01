#ifndef __WWS_GRAY_CODE_H__
#define __WWS_GRAY_CODE_H__

#include <iostream>

template<typename T>
T toGray(T b)
{
	T K = ((T)1);
	size_t len = sizeof(T) * 8 - 1;
	size_t j = len;
	T f = K & (b >> j);
	T res = f << j;

	for(;j > 0;j--)
	{
		T nf = (K & (b >> ( j - 1 )));
		res |= ((f ^ nf) << (j - 1));
		f = nf;	
	}

	return res;
}

template <typename T>
T fromGray(T g)
{
	T K = ((T)1);
	size_t len = sizeof(T) * 8 - 1;
	size_t j = len;
	T res = (K & (g >> j)) << j;

	for(;j > 0;j--)
	{
		T nf = (K & (res >> j));
		T gi = (K & (g >> (j - 1)));
		res |= ((gi ^ nf) << (j - 1));
	}
	return res;
}

template<typename T>
void printBin(T b)
{
	size_t len = sizeof(T) * 8 - 1;
	T K = ((T)1) << len;
	for(int i = len;i >= 0;i--)
	{
		if( (K & (b << (len - i))) == 0)
		{
			printf("%c",'0');
		}else{
			printf("%c",'1');
		}
		if( i % 4 == 0 )
			printf(" ");
	}
	printf("\n");
}
#endif //__WWS_GRAY_CODE_H__
 
