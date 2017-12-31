docker-name = "acmumn/mentoring"

all: build build-docker run-docker
build:
	mkdir -p build
	docker build
clean:
	just -d server -f server/Justfile clean
	just -d web-client -f web-client/Justfile clean
	rm -r build

dev-client:
	watchexec -cre css,html,rs,toml \
		-w web-client/src \
		-w web-client/static \
		just dev-client-reload
dev-client-reload:
	mkdir -p build/tmp
	just -d web-client -f web-client/Justfile static-debug
	rsync -a web-client/target/static/ build/tmp/static/

package: build-local
	tar czf build/mentoring.tgz -C build/tmp .

build-docker:
	echo TODO
	false
run-docker:
	echo TODO
	false

build-local: build-local-client build-local-server
build-local-client:
	mkdir -p build/tmp
	just -d web-client -f web-client/Justfile static-release
	rsync -a web-client/target/static/ build/tmp/static/
build-local-server:
	mkdir -p build/tmp
	just -d server -f server/Justfile build-release
	cp server/defaults.env build/tmp/.env
	cp server/target/release/mentoring-server build/tmp/mentoring
run-local: build-local
	cd build/tmp && ./mentoring
