OUTFILE=inds

default: compile

compile c:
	rustc src/main.rs -o $(OUTFILE)

run r: compile
	./$(OUTFILE)

clean cl:
	rm $(OUTFILE)
