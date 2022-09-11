package main

import (
	"time"

	"github.com/hugolgst/rich-go/client"
)

func Activity(details *string, state *string, large_image *string, large_text *string, small_image *string, small_text *string, party client.Party, partyclean bool, timestamps client.Timestamps, timeclean bool) client.Activity {
	// Return all but Party
	if !partyclean {
		return client.Activity{
			State:      *state,
			Details:    *details,
			LargeImage: *large_image,
			LargeText:  *large_text,
			SmallImage: *small_image,
			SmallText:  *small_text,
			Timestamps: &timestamps,
		}
	}

	// Return all but Timestamps
	if !timeclean {
		return client.Activity{
			State:      *state,
			Details:    *details,
			LargeImage: *large_image,
			LargeText:  *large_text,
			SmallImage: *small_image,
			SmallText:  *small_text,
			Party:      &party,
		}
	}

	// Return with all
	return client.Activity{
		State:      *state,
		Details:    *details,
		LargeImage: *large_image,
		LargeText:  *large_text,
		SmallImage: *small_image,
		SmallText:  *small_text,
		Party:      &party,
		Timestamps: &timestamps,
	}
}

func Party(party_size *int, party_max *int) (client.Party, bool) {
	if *party_size == 0 && *party_max == 0 {
		return client.Party{}, false
	} else {
		return client.Party{
			ID:         "-1",
			Players:    *party_size,
			MaxPlayers: *party_max,
		}, true
	}
}

func Timestamps(start_time *string, end_time *string) (client.Timestamps, bool) {
	var start time.Time
	startinit := false
	var end time.Time
	endinit := false

	// Get Time
	if len(*start_time) > 0 {
		start = ParseTime(*start_time)
		startinit = true
	}
	if len(*end_time) > 0 {
		end = ParseTime(*end_time)
		endinit = true
	}

	// Return both Start and Ende
	if startinit && endinit {
		return client.Timestamps{
			Start: &start,
			End:   &end,
		}, true
	}

	// Return only Start
	if startinit {
		return client.Timestamps{
			Start: &start,
		}, true
	}

	// Return only End
	if endinit {
		return client.Timestamps{
			End: &end,
		}, true
	}

	return client.Timestamps{}, false
}
