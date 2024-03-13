package main

import (
	"fmt"
	"math"
	"math/rand"
	"time"
)

type Point struct {
	x, y, z float64
}

func randomWalk(numSteps int, stepSize float64) Point {
	rng := rand.New(rand.NewSource(time.Now().UnixNano()))
	current := Point{0.0, 0.0, 0.0}

	for i := 0; i < numSteps; i++ {
		theta := rng.Float64() * 2.0 * math.Pi
		phi := rng.Float64() * math.Pi
		dx := stepSize * math.Cos(theta) * math.Sin(phi)
		dy := stepSize * math.Sin(theta) * math.Sin(phi)
		dz := stepSize * math.Cos(phi)

		current.x += dx
		current.y += dy
		current.z += dz
	}

	return current
}

func meanSquaredDisplacement(k, n int, l float64) float64 {
	totalMSD := 0.0

	for i := 0; i < k; i++ {
		finalPoint := randomWalk(n, l)
		dx := finalPoint.x
		dy := finalPoint.y
		dz := finalPoint.z

		msd := dx*dx + dy*dy + dz*dz
		totalMSD += msd
	}

	return totalMSD / float64(k)
}

func main() {
	kValues := []int{10,100, 1000}
	nValues := []int{10000, 20000, 50000}
    l := 1.0

	for _, n := range nValues {
		for _, k := range kValues {
			startTime := time.Now()

			msd := meanSquaredDisplacement(k, n, l)

			endTime := time.Now()

			fmt.Printf("Mean Square Displacement for k = %d replicates, num_steps = %d: %f, time_taken = %v\n", k, n, msd, endTime.Sub(startTime))
		}
	}
}

