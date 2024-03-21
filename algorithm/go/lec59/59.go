package main

func generateMatrix(n int) [][]int {
	// 创建一个 n×n 的二维切片
	matrix := make([][]int, n)
	for i := range matrix {
		matrix[i] = make([]int, n)
	}

	num := 1
	top, bottom, left, right := 0, n-1, 0, n-1

	for num <= n*n {
		for i := 0; i <= right; i++ {
			matrix[top][i] = num
			num++
		}
		top++

		for i := top; i <= bottom; i++ {
			matrix[i][right] = num
			num++
		}
		right--

		for i := right; i >= left; i-- {
			matrix[bottom][i] = num
			num++
		}
		bottom--

		for i := bottom; i >= top; i-- {
			matrix[i][left] = num
			num++
		}
		left++
	}

	return matrix
}

func main() {
	generateMatrix(3)
}
