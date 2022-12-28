  Decimal          SNAFU
        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       11             21
       12             22
       13            1==
       14            1=-
       15            1=0
       16            1=1
       17            1=2
       18            1-=
       19            1--
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0

So, taking 2* as a starting point:

S_20 = (2 * 5) + 0 => D_10

Is 2 the right starting digit? If -2 <= N - (2 * 5) <= 2, then yes. Or:

0 <= N + 2 - (2 * 5) <= 4

…which certainly SMELLS like % 5.

Looks like we're only dealing with positive integers, so first digit must be either 1 or 2. So to find first digit:

~~Lower bound for N digit snafu number: 5^N - (2 * 5^(N - 1))~~
~~Upper bound:                          5^(N + 1) - (2 * 5^(N)) - 1~~

For 2 digits, lower bound = D_3; upper bound = D12
