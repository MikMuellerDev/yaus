version := 0.1.0

docker:
	cross build \
		--target-dir=../target \
		--target=x86_64-unknown-linux-musl \
		--release
	cp ../target/x86_64-unknown-linux-musl/release/yaus .
	docker build . \
		-t mikmuellerdev/yaus:$(version) \
		-t mikmuellerdev/yaus:latest

docker-push:
	docker push mikmuellerdev/yaus:$(version)
	docker push mikmuellerdev/yaus:latest

