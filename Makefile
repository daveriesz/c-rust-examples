
CPROGRAMS = $(subst .c,,$(subst C-,C/bin/,$(subst /,-,$(shell find C -name [0-9]*.c))))
CC        = gcc
CFLAGS    = -ansi -pedantic -lm

RPROGRAMS = $(subst .rs,,$(subst rustlang-,rustlang/bin/,$(subst /,-,$(shell find rustlang -name [0-9]*.rs))))
RUSTC     = rustc
RUSTFLAGS = 

all: $(CPROGRAMS) $(RUSTPROGRAMS)

cprogs: $(CPROGRAMS)
rprogs: $(RPROGRAMS)

C/bin/1-%: C/1/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/2-%: C/2/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/3-%: C/3/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/4-%: C/4/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/5-%: C/5/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/6-%: C/6/%.c
	$(CC) $< -o $@ $(CFLAGS)
C/bin/4-11: C/4/11/*
	$(CC) C/4/11/main.c C/4/11/getop.c C/4/11/stack.c C/4/11/getch.c -o $@ $(CFLAGS)

rustlang/bin/1-%: rustlang/1/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/2-%: rustlang/2/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/3-%: rustlang/3/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/4-%: rustlang/4/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/5-%: rustlang/5/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/6-%: rustlang/6/%.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
rustlang/bin/4-11:
	$(RUSTC) $(RUSTFLAGS) $< -o $@

clean: c-clean rust-clean
c-clean:
	$(RM) $(CPROGRAMS) C/bin/4-11
rust-clean:
	$(RM) $(RPROGRAMS)

