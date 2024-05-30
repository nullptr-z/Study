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
	concussion(digits, &result)

	return result
}

func concussion(digits string, tmp *[]string) {
	if len(digits) == 0 {
		return
	}

	tmps := make([]string, 0)
	chars := digitsMap[rune(digits[0])]
	for _, ch := range chars {
		if len(*tmp) == 0 {
			tmps = append(tmps, string(ch))
		} else {
			for i := 0; i < len(*tmp); i++ {
				tmps = append(tmps, (*tmp)[i]+string(ch))
			}
		}
	}
	*tmp = tmps

	concussion(digits[1:], tmp)
}
