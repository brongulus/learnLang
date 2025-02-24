package main

import (
	"fmt"
	"sync"
	"time"
)

var (
	jobs    = 20                 // Run 20 jobs in total.
	running = make(chan bool, 3) // Limit concurrent jobs to 3.
	wg      sync.WaitGroup       // Keep track of which jobs are finished.
)

func main() {
	wg.Add(jobs)
	for i := 1; i <= jobs; i++ {
		running <- true // Fill running;
		// this will block and wait if it's already full.

		// Start a job.
		go func(i int) {
			defer func() {
				<-running // Drain running so new jobs can be added.
				wg.Done() // Signal that this job is done.
			}()

			// "do work"
			time.Sleep(1 * time.Second)
			fmt.Println(i)
		}(i)
	}

	wg.Wait() // Wait until all jobs are done.
	fmt.Println("done")
}
