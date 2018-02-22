# Fractalometre
A collection of functions to help with fractal analysis. They are all based on the "Z-box merging" method described in the paper of the same name.

This code is very much a work in progress. Contributions are welcome.



### Explanation of the program:
#### `morton_functions.cpp`:
This file provides sample Morton-encoding functions for every possible combination of `uint8_t`, `uint16_t`, `uint32_t` and `uint64_t`. The encoding happens by inserting an equal number of zeroes before each bit, as described in https://en.wikipedia.org/wiki/Z-order_curve. The functions also shift the resulting Morton Key left as needed, for convenience.

#### `CLZ_functions.cpp`:
This file provides sample functions for counting the amount of Leading Zeroes of a number, ie the number of zeroes that appear before the first 1-value bit is encountered. If the input equals zero, these functions output the size of the input in bits; otherwise, they function as a simple wrapper around `__builtin_clz`.

#### `ZBM.cpp`:
This file contains the main functions that, when combined, comprise the skeleton of the Z-Box Merging algorithm. Optimisation was not a priority; rather, the priorities were readability and ease of translation into other languages. The former explains why `clz_array` exists as a separate array. The latter explains why some high-level features of C++ (such as `vector<morton_t>` instead of `morton_t *`) were not utilised.

##### Explanation of functions:
**`get_morton_key`** accepts a function that outputs an array of coordinates (eg an iterator), and a function that can "bloat" each co-ordinate to get its Morton (Z-Order) encoding. It outputs the Morton Key that corresponds to this array of coordinates.
As mentioned above, sample encoding functions are provided  in the `morton_functions.cpp` file.

**`sort_keys`** is self-explanatory. Currently this is just a wrapper around `std::sort`.

**`extract_neighbouring_clzs`** accepts the array of Morton Keys, and outputs the amount of leading identical bits between each consecutive pair of keys. This happens by `xor`ing them and counting the Leading Zeroes, as described above. The resulting array is one element shorter than the aforementioned array of Morton Keys.

**`cumul_histo`** accepts the array of leading identical bits (see above) and calculates how many populated boxes there are in each scale (`S[i]`) as well as the sum of the squares of all box populations in each scale (`squares[i]`). The former is used in the calculation of the fractal dimension, and both are used (mainly the latter) in the calculation of lacunarity.

**`get_fd_lin_fit`** calculates the fractal dimension by taking the cumulative histogram as a log-log scale, ignoring points that occur after the cut-off, and taking the slope of the Linear Fit of the points that remain. The user can choose to either divide each axis in turn (`all_points`) or divide them all at the same time (`equal_division`).

**`get_lacun`** calculates the lacunarity for each scale. Please note that the lacunarity is a scale-dependent quantity, and cannot be computed analytically.

**`get_fd_and_lacun`** is an untested, almost-pseudocode combination of all the previous functions into a complete Fractal Analysis algorithm (that hopefully compiles).
