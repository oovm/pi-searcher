## The largest position in each digit

| Digits |        Max |      Position | NotFound |
|-------:|-----------:|--------------:|---------:|
|      1 |          0 |            31 |        0 |
|      2 |         68 |           604 |        0 |
|      3 |        483 |         8,552 |        0 |
|      4 |      6,716 |        99,845 |        0 |
|      5 |     33,394 |     1,369,559 |        0 |
|      6 |    569,540 |    14,118,306 |        0 |
|      7 |  1,075,656 |   166,100,499 |        0 |
|      8 | 36,432,643 | 1,816,743,904 |        0 |
|      9 |          ? |             ? |        ? |

Assuming pi is a [Normal Number](https://en.wikipedia.org/wiki/Normal_number), then the distribution of the maximum
number position satisfies the [Exponential Distribution](https://en.wikipedia.org/wiki/Exponential_distribution).

That is, its probability density function is:

$$\frac{e^{-\frac{d}{10^d}}}{10^d}$$

where $d$ is the number of digits in the number.

So we have:
$$\max(d)\approx 10^d d \log (10)$$

# Minimize storage

In decimal, it takes 1 byte to store a digit, and 24 + 8 = 32 bytes to store a Record

So if we only record numbers greater than n, the required storage space is as follows:

```wolfram
n + 32 * 10^d Probability[x>n, x\[Distributed]ExponentialDistribution[10^-d],Assumptions->{n>0}]//FullSimplify
2^(5+d) 5^d E^(-10^-d n)+n
```

For any d > 0 this function has a minimum in the domain of n > 0.

So it can be solved as:

$$n = 5*10^d \ln{2}, \bytes_{\max} = 10^d (1+\ln{32})$$

Actually, according to the speed of lookup and table lookup, the size of n can be optimized additionally.
