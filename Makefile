GOFILES = $(shell find . -name '*.go' -not -path './vendor/*')
GOPACKAGES = $(shell go list ./...  | grep -v /vendor/)
GIT_DESCR = $(shell git describe --tags --always)
APP=costoflife
# build output folder
OUTPUTFOLDER = target
# docker image
DOCKER_REGISTRY = noandrea
DOCKER_IMAGE = costoflife
DOCKER_TAG = $(GIT_DESCR)
# build paramters
OS = linux
ARCH = amd64
# K8S
K8S_NAMESPACE = default
K8S_DEPLOYMENT = costoflife

.PHONY: list
list:
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$' | xargs


default: build

build: build-web 
	trunk build --release	

build-dist:
	@echo building release
	cargo build --release
	# @echo copy resources
	# cp -r README.md LICENSE $(OUTPUTFOLDER)
	@echo done

build-web:
# befor running this you need to have the tailwindcss raw css: ~/.yarn/bin/taildin
	@echo building release
	NODE_ENV=production ~/.yarn/bin/tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
	@echo done


build-zip: build-dist
	@echo build zip release
	zip -rmT $(APP)-$(GIT_DESCR).zip $(OUTPUTFOLDER)
	sha1sum $(APP)-$(GIT_DESCR).zip
	@echo done

test: 
	RUST_BACKTRACE=1 cargo tarpaulin -o Lcov

test-wasm: 
	wasm-pack test --firefox --headless
	wasm-pack test --node

test-all: test test-wasm

lint: lint-all

lint-all:
	cargo fmt
	# cargo fix
	cargo clippy

clean:
	@echo remove $(OUTPUTFOLDER) folder
	rm -rf $(OUTPUTFOLDER)
	rm -rf pkg
	@echo done

docker: docker-build

docker-build:
	@echo copy resources
	docker build --build-arg DOCKER_TAG='$(GIT_DESCR)' -t $(DOCKER_IMAGE)  .
	@echo done

docker-push:
	@echo push image
	docker tag $(DOCKER_IMAGE):latest $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	@echo done

run-dev: build-web 
	( trunk serve & ~/.yarn/bin/tailwindcss -c ./tailwind.config.js -o ./tailwind.css --watch  )

wasm-build:
	wasm-pack build --release

wasm-publish: wasm-build
	wasm-pack publish

publish:
	cargo publish

publish-all: publish wasm-publish

k8s-deploy:
	@echo deploy k8s
	kubectl -n $(K8S_NAMESPACE) set image deployment/$(K8S_DEPLOYMENT) $(DOCKER_IMAGE)=$(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	@echo done

k8s-rollback:
	@echo deploy k8s
	kubectl -n $(K8S_NAMESPACE) rollout undo deployment/$(K8S_DEPLOYMENT)
	@echo done

changelog:
	git-chglog --output CHANGELOG.md

git-release:
	@echo making release
	git tag $(GIT_DESCR)
	git-chglog --output CHANGELOG.md
	git tag $(GIT_DESCR) --delete
	git add CHANGELOG.md && git commit -m "$(GIT_DESCR)" -m "Changelog: https://github.com/noandrea/$(APP)/blob/master/CHANGELOG.md"
	git tag -s -a "$(GIT_DESCR)" -m "Changelog: https://github.com/noandrea/$(APP)/blob/master/CHANGELOG.md"
	@echo release complete


_release-patch:
	$(eval GIT_DESCR = $(shell git describe --tags | awk -F '("|")' '{ print($$1)}' | awk -F. '{$$NF = $$NF + 1;} 1' | sed 's/ /./g'))
	cargo bump patch
	git add Cargo.toml Cargo.lock
release-patch: _release-patch git-release

_release-minor:
	$(eval GIT_DESCR = $(shell git describe --tags | awk -F '("|")' '{ print($$1)}' | awk -F. '{$$(NF-1) = $$(NF-1) + 1;} 1' | sed 's/ /./g' | awk -F. '{$$(NF) = 0;} 1' | sed 's/ /./g'))
	cargo bump minor
	git add Cargo.toml Cargo.lock
release-minor: _release-minor git-release

_release-major:
	$(eval GIT_DESCR = $(shell git describe --tags | awk -F '("|")' '{ print($$1)}' | awk -F. '{$$(NF-2) = $$(NF-2) + 1;} 1' | sed 's/ /./g' | awk -F. '{$$(NF-1) = 0;} 1' | sed 's/ /./g' | awk -F. '{$$(NF) = 0;} 1' | sed 's/ /./g' ))
	cargo bump major
	git add Cargo.toml Cargo.lock
release-major: _release-major git-release 
