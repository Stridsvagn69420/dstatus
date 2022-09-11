package main

import (
	"github.com/Stridsvagn69420/pringo"
)

func Contains(s []string, str string) bool {
	for _, v := range s {
		if v == str {
			return true
		}
	}

	return false
}

func HelpMessage() {
	// TODO: Print Help Message
	printer.Println("WIP.", pringo.Green)
}
