# Base-N Divisor

Base-N Divisor was an experiment I recently coded up to see if using the base-divisor version of a number to divide it would be a faster alternative to regular division.

## The result of the experiment

It is slower by about 0.0001s, more or less. But I still feel that something along the lines of this process might create a future, simpler division process. But it also could just be one of those tiny experiments with nothing special about it. I am still proud of this tiny project even though I scrambled it in about 2 days. This was my first ever Rust project.

## May I use this to do division even though it will probably be slower than regular division?

Sure! Whatever!

## How does the divisor work?

The divisor works with two inputs, the numbers in base-divisor, and the divisor. I wish I knew how do convert the number that will be divided into base-divisor, but the simple process of doing so uses division, and it would be cheating if I included it in the whole divisor. If I figure out how to do that, I will try to add the method and let the divisor take in just numbers instead of base-divisor "bits". The divisor takes the input and simply just shifts the "bits" to the right once. Then it returns an unsigned integer number. The divisor can convert the "bits" back to an unsigned integer number, just not the other way around.
I wouldn't call the "bits" actual bits, because bits refer to binary, so when I say "bits", I mean numbers in a base-divisor representation.

## Limitations

1. It is slower than regular division.
2. The code is most likely not optimized to its fullest due to my lack of Rust knowledge.
3. Again, the input needs the base-divisor representation of the number to divide, which probably won't work for actual projects.
4. It can only work with unsigned integer inputs and outputs. There is a boolean that dictates if the size of the "bit" list includes non-integer digits, but the code doesn't actually take non-integer numbers into account, nor does it take signed numbers into account, only unsigned integer numbers will work.
