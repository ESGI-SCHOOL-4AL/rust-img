from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.so")
print(lib.forty_two())

print("done!")