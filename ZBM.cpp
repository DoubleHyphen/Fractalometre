#include <cstdint>
#include <cinttypes>
#include <cmath>
#include <functional>
#include <array>
#include <algorithm>
#include <bitset>
#include <random>
#include "morton_functions.cpp"
#include "CLZ_functions.cpp"

typedef uintmax_t hugeint_t                             ;
typedef uint_fast8_t small_t                            ;


#define sampler std::function<std::array<coor_t, coor_amt>()>
#define encoder std::function<morton_t(coor_t, small_t)>

template <typename morton_t, typename coor_t, small_t coor_amt>
morton_t get_morton_key(sampler get_next_and_normalise,
                        encoder bloat_and_shift)        {
    
    std::array<coor_t, coor_amt> coor_array             ;
    small_t i                                           ;
    morton_t final_key = 0                              ;
    coor_array = get_next_and_normalise()               ;
    
    for (i = 0; i<coor_amt; i++)                        {
        final_key |= bloat_and_shift(coor_array[i], i)  ;}
    
    return final_key                                    ;}


template <typename morton_t>
void sort_keys(morton_t * morton_key_array,
                size_t sample_size)                     {
    
    std::sort(morton_key_array, 
              morton_key_array + sample_size)           ;}


              
template <typename morton_t>
void extract_neighbouring_clzs(morton_t * morton_key_array,
                                small_t * clz_table,
                                size_t sample_size)     {
    
    auto clz_function = count_leading_zeroes<morton_t>  ;

    for (auto i=0; i<sample_size-1; i++)                
        clz_table[i] = clz_function( morton_key_array[i] ^ morton_key_array[i+1] )               ;}


        
#define get_boxes(r)                                {\
    S[r]++                                          ;\
    amt[r]=(hugeint_t)(iter-prevs[r])               ;\
    squares[r]+=amt[r]*amt[r]                       ;\
    prevs[r]=iter                                   ;}

// When key_bit_amt is known at compile-time, this
// macro can be combined with a switch-statement
// with fall-through to unroll the inner loop.
    
#undef get_boxes

void cumul_histo(small_t * clz_table, 
                 hugeint_t * S,             //output
                 hugeint_t * squares,       //output
                 size_t sample_size,
                 small_t key_bit_amt)                   {
    
    small_t * prevs[key_bit_amt]                        ;
    small_t i                                           ;
    small_t * iter                                      ;
    small_t * limit = clz_table + sample_size - 1       ;
    hugeint_t amt[key_bit_amt]                          ;

    // Initialisations:
    for (i=0; i<key_bit_amt; i++)                       {
        S[i]=1                                          ;
        squares[i]=0                                    ;
        prevs[i]=clz_table                              ;}
    
    // Results:
    for (iter = clz_table; iter < limit; iter++)        {  
        for (i = *iter; i<key_bit_amt; i++)             {
            S[i]++                                      ;
            amt[i] = (hugeint_t)(iter-prevs[i])         ;
            squares[i] = amt[i]*amt[i]                  ;
            prevs[i] = iter                             ;}}}
            




enum fd_options{all_points, equal_division}         ;

float get_fd_lin_fit(hugeint_t * S, 
                    small_t key_bit_amt, 
                    small_t coor_bit_amt,
                    size_t sample_size,
                    small_t maxq = 1024, 
// maxq should only be used when some coordinate has
// been normalised to a larger span, eg 0-200 -> 0-256.
// maxq is then equal to floor(log2(top)), or 7 in this case.
                    fd_options selection = equal_division){
// Seeing as this function, as well as the following one, 
// both run in (practically) O(1) time, their optimisation
// was not deemed worthwhile.
    
        
    int i=0, max_reas=0;
    float num=0, denom=0, avx=0, avy=0, maxlog, fa  ;
    float *logs                                     ;
    
    int point_amt = (selection == equal_division)   ?
                    coor_bit_amt                    :
                    key_bit_amt                     ;
                    
    small_t step = (selection == equal_division)    ?
                   key_bit_amt/coor_bit_amt         :
                   1                                ;
        
    logs = new float[point_amt + 1]                 ;
    maxlog=0.9*log2(sample_size)                    ;
    
    //Get the maximum point before the cut-off
    for (i=point_amt; i>0; i--)                     {
        logs[i]=log2(S[(i-1)*step])                 ;
        if (logs[i]>maxlog)                         {
            continue                                ;}
        else if (i>maxq)                            {
            continue                                ;}
        if (max_reas==0)                            { 
            max_reas=i                              ;}
        avy += logs[i]                              ;
        avx += i                                    ;}
    
    //Use linear fitting to get the FD
    avy/=max_reas                                   ;
    avx/=max_reas                                   ;
    for (i=max_reas; i>=0; i--)                     {
        num+=(logs[i]-avy)*(i - avx)                ;
        denom+=(i - avx)*(i - avx)                  ;}
    fa=num/denom                                    ;
    delete logs                                     ;
    return fa                                       ;}



enum lacun_options {all_boxes, populated_boxes}     ;

void get_lacun(hugeint_t * squares,
               hugeint_t * S,
               long double * Lacun,        //output
               size_t sample_size, 
               small_t key_bit_amt, 
               small_t coor_amt, 
               lacun_options selection = populated_boxes){
    
    hugeint_t boxamt = 1                ;
    small_t i                           ;
    long double temp                    ;
    
    
    for (i=0; i<key_bit_amt; i++)       {
        boxamt=(selection == all_boxes) ?
                boxamt<<coor_amt        :
                S[i]                    ;
        Lacun[i]=squares[i]             ;
        temp = boxamt / sample_size     ;
        Lacun[i]*=temp                  ;
        Lacun[i]/=sample_size           ;
        Lacun[i]--                      ;}}
        
        
template <class T, typename morton_t>
float get_fd_and_lacun(T set, long double * Lacun)  {
    size_t sample_size = set.get_sample_size()      ;
    morton_t * morton_key_array                     ;
    hugeint_t * amts_of_populated_boxes             ;
    hugeint_t * sums_of_population_squares          ;
    small_t * clz_array                             ;
    small_t coor_amt = set.get_coor_amt()           ;
    small_t coor_bits = set.get_coor_bits           ;
    small_t key_bits = sizeof(morton_t)*CHAR_BIT    ;
    
    assert (sizeof (morton_t) >= coor_amt*coor_bits);
    
    morton_key_array = new morton_t(sample_size)    ;
    
    for (auto i = 0; i<sample_size; i++)            {
        morton_key_array[i] = get_morton_key(set.sampler, set.encoder);}
        
    sort_keys(morton_key_array, sample_size)        ;
    clz_array = new small_t(sample_size-1)          ;
    extract_neighbouring_clzs(morton_key_array, clz_array, sample_size);
    delete[] morton_key_array                       ;
    
    amts_of_populated_boxes = new hugeint_t(key_bits);
    sums_of_population_squares = new hugeint_t(key_bits);
    
    cumul_histo(clz_table, amts_of_populated_boxes, sums_of_population_squares, sample_size, key_bits);
    delete[] clz_table;
    
    get_lacun(sums_of_population_squares, amts_of_populated_boxes, Lacun, sample_size, key_bits, coor_amt);
    delete[] sums_of_population_squares             ;
    
    return get_fd_lin_fit (S, key bit, coor bit amt, sample_size);}
