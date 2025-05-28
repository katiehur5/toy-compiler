FRONTEND = ./front
R_DIR = ./rust
TESTASSEMBLY = testassembly
TESTCASES = tests

FRONT_FILES = $(FRONTEND)/Lexer.c $(FRONTEND)/Parser.c $(FRONTEND)/Expression.c $(FRONTEND)/symtab.c
R_LIB = -L$(R_DIR)/target/release/ -lrust
R_FILES = $(R_DIR)/main.c


CC = gcc
CFLAGS = -g -ansi -std=gnu11
TESTCASES = tests
TESTASSEMBLY = testassembly

all: testopt testcg

323compiler: prebuild $(FRONT_FILES) $(R_FILES)
	$(CC) $(CFLAGS) $(FRONT_FILES) $(R_FILES)  $(R_LIB) -o 323compiler

prebuild:
	@echo "Building Rust library"
	cargo build -r --manifest-path $(R_DIR)/Cargo.toml

testopt: clean 323compiler
	@for i in $(shell ls ${TESTCASES}); do \
		./323compiler ${TESTCASES}/$${i} > test.ir; \
		cp goldIR/$${i}.ir gold.ir; \
		cmp -s test.ir gold.ir; \
		RETVALOPT=$$?; \
		if [ $$RETVALOPT -eq 0 ]; then\
			echo $${i} "OPTIMIZATION PASS"; \
		else \
			echo $${i} "OPTIMIZATION FAIL"; \
		fi \
	done;
	@rm gold.ir
	@rm test.ir

testcg: clean 323compiler
	@for i in $(shell ls ${TESTCASES}); do \
		./323compiler ${TESTCASES}/$${i} > dummy.ir; \
		gcc ${TESTASSEMBLY}/main.c assembly.s -o test; \
		./test > test.cg; \
		gcc -S ${TESTCASES}/$${i} -o temp.s; \
		gcc ${TESTASSEMBLY}/main.c temp.s -o gold; \
		./gold > gold.cg; \
		cmp -s test.cg gold.cg; \
		RETVALCG=$$?; \
		if [ $$RETVALCG -eq 0 ]; then\
			echo $${i} "CODEGEN PASS"; \
		else \
			echo $${i} "CODEGEN FAIL"; \
		fi \
	done;	
	@rm dummy.ir
	@rm gold.cg
	@rm test.cg

clean:
	rm -f *.o *~  323compiler *.s a.out *.ir *.cg
