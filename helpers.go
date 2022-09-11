package main

import (
	"strconv"
	"time"
)

func ParseTime(s string) time.Time {
	i, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return time.Now()
	}
	return time.Unix(i, 0)
}
