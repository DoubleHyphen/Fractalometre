#include <cstdint>


typedef uint_fast8_t small_t;


uint64_t u32_to_u64(uint32_t input, small_t offset)
{
    uint64_t result = input;

    result |= result << 16;
    result &= 0x0000FFFF0000FFFF;
    result |= result << 8;
    result &= 0x00FF00FF00FF00FF;
    result |= result << 4;
    result &= 0x0F0F0F0F0F0F0F0F;
    result |= result << 2;
    result &= 0x3333333333333333;
    result |= result << 1;
    result &= 0x5555555555555555;

    return result<<offset;
}

uint32_t u16_to_u32(uint16_t input, small_t offset)
{
    uint32_t result = input;

    result |= result << 8;
    result &= 0x00FF00FF;
    result |= result << 4;
    result &= 0x0F0F0F0F;
    result |= result << 2;
    result &= 0x33333333;
    result |= result << 1;
    result &= 0x55555555;

    return result<<offset;
}

uint16_t u8_to_u16(uint8_t input, small_t offset)
{
    uint16_t result = input;
    
    result |= result << 4;
    result &= 0x0F0F;
    result |= result << 2;
    result &= 0x3333;
    result |= result << 1;
    result &= 0x5555;

    return result<<offset;
}

uint32_t u8_to_u32(uint8_t input, small_t offset)
{
    uint32_t result = input;

    result |= (result<<12);
    result |= (result<<6);
    result &= 0x03030303;
    result |= (result<<3);
    result &= 0x11111111;

    return result<<offset;
}

uint64_t u16_to_u64(uint16_t input, small_t offset)
{
    uint64_t result = input;

    result |= (result<<24);
    result |= (result<<12);
    result &= 0x000F000F000F000F;
    result |= (result<<6);
    result |= (result<<3);
    result &= 0x1111111111111111;    

    return result<<offset;
}

uint64_t u8_to_u64(uint8_t input, small_t offset)
{
    uint64_t result = input;

    result |= (result<<28);
    result |= (result<<14);
    result |= (result<<7);
    result &= 0x0101010101010101;
	
    return result<<offset;
}
 
