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
File EOF

 Performance counter stats for './new-io 40mb.rdb 10':

        195,592509      task-clock (msec)         #    0,996 CPUs utilized          
                 1      context-switches          #    0,005 K/sec                  
                 1      cpu-migrations            #    0,005 K/sec                  
                90      page-faults               #    0,460 K/sec                  
       361.193.546      cycles                    #    1,847 GHz                    
        68.513.968      stalled-cycles-frontend   #   18,97% frontend cycles idle   
        17.128.221      stalled-cycles-backend    #    4,74% backend  cycles idle   
       854.905.162      instructions              #    2,37  insns per cycle        
                                                  #    0,08  stalled cycles per insn
       200.417.645      branches                  # 1024,669 M/sec                  
           703.271      branch-misses             #    0,35% of all branches        

       0,196332262 seconds time elapsed

perf stat ./old-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
File EOF

 Performance counter stats for './old-io 40mb.rdb 10':

         66,669576      task-clock (msec)         #    0,987 CPUs utilized          
                 4      context-switches          #    0,060 K/sec                  
                 2      cpu-migrations            #    0,030 K/sec                  
                93      page-faults               #    0,001 M/sec                  
        74.708.520      cycles                    #    1,121 GHz                    
        15.413.747      stalled-cycles-frontend   #   20,63% frontend cycles idle   
         3.201.621      stalled-cycles-backend    #    4,29% backend  cycles idle   
       191.248.318      instructions              #    2,56  insns per cycle        
                                                  #    0,08  stalled cycles per insn
        40.266.057      branches                  #  603,964 M/sec                  
            22.353      branch-misses             #    0,06% of all branches        

       0,067532663 seconds time elapsed
```

Unoptimized build:

```
$ make -B FLAGS=
$ make perf
perf stat ./new-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
File EOF

 Performance counter stats for './new-io 40mb.rdb 10':

       1409,771195      task-clock (msec)         #    0,999 CPUs utilized          
                46      context-switches          #    0,033 K/sec                  
                 7      cpu-migrations            #    0,005 K/sec                  
                93      page-faults               #    0,066 K/sec                  
     3.262.241.400      cycles                    #    2,314 GHz                    
     1.229.369.572      stalled-cycles-frontend   #   37,68% frontend cycles idle   
       148.728.645      stalled-cycles-backend    #    4,56% backend  cycles idle   
     5.509.870.855      instructions              #    1,69  insns per cycle        
                                                  #    0,22  stalled cycles per insn
     1.263.217.302      branches                  #  896,044 M/sec                  
         1.387.391      branch-misses             #    0,11% of all branches        

       1,410970533 seconds time elapsed

perf stat ./old-io 40mb.rdb 10
reading file '40mb.rdb'
len: 10
File EOF

 Performance counter stats for './old-io 40mb.rdb 10':

        741,191122      task-clock (msec)         #    0,998 CPUs utilized          
                47      context-switches          #    0,063 K/sec                  
                 2      cpu-migrations            #    0,003 K/sec                  
                91      page-faults               #    0,123 K/sec                  
     1.701.845.437      cycles                    #    2,296 GHz                    
       667.249.110      stalled-cycles-frontend   #   39,21% frontend cycles idle   
        81.374.644      stalled-cycles-backend    #    4,78% backend  cycles idle   
     2.824.367.181      instructions              #    1,66  insns per cycle        
                                                  #    0,24  stalled cycles per insn
       590.932.539      branches                  #  797,274 M/sec                  
           704.589      branch-misses             #    0,12% of all branches        

       0,742850663 seconds time elapsed
```
