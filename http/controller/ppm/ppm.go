package ppm

/*
#cgo LDFLAGS: -L../../../lib -lembed
#include <stdlib.h>
#include "../../../lib/libembed.h"
*/
import "C"

import (
	"strconv"
	"time"
	"unsafe"

	"github.com/System-Glitch/goyave/v2"
	"github.com/System-Glitch/goyave/v2/helper/filesystem"
)

// Grayscale takes the input image and returns it with a grayscale filter.
func Grayscale(response *goyave.Response, request *goyave.Request) {
	name := strconv.FormatInt(time.Now().UnixNano()/int64(time.Millisecond), 10)
	srcPath := "storage/" + request.File("image")[0].Save("storage", name+".ppm")
	dstPath := srcPath + "_dst.ppm"

	src := C.CString(srcPath)
	dst := C.CString(dstPath)
	C.grayscale(src, dst)
	C.free(unsafe.Pointer(src))
	C.free(unsafe.Pointer(dst))
	response.Header().Set("Content-Type", "image/x-portable-pixmap")
	response.Download(dstPath, "grayscale.ppm")
	filesystem.Delete(srcPath)
	filesystem.Delete(dstPath)
}

// Invert takes the input image and returns it with a invert filter.
func Invert(response *goyave.Response, request *goyave.Request) {
	name := strconv.FormatInt(time.Now().UnixNano()/int64(time.Millisecond), 10)
	srcPath := "storage/" + request.File("image")[0].Save("storage", name+".ppm")
	dstPath := srcPath + "_dst.ppm"

	src := C.CString(srcPath)
	dst := C.CString(dstPath)
	C.invert(src, dst)
	C.free(unsafe.Pointer(src))
	C.free(unsafe.Pointer(dst))
	response.Header().Set("Content-Type", "image/x-portable-pixmap")
	response.Download(dstPath, "invert.ppm")
	filesystem.Delete(srcPath)
	filesystem.Delete(dstPath)
}
