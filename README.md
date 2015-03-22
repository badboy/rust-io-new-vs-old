# Rust benchmark: old io vs. new io

While porting [rdb-rs](https://github.com/badboy/rdb-rs) I came across huge performance drops when switching over to the [new io module](http://doc.rust-lang.org/nightly/std/io/index.html).

This repository contains a small test case with the [old](old-io.rs) and [new](new-io.rs) module in use.
I know my implementation of `read_exact` is far from perfect or optimized and I wonder how to do it properly.

Compile with:

```
make
# For unoptimized build:
make FLAGS=
```

Run performance test (requires Linux perf):

```
make perf
```

## Local results for the included `40mb.rdb`

```
$ rustc --version
rustc 1.0.0-nightly (e2fa53e59 2015-03-20) (built 2015-03-21)
$ make perf
perf stat ./new-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
total read: 6789100
Error occured: Error { repr: Custom(Custom { kind: Other, desc: "Premature EOF", detail: None }) }

 Performance counter stats for './new-io 40mb.rdb 10':

         51,792310      task-clock (msec)         #    0,988 CPUs utilized
                 0      context-switches          #    0,000 K/sec
                 1      cpu-migrations            #    0,019 K/sec
                91      page-faults               #    0,002 M/sec
        76.092.295      cycles                    #    1,469 GHz
        17.007.436      stalled-cycles-frontend   #   22,35% frontend cycles idle
         4.666.648      stalled-cycles-backend    #    6,13% backend  cycles idle
       198.183.724      instructions              #    2,60  insns per cycle
                                                  #    0,09  stalled cycles per insn
        40.159.421      branches                  #  775,394 M/sec
            23.136      branch-misses             #    0,06% of all branches

       0,052395574 seconds time elapsed

perf stat ./old-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
total read: 6789100
File EOF

 Performance counter stats for './old-io 40mb.rdb 10':

         38,478493      task-clock (msec)         #    0,990 CPUs utilized
                 0      context-switches          #    0,000 K/sec
                 1      cpu-migrations            #    0,026 K/sec
                92      page-faults               #    0,002 M/sec
        73.208.076      cycles                    #    1,903 GHz
        15.416.222      stalled-cycles-frontend   #   21,06% frontend cycles idle
         3.199.782      stalled-cycles-backend    #    4,37% backend  cycles idle
       191.857.108      instructions              #    2,62  insns per cycle
                                                  #    0,08  stalled cycles per insn
        40.252.577      branches                  # 1046,106 M/sec
            22.051      branch-misses             #    0,05% of all branches

       0,038877979 seconds time elapsed

```

Unoptimized build:

```
$ make -B FLAGS=
$ make perf
perf stat ./new-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
total read: 6789100
Error occured: Error { repr: Custom(Custom { kind: Other, desc: "Premature EOF", detail: None }) }

 Performance counter stats for './new-io 40mb.rdb 10':

        860,784463      task-clock (msec)         #    0,999 CPUs utilized
                12      context-switches          #    0,014 K/sec
                15      cpu-migrations            #    0,017 K/sec
                93      page-faults               #    0,108 K/sec
     2.058.030.887      cycles                    #    2,391 GHz
       877.873.779      stalled-cycles-frontend   #   42,66% frontend cycles idle
        85.669.427      stalled-cycles-backend    #    4,16% backend  cycles idle
     3.466.656.663      instructions              #    1,68  insns per cycle
                                                  #    0,25  stalled cycles per insn
       710.519.986      branches                  #  825,433 M/sec
           708.532      branch-misses             #    0,10% of all branches

       0,861473071 seconds time elapsed

perf stat ./old-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
total read: 6789100
File EOF

 Performance counter stats for './old-io 40mb.rdb 10':

        674,233347      task-clock (msec)         #    0,999 CPUs utilized
                28      context-switches          #    0,042 K/sec
                 4      cpu-migrations            #    0,006 K/sec
                93      page-faults               #    0,138 K/sec
     1.678.672.215      cycles                    #    2,490 GHz
       647.754.789      stalled-cycles-frontend   #   38,59% frontend cycles idle
        75.167.751      stalled-cycles-backend    #    4,48% backend  cycles idle
     2.832.107.349      instructions              #    1,69  insns per cycle
                                                  #    0,23  stalled cycles per insn
       592.338.044      branches                  #  878,536 M/sec
           709.566      branch-misses             #    0,12% of all branches

       0,674874353 seconds time elapsed
```

Results on a 416 Mb file are [in a Gist](https://gist.github.com/badboy/24d4ba25679afd1b576d).
