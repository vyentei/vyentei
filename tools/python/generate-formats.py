#!/usr/bin/python3

import fontforge
import psMat as psmat
import math

TAU = math.tau

def generate(sfd, output):
    gen_flags = ("opentype", "old-kern")

    font = fontforge.open(sfd)
    font.generate(output, "", gen_flags)

    return

def main():
    generate("./src/Vyentei.sfd", "./generated/Vyentei.ttf")
    generate("./src/VyenteiMono.sfd", "./generated/VyenteiMono.ttf")
    generate("./generated/Vyentei-Oblique.sfd", "./generated/Vyentei-Oblique.ttf")
    generate("./generated/VyenteiMono-Oblique.sfd", "./generated/VyenteiMono-Oblique.ttf")

    generate("./src/Vyentei.sfd", "./generated/Vyentei.woff")
    generate("./src/VyenteiMono.sfd", "./generated/VyenteiMono.woff")
    generate("./generated/Vyentei-Oblique.sfd", "./generated/Vyentei-Oblique.woff")
    generate("./generated/VyenteiMono-Oblique.sfd", "./generated/VyenteiMono-Oblique.woff")

    return

main()
