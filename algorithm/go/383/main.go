package main

func main() {
	canConstruct("aa", "bb")
}

func canConstruct(ransomNote string, magazine string) bool {
	mmap := make(map[rune]int)
	for _, m := range magazine {
		mmap[m]++
	}

	for _, r := range ransomNote {
		mmap[r]--
		if mmap[r] < 0 {
			return false
		}
	}

	return true
}
