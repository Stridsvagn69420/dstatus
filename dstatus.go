package main

import (
	"flag"
	"os"
	"os/signal"

	"github.com/Stridsvagn69420/pringo"
	"github.com/hugolgst/rich-go/client"
)

var printer *pringo.Printer = pringo.NewDefault()

func main() {
	if Contains(os.Args, "-h") || Contains(os.Args, "--help") {
		HelpMessage()
		os.Exit(0)
	}

	// ---- Flags ----
	id := flag.String(FlagID, "", DescID)
	details := flag.String(FlagDetails, "", DescDetails)
	state := flag.String(FlagState, "", DescState)

	// Assets
	large_image := flag.String(FlagImageXL, "", DescImageXL)
	large_text := flag.String(FlagImageXLText, "", DescImageXLText)
	small_image := flag.String(FlagImage, "", DescImage)
	small_text := flag.String(FlagImageText, "", DescImageText)

	// Party
	party_size := flag.Int(FlagPartySize, 0, DescPartySize)
	party_max := flag.Int(FlagPartyMax, 0, DescPartyMax)

	// Timestamps
	start_time := flag.String(FlagStart, "", DescStart)
	end_time := flag.String(FlagEnd, "", DescEnd)

	flag.Parse()

	// ---- Main ----
	// Login
	err := client.Login(*id)
	// Exit if login fails
	if err != nil {
		printer.Errorln("Couldn't login to Discord! Maybe Discord isn't running or your ID is invalid!", pringo.RedBright)
		os.Exit(1)
	}

	// Create and set activity
	party, partclean := Party(party_size, party_max)
	times, timeclean := Timestamps(start_time, end_time)
	act := Activity(details, state, large_image, large_text, small_image, small_text, party, partclean, times, timeclean)
	err = client.SetActivity(act)
	// Exit if Activity could not be set
	if err != nil {
		printer.Errorln("Could not set the custom status...", pringo.RedBright)
		os.Exit(1)
	}

	// Exit on Ctrl+C
	printer.Println("Successfully set the custom status! You can now exit with Ctrl + C.", pringo.CyanBright)
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	for sig := range c {
		if sig == os.Interrupt {
			client.Logout()
			os.Exit(0)
		}
	}
}
