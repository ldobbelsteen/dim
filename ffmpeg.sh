/usr/bin/ffmpeg -fflags +genpts -noaccurate_seek -f matroska,webm -i file:"/home/hinach4n/media/media1/movies/Alita.Battle.Angel.2019.1080p.BluRay.x264.DTS-HD.MA.7.1-FGT/Alita.Battle.Angel.2019.1080p.BluRay.x264.DTS-HD.MA.7.1-FGT.mkv" -map_metadata -1 -map_chapters -1 -threads 0 -map 0:0 -map 0:1 -map -0:s -codec:v:0 libx264 -bsf:v h264_mp4toannexb -copyts -vsync -1 -codec:a:0 aac -strict experimental -ac 2 -ab 384000 -af "volume=2" -f segment -max_delay 5000000 -avoid_negative_ts disabled -start_at_zero -segment_time 6  -individual_header_trailer 0 -break_non_keyframes 1 -segment_format mpegts -segment_list_type m3u8 -segment_start_number 0 -segment_list "index.m3u8" -y "./%d.ts"
