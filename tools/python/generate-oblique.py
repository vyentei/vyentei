#!/usr/bin/python3

import fontforge
import math
import psMat as psmat
import sys

TAU = math.tau

def main():
    marks = sys.argv[1].split(' ')

    font = fontforge.open("./src/QuantiiSans.sfd")
    font.selection.all()

    for glyph in font.glyphs():
        width = glyph.width
        glyph.transform(psmat.skew(TAU / 36), ("partialRefs", "round"))
        if glyph.unicode != -1 and str(glyph.unicode) not in marks:
            glyph.transform(psmat.translate(-89))

        glyph.width = width

    font.save("./generated/QuantiiSans-Oblique.sfd")

    return

main()
