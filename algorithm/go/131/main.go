package main

func main() {
	partition("cbbbcc")
}

func partition(s string) [][]string {
	result := make([][]string, 0)
	temp := make([]string, 0)

	concussion(s, temp, &result)

	return result
}

func concussion(s string, temp []string, result *[][]string) {
	if s == "" {
		*result = append(*result, append([]string{}, temp...))
		return
	}

	for i, _ := range s {
		subStr := s[:i+1]
		if !isPalindrome(subStr) {
			continue
		}

		temp = append(temp, subStr)
		concussion(s[i+1:], temp, result)
		temp = temp[:len(temp)-1]
	}
}

func isPalindrome(s string) bool {
	l, r := 0, len(s)-1
	for l < r {
		if s[l] != s[r] {
			return false
		}
		l++
		r--
	}
	return true
}
