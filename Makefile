#
# Makefile
#
# This makefile helps you building the document and testing the sources.
#

#
# The latex compiler
#
LATEX=$(shell which pdflatex)
export LATEX

#
# The output command
#
ECHO_CMD=$(shell which echo)

#
# The command to remove something
#
RM_CMD=$(shell which rm)


##
## Flags
##

#
# Flags for subdirectories
#
MAKE_FLAGS=--no-print-directory
export MAKE_FLAGS

#
# Flags for the latex compilre
#
LATEX_FLAGS=-halt-on-error -shell-escape
export LATEX_FLAGS

#
# Output flags
#
ECHO_FLAGS=-e
export ECHO_FLAGS

#
# The remove flags
#
RM_FLAGS=-f


##
## Standard commands
##

#
# The command for printing something
#
ECHO=$(ECHO_CMD) $(ECHO_FLAGS)
export ECHO

RM=$(RM_CMD) $(RM_FLAGS)
export RM


##
## Files
##


#
# The PDF file we want
#
PDF_FILE=main.pdf

#
# The latex source file
#
LATEX_MAIN_SOURCE=main.tex

#
# All latex sources
#
LATEX_SUB_SOURCES:=$(shell find src -iname '*.tex')

#
# The main.aux file
#
AUX_FILE=main.aux

#
# .ist file
#
IST_FILE=main.ist

#
# Destination of the log from the make files
#
MAKELOG=make.log
export MAKELOG

##
## Misc vars
##

# Avoid strange shell setups, as proposed by the GNU coding standards
SHELL = /bin/sh

#
# removeable latex temporary files
#
LATEX_TMPFILES_FLAT=log nav out snm toc vrb bbl blg idx lof lot glg glo gls ist glsdefs
LATEX_TMPFILES_RECURSIVE=aux



##
## High level tasks
##


#
# The standard task, compiling the latex source to a
# pdf document.
#
# We compile the sources three times, to be sure everything compiled correctly
#
all: $(PDF_FILE)

#
# Testing task. Not meant to produce a pdf but to check for flaws in the source
# files
#
test one_time: $(AUX_FILE) $(BBL_FILE) $(GLS_FILE)


##
## Generation tasks
##

#
# Generate main.aux from TeX sources
#
$(AUX_FILE): $(LATEX_MAIN_SOURCE) $(LATEX_SUB_SOURCES)
	@$(ECHO) "\t[LATEX]"
	@$(LATEX) $(LATEX_FLAGS) -draftmode $< >> $(MAKELOG)


#
# "Generate" .ist and .glo file simply by depending on the auxilary file
#
$(IST_FILE) $(GLO_FILE): $(AUX_FILE)


#
# Generate glossaries
#
$(GLS_FILE): $(GLO_FILE) $(IST_FILE)
	@$(ECHO) "\t[GLOSSARY]"
	@$(GLOSSARY) $(basename $(LATEX_MAIN_SOURCE)) >> $(MAKELOG)

#
# Generate the PDF TODO: use stamp file and [[ -nt ]] or [[ -ot ]] (maybe in script)
#
$(PDF_FILE): $(AUX_FILE)
	@$(ECHO) "\t[LATEX]"
	@$(LATEX) $(LATEX_FLAGS) $(LATEX_MAIN_SOURCE) >> $(MAKELOG)

	@$(ECHO) "\t[LATEX]"
	@$(LATEX) $(LATEX_FLAGS) $(LATEX_MAIN_SOURCE) >> $(MAKELOG)

##
## Clean tasks
##


#
# Clean everything
#
clean: nearly_clean
	@$(ECHO) "\t[RM] *.pdf"
	@$(RM) "*.pdf"

#
# Remove everything except the target pdf file
#
nearly_clean:
	@for tmp in $(LATEX_TMPFILES_FLAT);do		\
		$(ECHO) "\t[RM]\t*.$$tmp";				\
		$(RM) *.$$tmp;							\
	done
	@for tmp in $(LATEX_TMPFILES_RECURSIVE);do	\
		$(ECHO) "\t[RM]\t*.$$tmp";				\
		$(RM) $$(find -name *.$$tmp -type f);	\
	done
	@$(ECHO) "\t[RM]\t$(MAKELOG)"
	@$(RM) $(MAKELOG)

