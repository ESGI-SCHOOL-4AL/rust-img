package requests

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/System-Glitch/goyave/v2/helper/filesystem"
	"github.com/System-Glitch/goyave/v2/validation"
)

// If none of the available validation rules satisfy your needs, you can implement custom validation rules.
// https://system-glitch.github.io/goyave/guide/basics/validation.html#custom-rules

func init() {
	// Register your custom validation rules here.
	validation.AddRule("ppm", false, validatePPM)
}

func validatePPM(field string, value interface{}, parameters []string, form map[string]interface{}) bool {
	files, ok := value.([]filesystem.File)
	if ok {
		for _, file := range files {

			reader := bufio.NewReader(file.Data)

			step := 0
			for {
				buf, _, err := reader.ReadLine()
				if err == io.EOF {
					break
				}

				line := string(buf)

				if strings.HasPrefix(line, "#") {
					continue
				}

				switch step {
				case 0: // Version
					if line != "P3" {
						return false
					}
				case 1: // Dimension
					widthStr := ""
					_, err := fmt.Sscanf(line, "%s", &widthStr)
					if err != nil {
						return false
					}
					width, err := strconv.Atoi(widthStr)
					if err != nil || width <= 0 {
						return false
					}

					height := 0
					_, err = fmt.Sscanf(line[strings.Index(line, widthStr)+len(widthStr):], "%d", &height)
					if err != nil || height <= 0 {
						return false
					}
				case 2: // Max
					max := -1
					_, err = fmt.Sscanf(line, "%d", &max)
					if err != nil || max < 0 || max > 255 {
						return false
					}
				}

				step++
				if step == 3 {
					break
				}
			}

			if _, err := file.Data.Seek(0, 0); err != nil {
				panic(err)
			}

			if step != 3 {
				return false
			}
		}
		return true
	}
	return false
}
