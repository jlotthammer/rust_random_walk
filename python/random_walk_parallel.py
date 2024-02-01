import numpy as np
import time
from concurrent.futures import ThreadPoolExecutor
from concurrent.futures import ProcessPoolExecutor

class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

def random_walk(num_steps, step_size):
    rng = np.random.default_rng()
    current = Point(0.0, 0.0, 0.0)

    for _ in range(num_steps):
        theta = rng.uniform(0.0, 2.0 * np.pi)
        phi = rng.uniform(0.0, np.pi)
        dx = step_size * np.cos(theta) * np.sin(phi)
        dy = step_size * np.sin(theta) * np.sin(phi)
        dz = step_size * np.cos(phi)

        current.x += dx
        current.y += dy
        current.z += dz

    return current

def mean_squared_displacement(args):
    k, n, l = args
    total_msd = 0.0

    start_time = time.perf_counter()

    for _ in range(k):
        final_point = random_walk(n, l)
        dx = final_point.x
        dy = final_point.y
        dz = final_point.z

        msd = dx**2 + dy**2 + dz**2
        total_msd += msd

    end_time = time.perf_counter()
    time_taken = end_time - start_time

    return total_msd / k, time_taken

##def main():
#    k_values = [10, 100, 1000]
#    n_values = [10000, 20000, 50000]
#    l = 1.0
#
#    with ThreadPoolExecutor() as executor:
#        futures = []
#        for n in n_values:
#            for k in k_values:
#                args = (k, n, l)
#                futures.append(executor.submit(mean_squared_displacement, args))
#
#        for future, n, k in zip(futures, n_values * len(k_values), k_values * len(n_values)):
#            msd, time_taken = future.result()
#            print(f"Mean Square Displacement for k = {k} replicates, num_steps = {n}: {msd}, time_taken = {time_taken}")
def main():
    k_values = [10, 100, 1000]
    n_values = [10000, 20000, 50000]
    l = 1.0

    with ProcessPoolExecutor() as executor:
        futures = []
        for n in n_values:
            for k in k_values:
                args = (k, n, l)
                futures.append(executor.submit(mean_squared_displacement, args))

        for future, n, k in zip(futures, n_values * len(k_values), k_values * len(n_values)):
            msd, time_taken = future.result()
            print(f"Mean Square Displacement for k = {k} replicates, num_steps = {n}: {msd}, time_taken = {time_taken}")

if __name__ == "__main__":
    main()
