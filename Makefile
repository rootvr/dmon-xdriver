XDRIVER_CARGO := cargo

XDRIVER_TARGET := dmon-xdriver

PREFIX := .
DESTDIR := target/release

IPREFIX := ${HOME}
IDESTDIR := .local/bin

XDRIVER_FMT := rustfmt

XDRIVER_ARGS := --log-dirname ./data \
				--log-maxrows 100 \
				--filter none \
				--redis-ipv4 0.0.0.0 \
				--redis-port 6379 \
				--wgen-workload-file ../dmon-analysis/test/config/robotshop/workload.yml \
				--wgen-apispec-file ../dmon-analysis/test/config/robotshop/apispec.yml \
				--wgen-day-length 30s

.SILENT: help
.PHONY: help # print help
help:
	grep '^.PHONY: .* #' $(firstword $(MAKEFILE_LIST)) |\
	sed 's/\.PHONY: \(.*\) # \(.*\)/\1 # \2/' |\
	awk 'BEGIN {FS = "#"}; {printf "%-20s %s\n", $$1, $$2}' 

.PHONY: build # compile dmon-xdriver
build:
	$(XDRIVER_CARGO) b -r

.PHONY: clean # clean and remove dmon-xdriver binary
clean:
	-rm -rf ./data
	$(XDRIVER_CARGO) clean

.PHONY: run # compile and run dmon-xdriver
run:
	clear
	$(XDRIVER_CARGO) r -- $(XDRIVER_ARGS)

.PHONY: install # install dmon-xdriver into ~/.local/bin
install:
	install -D $(PREFIX)/$(DESTDIR)/$(XDRIVER_TARGET) $(IPREFIX)/$(IDESTDIR)/$(XDRIVER_TARGET)

.PHONY: uninstall # unininstall dmon-xdriver from ~/.local/bin
uninstall:
	rm -f $(IPREFIX)/$(IDESTDIR)/$(XDRIVER_TARGET)

.SILENT: format
.PHONY: format # format all
format:
	$(XDRIVER_CARGO) fmt -- -l
