package math

import (
	"math"
)

// Sqrt calcola la radice quadrata di x.
func Sqrt(x float64) float64 {
	return math.Sqrt(x)
}

// Pow calcola x elevato alla potenza y.
func Pow(x, y float64) float64 {
	return math.Pow(x, y)
}

// Log calcola il logaritmo di x con base y.
func Log(x, y float64) float64 {
	return math.Log(x) / math.Log(y)
}

// Cube calcola il cubo di x.
func Cube(x float64) float64 {
	return x * x * x
}

// CircleArea calcola l'area di un cerchio con raggio r.
func CircleArea(r float64) float64 {
	return math.Pi * r * r
}
