#!/usr/bin/python3

import fontforge
import psMat as psmat
import math

TAU = math.tau

def main():
    font = fontforge.open("./src/QuantiiSans.sfd")
    font.selection.all()

    for glyph in font.glyphs():
        width = glyph.width
        glyph.transform(psmat.skew(TAU / 36), ("partialRefs", "round"))
        if glyph.unicode != -1:
            glyph.transform(psmat.translate(-89))

        glyph.width = width

    font.save("./generated/QuantiiSans-Oblique.sfd")

    return

main()
