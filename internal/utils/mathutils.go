package utils

func ClampInts(v, min, max int) int {
	if v > max {
		return max
	} else if v < min {
		return min
	}
	return v
}
