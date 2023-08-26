#!/usr/bin/python3

import fontforge
import math
import psMat as psmat

TAU = math.tau

def main():
    font = fontforge.open("./src/QuantiiSansMono.sfd")
    font.selection.all()

    for glyph in font.glyphs():
        width = glyph.width
        glyph.transform(psmat.skew(TAU / 36), ("partialRefs", "round"))
        if glyph.unicode != -1:
            glyph.transform(psmat.translate(-89))

        glyph.width = width

    font.save("./generated/QuantiiSansMono-Oblique.sfd")

    return

main()
