package main

import (
	"fmt"
	"os"
	"strconv"
)

func fibonacci(n int) (int, error) {
	if n < 0 {
		return 0, fmt.Errorf("cannot calculate Fibonacci number for negative input")
	}

	if n <= 1 {
		return n, nil
	}

	a, b := 0, 1
	for i := 2; i <= n; i++ {
		c := a + b
		if c < a || c < b { // Overflow check
			return 0, fmt.Errorf("arithmetic overflow occurred at Fibonacci number %d", n)
		}
		a, b = b, c
	}

	return b, nil
}

func main() {
	if len(os.Args) != 2 {
		fmt.Println("Usage: go run main.go <number>")
		return
	}

	n, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Error: Invalid number provided.")
		return
	}

	result, err := fibonacci(n)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	fmt.Printf("Fibonacci number %d is %d\n", n, result)
}
