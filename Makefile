.PHONY: all check munge full sans ttf full-ttf sans-ttf status dist src-dist full-dist sans-dist norm check-harder pre-patch clean

# Release version
VERSION = 0.2
# Snapshot version
SNAPSHOT =
# Initial source directory, assumed read-only
SRCDIR  = src
# Directory where temporary files live
TMPDIR  = tmp
# Directory where final files are created
BUILDDIR  = build
# Directory where final archives are created
DISTDIR = dist

# Release layout
FONTCONFDIR = fontconfig
DOCDIR = .
SCRIPTSDIR = scripts
TTFDIR = ttf
RESOURCEDIR = resources

ifeq "$(SNAPSHOT)" ""
ARCHIVEVER = $(VERSION)
else
ARCHIVEVER = $(VERSION)-$(SNAPSHOT)
endif

SRCARCHIVE  = quantii-sans-fonts-$(ARCHIVEVER)
FULLARCHIVE = quantii-sans-fonts-ttf-$(ARCHIVEVER)
SANSARCHIVE = quantii-sans-ttf-$(ARCHIVEVER)

ARCHIVEEXT = .zip .tar.bz2
SUMEXT     = .zip.md5 .tar.bz2.md5 .tar.bz2.sha512

OLDSTATUS   = $(DOCDIR)/status.txt
BLOCKS      = $(RESOURCEDIR)/Blocks.txt
UNICODEDATA = $(RESOURCEDIR)/UnicodeData.txt
FC-LANG     = $(RESOURCEDIR)/fc-lang

GENERATE    = $(SCRIPTSDIR)/generate.pe
TTPOSTPROC  = $(SCRIPTSDIR)/ttpostproc.pl
UNICOVER    = $(SCRIPTSDIR)/unicover.pl
LANGCOVER   = $(SCRIPTSDIR)/langcover.pl
STATUS	    = $(SCRIPTSDIR)/status.pl
PROBLEMS    = $(SCRIPTSDIR)/problems.pl
NORMALIZE   = $(SCRIPTSDIR)/sfdnormalize.pl
NARROW      = $(SCRIPTSDIR)/narrow.pe

SRC      := $(wildcard $(SRCDIR)/*.sfd)
SFDFILES := $(patsubst $(SRCDIR)/%, %, $(SRC))
FULLSFD  := $(patsubst $(SRCDIR)/%.sfd, $(TMPDIR)/%.sfd, $(SRC))
NORMSFD  := $(patsubst %, %.norm, $(FULLSFD))
MATSHSFD := $(wildcard $(SRCDIR)/*Math*.sfd)
FULLTTF  := $(patsubst $(TMPDIR)/%.sfd, $(BUILDDIR)/%.ttf, $(FULLSFD))

FONTCONF     := $(wildcard $(FONTCONFDIR)/*.conf)

STATICDOC := $(addprefix $(DOCDIR)/, AUTHORS LICENSE NEWS README.md)
STATICSRCDOC := $(addprefix $(DOCDIR)/, BUILDING.md)
GENDOCFULL = unicover.txt langcover.txt status.txt
GENDOCSANS = unicover-sans.txt langcover-sans.txt

all : full sans

$(TMPDIR)/%.sfd: $(SRCDIR)/%.sfd
	@echo "[1] $< => $@"
	install -d $(dir $@)
	sed "s@\(Version:\? \)\(0\.[0-9]\+\.[0-9]\+\|[1-9][0-9]*\.[0-9]\+\)@\1$(VERSION)@" $< > $@
	touch -r $< $@

$(BUILDDIR)/%.ttf: $(TMPDIR)/%.sfd
	@echo "[3] $< => $@"
	install -d $(dir $@)
	$(GENERATE) $<
	mv $<.ttf $@
	$(TTPOSTPROC) $@
	$(RM) $@~
	touch -r $< $@

$(BUILDDIR)/status.txt: $(FULLSFD)
	@echo "[4] => $@"
	install -d $(dir $@)
	$(STATUS) $(VERSION) $(OLDSTATUS) $(FULLSFD) > $@

$(BUILDDIR)/unicover.txt: $(patsubst %, $(TMPDIR)/%.sfd, Vyentei VyenteiMono)
	@echo "[5] => $@"
	install -d $(dir $@)
	$(UNICOVER) $(UNICODEDATA) $(BLOCKS) \
	            $(TMPDIR)/Vyentei.sfd "Sans" \
	            $(TMPDIR)/VyenteiMono.sfd "Sans Mono" > $@

$(BUILDDIR)/unicover-sans.txt: $(TMPDIR)/Vyentei.sfd
	@echo "[5] => $@"
	install -d $(dir $@)
	$(UNICOVER) $(UNICODEDATA) $(BLOCKS) \
	            $(TMPDIR)/Vyentei.sfd "Sans" > $@

$(BUILDDIR)/langcover.txt: $(patsubst %, $(TMPDIR)/%.sfd, Vyentei VyenteiMono)
	@echo "[6] => $@"
	install -d $(dir $@)
ifeq "$(FC-LANG)" ""
	touch $@
else
	$(LANGCOVER) $(FC-LANG) \
	             $(TMPDIR)/Vyentei.sfd "Sans" \
	             $(TMPDIR)/VyenteiMono.sfd "Sans Mono" > $@
endif

$(BUILDDIR)/langcover-sans.txt: $(TMPDIR)/Vyentei.sfd
	@echo "[6] => $@"
	install -d $(dir $@)
ifeq "$(FC-LANG)" ""
	touch $@
else
	$(LANGCOVER) $(FC-LANG) \
	             $(TMPDIR)/Vyentei.sfd "Sans" > $@
endif

$(BUILDDIR)/Makefile: Makefile
	@echo "[7] => $@"
	install -d $(dir $@)
	sed -e "s+^VERSION\([[:space:]]*\)=\(.*\)+VERSION = $(VERSION)+g"\
	    -e "s+^SNAPSHOT\([[:space:]]*\)=\(.*\)+SNAPSHOT = $(SNAPSHOT)+g" < $< > $@
	touch -r $< $@

$(TMPDIR)/$(SRCARCHIVE): $(addprefix $(BUILDDIR)/, $(GENDOCFULL) Makefile) $(FULLSFD)
	@echo "[8] => $@"
	install -d -m 0755 $@/$(SCRIPTSDIR)
	install -d -m 0755 $@/$(SRCDIR)
	install -d -m 0755 $@/$(FONTCONFDIR)
	install -d -m 0755 $@/$(DOCDIR)
	install -p -m 0644 $(BUILDDIR)/Makefile $@
	install -p -m 0755 $(GENERATE) $(TTPOSTPROC) $(NORMALIZE) \
	                   $(UNICOVER) $(LANGCOVER) $(STATUS) $(PROBLEMS) \
	                   $@/$(SCRIPTSDIR)
	install -p -m 0644 $(FULLSFD) $@/$(SRCDIR)
	install -p -m 0644 $(FONTCONF) $@/$(FONTCONFDIR)
	install -p -m 0644 $(addprefix $(BUILDDIR)/, $(GENDOCFULL)) \
	                   $(STATICDOC) $(STATICSRCDOC) $@/$(DOCDIR)

$(TMPDIR)/$(FULLARCHIVE): full
	@echo "[8] => $@"
	install -d -m 0755 $@/$(TTFDIR)
	install -d -m 0755 $@/$(FONTCONFDIR)
	install -d -m 0755 $@/$(DOCDIR)
	install -p -m 0644 $(FULLTTF) $@/$(TTFDIR)
	install -p -m 0644 $(FONTCONFFULL) $@/$(FONTCONFDIR)
	install -p -m 0644 $(addprefix $(BUILDDIR)/, $(GENDOCFULL)) \
	                   $(STATICDOC) $@/$(DOCDIR)

$(TMPDIR)/$(SANSARCHIVE): sans
	@echo "[8] => $@"
	install -d -m 0755 $@/$(TTFDIR)
	install -d -m 0755 $@/$(DOCDIR)
	install -p -m 0644 $(BUILDDIR)/Vyentei.ttf $@/$(TTFDIR)
	install -p -m 0644 $(addprefix $(BUILDDIR)/, $(GENDOCSANS)) \
	                   $(STATICDOC) $@/$(DOCDIR)

$(DISTDIR)/%.zip: $(TMPDIR)/%
	@echo "[9] => $@"
	install -d $(dir $@)
	(cd $(TMPDIR); zip -rv $(abspath $@) $(notdir $<))

$(DISTDIR)/%.tar.bz2: $(TMPDIR)/%
	@echo "[9] => $@"
	install -d $(dir $@)
	(cd $(TMPDIR); tar cjvf $(abspath $@) $(notdir $<))

%.md5: %
	@echo "[10] => $@"
	(cd $(dir $<); md5sum -b $(notdir $<) > $(abspath $@))

%.sha512: %
	@echo "[10] => $@"
	(cd $(dir $<); sha512sum -b $(notdir $<) > $(abspath $@))

%.sfd.norm: %.sfd
	@echo "[11] $< => $@"
	$(NORMALIZE) $<
	touch -r $< $@

check : $(NORMSFD)
	for sfd in $^ ; do \
	echo "[12] Checking $$sfd" ;\
	$(PROBLEMS)  $$sfd ;\
	done

munge: $(NORMSFD)
	for sfd in $(SFDFILES) ; do \
	echo "[13] $(TMPDIR)/$$sfd.norm => $(SRCDIR)/$$sfd" ;\
	cp $(TMPDIR)/$$sfd.norm $(SRCDIR)/$$sfd ;\
	done

full : $(FULLTTF) $(addprefix $(BUILDDIR)/, $(GENDOCFULL))

sans : $(addprefix $(BUILDDIR)/, Vyentei.ttf $(GENDOCSANS))

ttf : full-ttf sans-ttf

full-ttf : $(FULLTTF)

sans-ttf: $(BUILDDIR)/Vyentei.ttf

status : $(addprefix $(BUILDDIR)/, $(GENDOCFULL))

dist : src-dist full-dist sans-dist

src-dist :  $(addprefix $(DISTDIR)/$(SRCARCHIVE),  $(ARCHIVEEXT) $(SUMEXT))

full-dist : $(addprefix $(DISTDIR)/$(FULLARCHIVE), $(ARCHIVEEXT) $(SUMEXT))

sans-dist : $(addprefix $(DISTDIR)/$(SANSARCHIVE), $(ARCHIVEEXT) $(SUMEXT))

norm : $(NORMSFD)

check-harder : clean check

pre-patch : munge clean

clean :
	$(RM) -r $(TMPDIR) $(BUILDDIR) $(DISTDIR)
