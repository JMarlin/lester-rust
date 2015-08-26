from ctypes import cdll

lib = cdll.LoadLibrary("target/release/liblester.so")

lib.process()

print("done!")
