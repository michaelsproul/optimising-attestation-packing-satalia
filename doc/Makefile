.PHONY: all open clean wipe

all: problem.pdf abridged.pdf approaches.pdf report.pdf

problem.pdf: problem.tex problem.bib
	pdflatex problem; bibtex problem; pdflatex problem; pdflatex problem;
	make clean

abridged.pdf: abridged.tex abridged.bib
	pdflatex abridged; bibtex abridged; pdflatex abridged; pdflatex problem;
	make clean

approaches.pdf: approaches.tex approaches.bib
	pdflatex approaches; bibtex approaches; pdflatex approaches; pdflatex approaches;
	make clean

report.pdf: report.tex report.bib
	pdflatex report; bibtex report; pdflatex report; pdflatex report;
	make clean

clean:
	rm -rf *.aux *.bbl *.blg *.log *.out *.toc

wipe: clean
	rm -rf *.pdf
