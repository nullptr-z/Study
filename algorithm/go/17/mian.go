package main

func main() {
	letterCombinations("23")
}

var digitsMap = map[rune]string{
	'2': "abc",
	'3': "def",
	'4': "ghi",
	'5': "jkl",
	'6': "mno",
	'7': "pqrs",
	'8': "tuv",
	'9': "wxyz",
}

func letterCombinations(digits string) []string {
	result := make([]string, 0)
	concussion(digits, "", &result)

	return result
}

func concussion(digits string, curStr string, result *[]string) {
	if len(digits) == 0 {
		if curStr != "" {
			*result = append(*result, curStr)
		}
		return
	}

	head := rune(digits[0])
	digits = digits[1:]
	for _, c := range digitsMap[head] {
		concussion(digits, curStr+string(c), result)
	}
}

func concussion2(digits string, result *[]string) {
	if len(digits) == 0 {
		return
	}

	tmps := make([]string, 0)
	chars := digitsMap[rune(digits[0])]
	for _, ch := range chars {
		if len(*result) == 0 {
			tmps = append(tmps, string(ch))
		} else {
			for i := 0; i < len(*result); i++ {
				tmps = append(tmps, (*result)[i]+string(ch))
			}
		}
	}
	*result = tmps

	concussion2(digits[1:], result)
}
