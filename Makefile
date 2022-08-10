CARGO    = cargo
FEATURES = handlebars \
           minijinja \
		   tera

sync: README.md

README.md: src/lib.rs
	@echo Updating $@
	$(CARGO) sync-readme


.PHONY: sync test