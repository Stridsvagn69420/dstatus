package main

import (
	"time"

	"github.com/hugolgst/rich-go/client"
)

func Activity(details *string, state *string, large_image *string, large_text *string, small_image *string, small_text *string, party client.Party, timestamps client.Timestamps) client.Activity {
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

func Party(party_size *int, party_max *int) client.Party {
	if *party_size == 0 && *party_max == 0 {
		return client.Party{}
	} else {
		return client.Party{
			ID:         "-1",
			Players:    *party_size,
			MaxPlayers: *party_max,
		}
	}
}

func Timestamps(start_time *string, end_time *string) client.Timestamps {
	var start time.Time
	var end time.Time

	if len(*start_time) > 0 {
		start = ParseTime(*start_time)
	}

	if len(*end_time) > 0 {
		end = ParseTime(*end_time)
	}

	return client.Timestamps{
		Start: &start,
		End:   &end,
	}
}
