import numpy as np
import time


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

def mean_squared_displacement(k, n, l):
    total_msd = 0.0

    for _ in range(k):
        final_point = random_walk(n, l)
        dx = final_point.x
        dy = final_point.y
        dz = final_point.z

        msd = dx**2 + dy**2 + dz**2
        total_msd += msd

    return total_msd / k

def main():
    k_values = [10, 50]
    n_values = [10000000]
    l = 1.0
    for n in n_values:
        for k in k_values:
            start_time = time.perf_counter()

            msd = mean_squared_displacement(k, n, l)
            
            end_time = time.perf_counter()

            print(f"Mean Square Displacement for k = {k} replicates, num_steps = {n}: {msd}, time_taken = {end_time-start_time}")

if __name__ == "__main__":
    main()

