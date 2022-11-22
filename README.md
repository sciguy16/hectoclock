## hectoclock

Implementation of decimal time. Each day (86400 seconds) is divided into
100 "hectodays", each of which is divided into ten "tenths", or "millidays".
Millidays can be represented as a floating point to include fractional
millidays (but fixed point would be better).

Running this program produces the following output format:

```bash
-cargo r
time: 91.2.780
```

This represents 91 hectodays, 2 tenths, 780 microdays, or about 21:54:24
in traditional 24-hour time.
