# rust_random_walk
Single-process and multiprocess Python and Rust [native+cython bindings] implementation to compute the effects of simulation steps, n, and number of trajectory replicas, k, on the mean squared displacement for a 3D random walk for a brownian particle.

DISCLAIMER: The mini-project was for educational purposes only. I wanted to practice a new programming language [rust] and compare performance to languages I'm more familar with.


**Rust single process performance:**  
Mean Square Displacement for k = 10 replicates, num_steps = 10000: 10192.191, elapsed_time = 0.002106027  
Mean Square Displacement for k = 100 replicates, num_steps = 10000: 10075.982, elapsed_time = 0.021394288  
Mean Square Displacement for k = 1000 replicates, num_steps = 10000: 9848.219, elapsed_time = 0.211539328  
Mean Square Displacement for k = 10 replicates, num_steps = 20000: 16991.5, elapsed_time = 0.004313239  
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 20952.082, elapsed_time = 0.042394405  
Mean Square Displacement for k = 1000 replicates, num_steps = 20000: 20291.637, elapsed_time = 0.422848057  
Mean Square Displacement for k = 10 replicates, num_steps = 50000: 40920.28, elapsed_time = 0.010577796  
Mean Square Displacement for k = 100 replicates, num_steps = 50000: 46258.684, elapsed_time = 0.106180895  
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 50712.59, elapsed_time = 1.058756543  

**Python single-threaded performance:**. 
Mean Square Displacement for k = 10 replicates, num_steps = 10000: 6833.8876915999035, time_taken = 0.5159621872007847  
Mean Square Displacement for k = 100 replicates, num_steps = 10000: 10634.575957497713, time_taken = 5.136959375813603  
Mean Square Displacement for k = 1000 replicates, num_steps = 10000: 10807.334528437468, time_taken = 51.843449180945754  
Mean Square Displacement for k = 10 replicates, num_steps = 20000: 36691.720214161476, time_taken = 1.0094404853880405  
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 23129.646697901724, time_taken = 10.06736945733428  
Mean Square Displacement for k = 1000 replicates, num_steps = 20000: 20643.71945733481, time_taken = 101.59397887997329  
Mean Square Displacement for k = 10 replicates, num_steps = 50000: 53297.01535155461, time_taken = 2.513979187235236  
Mean Square Displacement for k = 100 replicates, num_steps = 50000: 48949.95597669806, time_taken = 25.25233313627541  
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 48440.117448288045, time_taken = 251.9878515955060  

**Rust multiprocess performance:**.  
Mean Square Displacement for k = 10 replicates, num_steps = 10000: 8747.771, elapsed_time = 0.000950147  
Mean Square Displacement for k = 100 replicates, num_steps = 10000: 10360.072, elapsed_time = 0.002640583  
Mean Square Displacement for k = 1000 replicates, num_steps = 10000: 9969.255, elapsed_time = 0.022421757  
Mean Square Displacement for k = 10 replicates, num_steps = 20000: 22224.982, elapsed_time = 0.000778944  
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 20343.19, elapsed_time = 0.004935162  
Mean Square Displacement for k = 1000 replicates, num_steps = 20000: 18588.477, elapsed_time = 0.044708656  
Mean Square Displacement for k = 10 replicates, num_steps = 50000: 55396.29, elapsed_time = 0.001771234  
Mean Square Displacement for k = 100 replicates, num_steps = 50000: 47232.63, elapsed_time = 0.011760786  
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 50997.117, elapsed_time = 0.110842002  

**Python multiprocess Performance: **
#TODO This is slower than single process (?) that seems odd - it shows CPU utilization across thread. I supposed it could be communication overhead [or that file I/O is making it slower, but threading is still slower than multiprocess? idk will revisit]

Mean Square Displacement for k = 10 replicates, num_steps = 10000: 6883.154785280928, time_taken = 0.6112805679440498 
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 10001.502011606852, time_taken = 6.649483831599355 
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 9428.137658705115, time_taken = 58.22610455006361           
Mean Square Displacement for k = 10 replicates, num_steps = 10000: 26847.080594440085, time_taken = 1.6962700989097357 
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 18053.40328286095, time_taken = 13.64875154197216 
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 19757.0114801063, time_taken = 112.55253617092967 
Mean Square Displacement for k = 10 replicates, num_steps = 10000: 57813.83716877069, time_taken = 3.4486921429634094 
Mean Square Displacement for k = 100 replicates, num_steps = 20000: 44096.42074399401, time_taken = 30.488818733021617 
Mean Square Displacement for k = 1000 replicates, num_steps = 50000: 50545.52291332853, time_taken = 265.4872075114399 


**CPU info:**
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
CPU(s):              16
On-line CPU(s) list: 0-15
Thread(s) per core:  2
Core(s) per socket:  8
Socket(s):           1
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               158
Model name:          Intel(R) Core(TM) i9-9900 CPU @ 3.10GHz
Stepping:            13
CPU MHz:             4357.310
CPU max MHz:         5000.0000
CPU min MHz:         800.0000
BogoMIPS:            6199.99
Virtualization:      VT-x
L1d cache:           32K
L1i cache:           32K
L2 cache:            256K
L3 cache:            16384K
NUMA node0 CPU(s):   0-15
