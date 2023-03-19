import pygame
import sys

width, height = 0, 0
pixels = []

if sys.argc < 2:
    print("Error: no file to display")
    sys.exit(1)

with open(sys.argv[1], "rb") as file:
    filetype, dimensions, pixels = file.readlines()
    width, height = map(int, dimensions.decode('utf-8')[:-1].split()[:-1])
    pixels = [int(pixel) for pixel in pixels]
    pixels = [tuple(pixels[i:i + 3]) for i in range(0, len(pixels), 3)]

screen = pygame.display.set_mode((400, 400))
imagesurface = pygame.Surface((width, height))
imagepixels = pygame.PixelArray(imagesurface)
for y in range(height):
    imagepixels[:, y] = pixels[width*y:width*y + width]

del imagepixels

pygame.display.flip()

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    screen.fill((0, 0, 0))
    screen.blit(imagesurface, (100, 100))
    pygame.draw.rect(screen, (255, 0, 0),
                     pygame.rect.Rect(100, 100, width, height), 1)
    pygame.display.flip()
