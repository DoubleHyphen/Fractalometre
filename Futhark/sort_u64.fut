import "lib/github.com/diku-dk/sorts/radix_sort"
import "lib/github.com/diku-dk/sorts/merge_sort"
import "lib/github.com/diku-dk/cpprandom/random"

module dist = uniform_int_distribution u64 xorshift128plus



let log_block_size = 8
let log_elems_per_thread = 2
let log_elems_per_block = log_block_size + log_elems_per_thread
let bits_each_time = 4
let sort_few_def = merge_sort (u64.<) 

let test_array_small: []u64 = [5, 14, 17, 1, 7, 31, 25, 4, 12, 21, 28, 19, 27, 9, 0, 6, 16, 2, 24, 26, 3, 22, 10, 23, 13, 29, 8, 30, 18, 15, 20, 11]
let test_array_big: []u64 = [127, 102, 128, 80, 60, 109, 234, 119, 143, 15, 23, 140, 218, 93, 1, 10, 61, 96, 147, 77, 120, 142, 18, 145, 71, 31, 158, 104, 209, 191, 216, 171, 134, 151, 232, 159, 19, 111, 172, 78, 187, 254, 169, 67, 112, 165, 125, 251, 175, 2, 37, 48, 46, 12, 130, 243, 49, 199, 70, 110, 194, 135, 178, 39, 148, 75, 213, 206, 34, 195, 192, 229, 204, 252, 95, 123, 41, 210, 107, 81, 141, 90, 131, 190, 69, 100, 88, 179, 7, 182, 113, 40, 152, 87, 86, 138, 248, 6, 236, 79, 94, 116, 132, 233, 155, 14, 144, 44, 188, 198, 255, 185, 181, 3, 197, 163, 219, 217, 56, 122, 160, 173, 20, 222, 33, 73, 84, 108, 202, 207, 228, 74, 27, 89, 249, 106, 196, 103, 224, 150, 105, 220, 208, 22, 28, 238, 157, 91, 8, 250, 29, 242, 54, 126, 30, 83, 114, 53, 51, 162, 200, 133, 215, 221, 230, 9, 170, 247, 186, 99, 45, 124, 146, 32, 253, 235, 64, 35, 212, 149, 16, 59, 154, 36, 72, 174, 24, 76, 25, 4, 50, 164, 0, 92, 223, 85, 183, 189, 43, 245, 168, 97, 176, 201, 58, 231, 66, 205, 167, 57, 129, 63, 47, 52, 17, 21, 156, 65, 214, 62, 68, 166, 226, 244, 115, 42, 225, 55, 193, 121, 161, 180, 101, 118, 38, 184, 26, 136, 211, 13, 237, 239, 117, 240, 82, 246, 153, 137, 203, 227, 177, 139, 98, 241, 11, 5]

let get_bit_group (size: u8)(k: u8)(i: u64): i32 =
    let amt_of_groups = 64/size
    let bits_to_shift = u64.u8 (   ((amt_of_groups-1) - k)*size   )
    let shifted_number = i>>bits_to_shift
    let bitmask = u64.u8 (1<<size) - 1
    let u64_result = shifted_number & bitmask
    in i32.u64 u64_result
    
let get_byte = get_bit_group 8

let get_nybble = get_bit_group 4
    
let histogram_one_thing [n] (k: i32) (is: [n]i32): [k]i32 =
  let bins = replicate k 0
  in reduce_by_index bins (+) 0 is (replicate n 1)

let histogram_per_bitgroup (size: u8)(k: u8)(is: []u64): []u64 =
    let bitgroups = map (get_bit_group size k) is
    in histogram_one_thing (1i32<<(i32.u8 size)) bitgroups |> map u64.i32

let histogram_per_byte = histogram_per_bitgroup 8

let histogram_per_nybble = histogram_per_bitgroup 4

let pad_array (lepb: i32)(xs: []u64): []u64 = 
    let mask = (1<<lepb) - 1
    let unmask = ~mask
    let lgth = length xs
    let padded_array_size = (lgth + mask) & unmask
    let difference = padded_array_size - lgth
    in xs ++ (replicate difference u64.highest)
    
let pad_array_def = pad_array log_elems_per_block

let find_indices (start: u64) (amt: u64) = iota (i32.u64 amt) |> map (u64.i32) |> map (+start) |> map (i32.u64)

let flatmap f xs = loop acc = [] for x in xs do acc ++ f x

let fm_s_helper f sizes blank xs =
  --let sizes = map f_size xs
  --in 
  loop (i, acc) = (0, replicate (i32.sum sizes) blank) for (x, size) in zip xs sizes do
    (i + size, acc with [i:(size+i)] <- f x)
    
let flatmap_sized f sizes xs =
    let (_, result) = fm_s_helper f sizes 0 xs
    in result
    


let sort_by_nybble (sort_few: []u64 -> []u64)(k: u8)(xs: []u64): []u64 =
    --let argh = k |> trace
    let lgth = length xs
    let pad_xs = pad_array_def xs
    let column_size = (lgth + (1<<log_elems_per_block) -1) >> log_elems_per_block
    let grouped_xs = pad_xs
                    --TODO: This doesn't work some times.
                    |> unflatten column_size (1<<log_elems_per_block)
                    |> map (sort_few) 
                    --|> map (merge_sort (<))
                    ---|> trace
                    ---|> break
    
    let gramz = grouped_xs
                |> map (histogram_per_nybble k)
                ---|> trace
                ---|> break
    
    let cumul_gramz = gramz
                        |> transpose
                        |> flatten
                        |> scan (+) 0
                        |> (\x -> [0u64] ++ init x)
                        |> unflatten 16 column_size
                        |> transpose
                        ---|> trace
                        ---|> break
                        
    let gramz = flatten gramz ---|> trace |> break
    let cumul_gramz = flatten cumul_gramz ---|> trace |> break
    let grouped_xs = flatten grouped_xs
    
    --let indices = map2 find_indices cumul_gramz gramz |> flatten |> trace |> break
    let f_i = uncurry find_indices
    let tuples = zip cumul_gramz gramz
    let indices = flatmap_sized f_i (map i32.u64 gramz) tuples ---|> trace |> break
    
    in scatter (copy grouped_xs) indices grouped_xs |> take lgth
    --in xs
  
--let extract_key_lsd 't (radix_bits:u16)(k: u8)(x: t): t = 
    --let bits_kept= x.num_bits - (k*radix_bits)
    --let mask = (1<<bits_kept) - 1
    --in x & mask
    
let extract_key_specialised (k:u8)(x:u64): u64 =
    let bits_kept = u64.u8 <| 64 - (4*k)
    let mask = (1<<bits_kept) -1
    in x & mask
  
let lsd_helper (k:u8)(xs:[]u64): []u64 = 
    let sort_few = merge_sort_by_key (extract_key_specialised k) (<)
    in sort_by_nybble sort_few k xs
    
let radix_sort_lsd (xs:[]u64): []u64 =
    let argh g = lsd_helper (u8.i8 g)
    in loop mixs = xs for i<16 do argh (15-i) mixs
    

  
--let rsort_lsd (xs: []u64): []u64 =
  

let get_rand_array (n: i32) =
    let r = xorshift128plus.rng_from_seed [123]
    let rs = xorshift128plus.split_rng n r
    let (rs', xs) = map xorshift128plus.rand rs |> unzip
    in xs
  
let is_sorted (xs:[]u64) =
    map2 (<) (init xs) (tail xs) |> and
  
let main (n: i32) =
    get_rand_array n 
    |> radix_sort_lsd
    |> is_sorted
