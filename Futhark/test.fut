import "lib/github.com/diku-dk/sorts/radix_sort"

let u8_bloat_u64 (x: u8): u64 =
    let x0 = u64.u8 x
    let x1 = x0 | x0<<7
    let x2 = x1 | x1<<14
    let x3 = x2 | x2<<28
    in x3 & 0x0101010101010101

let u8s_encoded_u64 (xs: []u8): u64 =
    let bloateds = map u8_bloat_u64 xs
    let zipped_list = zip bloateds (iota (length xs))
    let shifteds = map (\(a, b) -> a<<(u64.i32)b) zipped_list
    in reduce (|) 0 shifteds

let tuple_to_array (xs: (u8, u8, u8, u8, u8)): []u8 =
    let (a, b, c, d, e) = xs
    in [a, b, c, d, e]

let norm_to_u8 (x: i32)(total: i32): u8 = u8.i32 (x*256/total)

let get_img_dims (xs: [][][3]u8): (i32, i32) =
   (length xs, length <| head xs)

let get_keys_from_image [rows][cols] (image: [rows][cols][]u8): []u64 =
    let n_r = \r -> (norm_to_u8 r rows)
    let n_c = \c -> (norm_to_u8 c cols)
    let pixel_function = (\r c -> [n_r r, n_c c] ++ image[r, c])
    let groups = flatten <| tabulate_2d rows cols pixel_function
    in map u8s_encoded_u64 groups

let pairs (xs: []u64): [](u64, u64) =
    let med = [-1u64] ++ xs
    in zip (init med) xs

let get_test_image (rows: i32)(cols: i32): [][][]u8 = 
    let n_r = \r -> (norm_to_u8 r rows)
    let n_c = \c -> (norm_to_u8 c cols)
    in tabulate_2d rows cols (\r c -> [n_r r, n_c c, 128])

let clz (x: u64): u8 =
    let float_x: f32 = f32.u64 x
    let log2x = f32.log2 float_x
    let floorlog = u8.f32 <| f32.floor log2x
    in 64 - floorlog

let histogram_loop [n] (k: i32) (is: [n]u8): [k]u64 =
  let bucket i = is |> map ((<=i) >-> u64.bool) |> u64.sum
  in map bucket (0..<(u8.i32 k))

let main (x: i32)(y: i32): []u64 =
    get_test_image x y
    |> get_keys_from_image
    |> radix_sort_int u64.num_bits u64.get_bit
    |> pairs
    |> map (\(x, y) -> x^y)
    |> map (\x -> clz x)
    |> map (\x -> x>>3)
    |> histogram_loop 8
