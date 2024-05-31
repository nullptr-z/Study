package main

import (
	"strconv"
	"strings"
)

func main() {
	restoreIpAddresses("25525511135")
}

func restoreIpAddresses(s string) []string {
	result := make([]string, 0)
	temp := make([]string, 0)
	backtrack(s, temp, &result)
	return result
}

func backtrack(s string, temp []string, result *[]string) {
	if len(s) > (4-len(temp))*3 {
		// 剪枝，剩余的字符个数大于最大还能消化的字符数量
		return
	}
	if len(s) == 0 {
		if len(temp) == 4 {
			*result = append(*result, strings.Join(temp, "."))
		}
		return
	}

	for i := 0; i < len(s); i++ {
		subStr := s[:i+1]
		num, _ := strconv.ParseInt(subStr, 10, 64)
		if (i > 0 && subStr[0] == '0') || num > 255 {
			continue
		}
		temp = append(temp, subStr)
		backtrack(s[i+1:], temp, result)
		temp = temp[:len(temp)-1]
	}
}
