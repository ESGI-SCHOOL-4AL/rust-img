package ppmrequest

import "github.com/System-Glitch/goyave/v2/validation"

var (
	// Generic represents a generic PPM image manipulation request
	Generic validation.RuleSet = validation.RuleSet{
		"image": {"required", "file", "extension:ppm", "count:1", "ppm"},
	}
)
