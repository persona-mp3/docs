package main

func work_with_string(a *string) {
	println("working_with_string", *a)
}
func main() {
	s := "very_fat_string_imaging_it_'s_big"
	work_with_string(&s)
}
