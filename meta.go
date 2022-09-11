package main

import (
	"runtime"

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

// Flags enum
const (
	FlagID          string = "ID"
	FlagDetails     string = "Details"
	FlagState       string = "State"
	FlagImageXL     string = "LargeImage"
	FlagImageXLText string = "LargeText"
	FlagImage       string = "SmallImage"
	FlagImageText   string = "SmallText"
	FlagPartySize   string = "PartySize"
	FlagPartyMax    string = "PartyMax"
	FlagStart       string = "Start"
	FlagEnd         string = "End"
)

// Desc enum
const (
	DescID          string = "Discord Application ID"
	DescDetails     string = "Rich Presence details"
	DescState       string = "Rich Presence state"
	DescImageXL     string = "Rich Presence large image"
	DescImageXLText string = "Rich Presence large image text"
	DescImage       string = "Rich Presence small image"
	DescImageText   string = "Rich Presence small image text"
	DescPartySize   string = "Rich Presence party size"
	DescPartyMax    string = "Rich Presence max party size"
	DescStart       string = "Rich Presence start timestamp"
	DescEnd         string = "Rich Presence end timestamp"
)

// Metadata
const (
	Repository string = "https://github.com/Stridsvagn69420/dstatus"
	Author     string = "Stridsvagn69420 (https://github.com/Stridsvagn69420)"
	License    string = "MIT"
	Arch       string = runtime.GOARCH
	OS         string = runtime.GOOS
)

func entry_meta(k string, v string) {
	printer.Print(k, pringo.MagentaBright)
	printer.Print(": ", pringo.MagentaBright)
	printer.Writeln(v)
}

func entry_flag(k string, v string) {
	printer.Print("-", pringo.CyanBright)
	printer.Print(k, pringo.CyanBright)
	printer.Print(": ", pringo.CyanBright)
	printer.Writeln(v)
}

func HelpMessage() {
	// Meta
	entry_meta("Author", Author)
	entry_meta("License", License)
	entry_meta("Repository", Repository)
	entry_meta("OS", OS)
	entry_meta("Arch", Arch)
	entry_meta("Go", runtime.Version())

	// Spacer
	printer.Writeln("")

	// Flag
	entry_flag(FlagID, DescID)
	entry_flag(FlagDetails, DescDetails)
	entry_flag(FlagState, DescState)
	entry_flag(FlagImageXL, DescImageXL)
	entry_flag(FlagImageXLText, DescImageXLText)
	entry_flag(FlagImage, DescImage)
	entry_flag(FlagImageText, DescImageText)
	entry_flag(FlagPartySize, DescPartySize)
	entry_flag(FlagPartyMax, DescPartyMax)
	entry_flag(FlagStart, DescStart)
	entry_flag(FlagEnd, DescEnd)
}
