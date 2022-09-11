package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"

	"github.com/Stridsvagn69420/pringo"
	"github.com/hugolgst/rich-go/client"
)

var printer *pringo.Printer = pringo.NewDefault()

func main() {
	// ---- Flags ----
	id := flag.String("ID", "", "Application ID")
	details := flag.String("Details", "", "Rich Presence details")
	state := flag.String("State", "", "Rich Presence state")

	// Assets
	large_image := flag.String("LargeImage", "", "Rich Presence large image")
	large_text := flag.String("LargeText", "", "Rich Presence large image text")
	small_image := flag.String("SmallImage", "", "Rich Presence small image")
	small_text := flag.String("SmallText", "", "Rich Presence small image text")

	// Party
	party_size := flag.Int("PartySize", 0, "Rich Presence party size")
	party_max := flag.Int("PartyMax", 0, "Rich Presence max party size")

	// Timestamps
	start_time := flag.String("Start", "", "Rich Presence start timestamp")
	end_time := flag.String("End", "", "Rich Presence end timestamp")

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

	fmt.Printf("%+v\n", act)

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
