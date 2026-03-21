package main

import (
	"fmt"
)

// func str_replace(str, target, rep string) string {
// 	var result []byte
// 	to_bytes := []byte(str)
//
// 	for i:=0; i < len(str) - 1; i++ {
// 	}
// 	return ""
// }

func main() {
	username := "numbers_game_should_match"
	search := "am"
	replace := "ff"
	var result []byte

	to_bytes := []byte(username)
	for i := 0; i < len(username)-1; i++ {
		limit := i + len(search)

		cmp_str := string(to_bytes[i:limit])

		// would be nice if we could XOR here
		if cmp_str == search {
			result = append(result, []byte(replace)...)
		} else {
			if i == 0 {
				result = append(result, []byte(cmp_str[:1])...)
			}
			result = append(result, []byte(cmp_str[1:])...)
		}
	}

	fmt.Printf("we have String.Replace at home: %s\n", result)

}
