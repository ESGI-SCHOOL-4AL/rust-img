buildlib:
	cd lib/ && cargo +nightly build --release
	cp lib/target/release/libembed.so lib/

build: buildlib
	go build -a -ldflags="-r ./lib"

run: buildlib
	go run -ldflags="-r ./lib" kernel.go

clean:
	cd lib/ && cargo clean
	go clean
