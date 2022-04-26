#!/bin/bash

seq 0 100 | xargs -I{} convert \
  /Users/nathan/Documents/workspace/raytracer/output/radiosity/{}.hdr.pq \
  -set colorspace RGB \
  /Users/nathan/Documents/workspace/raytracer/output/exr/{}.exr

ffmpeg \
 -y `# overwrite` \
 -t 20 `# seconds` \
 -f image2 `# convert images to video` \
 -r 30 `# input framerate` \
 -i 'output/exr/%01d.exr' `#format string for output` \
 -r 30 `# output framerate` \
 -c:v libx265 `#encoder` \
 -tag:v hvc1 `#???` \
 -crf 22 `#okay compression` \
 -pix_fmt yuv420p10le `#output pixel format` \
 -x265-params "colorprim=bt2020:transfer=smpte2084:colormatrix=bt2020nc" \
  out.mp4

# yes | rav1e out.y4m -o output.ivf --transfer=SMPTE2084
# yes | ffmpeg -i /Users/nathan/Documents/workspace/raytracer/output.ivf -vcodec copy output.webm