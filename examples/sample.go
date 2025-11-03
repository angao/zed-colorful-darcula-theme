package main

import (
	"fmt"
	"time"
)

// ExportedConstant demonstrates exported constant highlighting
const ExportedConstant = 42

// localConstant demonstrates local constant highlighting
const localConstant = "local"

// ExportedInterface demonstrates exported interface highlighting
type ExportedInterface interface {
	ExportedMethod() string
	ProcessData(input string) error
}

// localInterface demonstrates local interface highlighting
type localInterface interface {
	process() bool
}

// ExportedStruct demonstrates exported struct highlighting
type ExportedStruct struct {
	ExportedField string // Exported struct field in lime green
	privateField  int    // Private struct field
	Data          []byte
}

// localStruct demonstrates local struct highlighting
type localStruct struct {
	value   string
	counter int
}

// Package level exported variable
var ExportedVariable = "exported"

// Package level local variable
var packageLocalVariable = "package local"

// ExportedFunction demonstrates exported function highlighting
func ExportedFunction(param string, count int) error {
	// Local variables in blue
	localVar := "local variable"
	result := count * 2

	// Shadowing variable demonstration
	{
		localVar := "shadowed variable"
		fmt.Println(localVar)
	}

	// Function call highlighting
	localFunction(param)
	ExportedFunction(localVar, result)

	return nil
}

// localFunction demonstrates local function highlighting
func localFunction(input string) string {
	return fmt.Sprintf("processed: %s", input)
}

// NewExportedStruct demonstrates constructor pattern
func NewExportedStruct(field string) *ExportedStruct {
	return &ExportedStruct{
		ExportedField: field,
		privateField:  0,
		Data:          make([]byte, 0),
	}
}

// ExportedMethod demonstrates method with receiver highlighting
func (e *ExportedStruct) ExportedMethod() string {
	// 'e' receiver should be highlighted in bright yellow
	return e.ExportedField
}

// ProcessData demonstrates method implementation
func (e *ExportedStruct) ProcessData(input string) error {
	// String literals in green
	message := "Processing data: " + input

	// Numbers in light blue
	timeout := 30 * time.Second
	maxRetries := 3

	// Boolean in orange
	isValid := true

	// Conditionals and keywords in orange
	if isValid && len(input) > 0 {
		for i := 0; i < maxRetries; i++ {
			fmt.Printf("Attempt %d of %d\n", i+1, maxRetries)
			time.Sleep(timeout)
		}
	}

	fmt.Println(message)
	return nil
}

// privateMethod demonstrates private method
func (e *ExportedStruct) privateMethod() int {
	return e.privateField * 2
}

// Generic function demonstrating type parameters
func GenericFunction[T any](items []T) T {
	if len(items) == 0 {
		var zero T
		return zero
	}
	return items[0]
}

func main() {
	// Comments in gray with italic style
	// This demonstrates the various color highlights

	// String highlighting
	greeting := "Hello, Colorful Darcula!"

	// Number highlighting
	numbers := []int{1, 2, 3, 4, 5}

	// Function calls - exported vs local
	ExportedFunction(greeting, 42)
	result := localFunction("test")

	// Struct initialization
	obj := NewExportedStruct("example")
	obj.ExportedField = "updated"

	// Method calls on receiver
	data := obj.ExportedMethod()
	obj.ProcessData(data)

	// Type assertions and conversions
	var iface interface{} = "string value"
	if str, ok := iface.(string); ok {
		fmt.Println(str)
	}

	// Generic function usage
	first := GenericFunction(numbers)

	// Error handling
	if err := obj.ProcessData("data"); err != nil {
		panic(err)
	}

	// Map and slice operations
	dataMap := make(map[string]int)
	dataMap["key"] = 100

	// Range loops
	for idx, num := range numbers {
		fmt.Printf("Index: %d, Value: %d, First: %d, Result: %s\n",
			idx, num, first, result)
	}

	// Switch statements
	switch greeting {
	case "Hello":
		fmt.Println("Matched Hello")
	default:
		fmt.Println("Default case")
	}
}
