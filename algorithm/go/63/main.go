package main

func main() {
	// uniquePathsWithObstacles([][]int{
	// 	{0, 0, 0},
	// 	{0, 1, 0},
	// 	{0, 0, 0},
	// })

	uniquePathsWithObstacles([][]int{
		{0, 1},
	})
}

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	row := len(obstacleGrid)
	col := len(obstacleGrid[0])

	dp := make([]int, col)
	for i := 0; i < col; i++ {
		if obstacleGrid[0][i] == 1 {
			break
		}
		dp[i] = 1
	}

	for r := 1; r < row; r++ {
		for c := 0; c < col; c++ {
			if obstacleGrid[r][c] == 1 {
				dp[c] = 0
				continue
			}
			if c != 0 {
				dp[c] += dp[c-1]
			}
		}
	}
	return dp[len(dp)-1]
}
