buildlib:
	cd lib/ && cargo build --release
	cp lib/target/release/libembed.so lib/

build: buildlib
	cp lib/libembed.so ./
	go build -a -ldflags="-r ./lib"

run: buildlib
	go run -ldflags="-r ./lib" kernel.go

clean:
	cd lib/ && cargo clean
	go clean
