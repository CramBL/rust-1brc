# Rust for 1 billion row challenge

# Baseline
```shell
time ./calculate_average_baseline.sh
```
>./calculate_average_baseline.sh  108,64s user 4,84s system 100% cpu 1:52,85 total


# Attemps

## Sequential

### Sequential, read all, build map, parse and display values
1. 7 min.
2. /target/release/rust-1brc  154,61s user 16,80s system 99% cpu 2:51,46 total

### Sequential, read all, index into vectors by starting letter and find name and push temps, finally process all vectors
3. 77.75user 8.29system 1:29.43elapsed

### As above but memory mapped
4. 77.11user 6.34system 1:23.47elapsed

### City name in 101-byte arrays instead of vec
5. 74.42user 6.33system 1:20.78elapsed

### Divide mmap into regions, process one region at a time and aggregate region result
> This is probably faster because the overall data structure slows down over time (a lot of cities to search through), and this approach builds many smaller data structures and aggregates them, which reduces the average search space when performing insertions. It might also be faster because of fewer large reallocations, when vectors grow beyong their capacity and has to be moved to allow them to grow in memory.
6. 68.46user 2.85system 1:11.35elapsed

## Parallel

### Naive scoped threads with 500 MiB mmap regions 

7. 07.41user 4.51system 0:23.52elapsed

### Rayon map-reduce in region and post-processing (calc of min/mean/max)

8. 136.67user 11.61system 0:15.28elapsed 

### Same as above but avoid floats for everything but mean

9. 110.37user 8.17system 0:11.02elapsed