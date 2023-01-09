CARGO          = cargo
CARGO_CCARGS   = 
CARGO_EXAMPLES = custom_engine \
                 custom_key \
				 handlebars \
				 minijinja \
				 nested \
				 tera

################################################################################
# Main goals
ci: CARGO_CCARGS = --all-features --verbose
ci: test build lint

sync: README.md

test:
	$(CARGO) test $(CARGO_CCARGS)

lint: lint/clippy lint/fmt

fmt: 
	$(CARGO) fmt
	
build: build/example build/crate
################################################################################
build/example: $(addprefix build/example/, $(CARGO_EXAMPLES))

build/example/%:
	$(CARGO) build --example=$* $(CARGO_CCARGS) 

build/crate: 
	$(CARGO) build $(CARGO_CCARGS)

lint/clippy:
	$(CARGO) clippy $(CARGO_CCARGS)

lint/fmt:
	$(CARGO) fmt --check

################################################################################
# Make targets
README.md: src/lib.rs
	@echo Updating $@
	$(CARGO) sync-readme

################################################################################

.PHONY: sync test ci \
        $(filter build%, $(MAKECMDGOALS)) \
        $(filter lint%, $(MAKECMDGOALS)) 

