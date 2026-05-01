#compdef clipcat

_arguments \
  '--head[Copy only first N lines]:lines:(10 20 50 100)' \
  '--tail[Copy only last N lines]:lines:(10 20 50 100)' \
  '--force[Bypass size guard]' \
  '--max-bytes[Maximum file size before refusing copy]:size:(1K 5M 10M 100M 1G)' \
  '--no-color[Disable ANSI colors]' \
  '--help[Print help]' \
  '--version[Print version]' \
  '*:file:_files'
