# Reconnect

Lightweight and highly performant reconciliation framework

## Benchmarks

## Benchmark Results

### postgres_full_table

|                | `single_store` | `multi_store` |
|:---------------|:---------------|:--------------|
| **`10000`**    | `34.08 ms`     | `188.74 ms`   |
| **`100000`**   | `197.86 ms`    | `1.44 s`      |
| **`1000000`**  | `1.92 s`       | `10.73 s`     |
| **`10000000`** | `16.96 s`      | `113.28 s`    |

### Overall distribution

![Violin](https://github.com/arunma/reconnect/blob/master/reconnect-bench/images/Violin.png)

### Both datasets residing in a single database

![Single store](https://github.com/arunma/reconnect/blob/master/reconnect-bench/images/single_store.png)

### Datasets residing in different databases

![Multi store](https://github.com/arunma/reconnect/blob/master/reconnect-bench/images/multi_store.png)



