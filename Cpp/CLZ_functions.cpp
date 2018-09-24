#include <cstdint>
#include <functional>
#include <climits>


typedef uint_fast8_t small_t;

#define input_bits sizeof(input)*CHAR_BIT

template <class T>
small_t count_leading_zeroes (T input) {throw std::bad_function_call( "CLZ function undefined for this data type." );}

template <>
small_t count_leading_zeroes<unsigned long long> (unsigned long long input) 
{return input==0? input_bits : __builtin_clzll(input);}

template <>
small_t count_leading_zeroes<unsigned long> (unsigned long input) 
{return input==0? input_bits : __builtin_clzl(input);}

template <>
small_t count_leading_zeroes<unsigned int> (unsigned int input) 
{return input==0? input_bits : __builtin_clz(input);} 

//...is there a __builtin_clz for shorts?
template <>
small_t count_leading_zeroes<unsigned short int> (unsigned short int input) 
#if UINT_MAX>>16 == USHRT_MAX
{return input==0? input_bits : (__builtin_clz(input)-16);} // Verified to work correctly for all possible values of a 16-bit short.
#elif UINT_MAX == USHRT_MAX
{return input==0? input_bits : __builtin_clz(input);}
#endif

#undef input_bits
