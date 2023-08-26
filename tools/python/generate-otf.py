#!/usr/bin/python3

import fontforge
import psMat as psmat
import math

TAU = math.tau

def main():
    font = fontforge.open("./src/QuantiiSans.sfd")
    font.generate("./generated/QuantiiSans.otf")

    font = fontforge.open("./generated/QuantiiSans-Oblique.sfd")
    font.generate("./generated/QuantiiSans-Oblique.otf")

    font = fontforge.open("./src/QuantiiSansMono.sfd")
    font.generate("./generated/QuantiiSansMono.otf")

    font = fontforge.open("./generated/QuantiiSansMono-Oblique.sfd")
    font.generate("./generated/QuantiiSansMono-Oblique.otf")

    return

main()
