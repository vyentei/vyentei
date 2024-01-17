#!/usr/bin/python3

import fontforge
import psMat as psmat
import math

TAU = math.tau

def main():
    font = fontforge.open("./src/Vyentei.sfd")
    font.generate("./generated/Vyentei.otf")

    font = fontforge.open("./generated/Vyentei-Oblique.sfd")
    font.generate("./generated/Vyentei-Oblique.otf")

    font = fontforge.open("./src/VyenteiMono.sfd")
    font.generate("./generated/VyenteiMono.otf")

    font = fontforge.open("./generated/VyenteiMono-Oblique.sfd")
    font.generate("./generated/VyenteiMono-Oblique.otf")

    return

main()
