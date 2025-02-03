complete -c qrrs -s o -l output-format -d 'Format in which the qrcode will be saved' -r -f -a "{image\t'',svg\t'',unicode\t''}"
complete -c qrrs -s m -l margin -d 'Margin applied to qrcode' -r
complete -c qrrs -l generate-completions -d 'Generate completion file for the specified shell' -r -f -a "{bash\t'',elvish\t'',fish\t'',powershell\t'',zsh\t''}"
complete -c qrrs -s e -l error-correction-level -d 'Error correction level for the QR code' -r -f -a "{low\t'',l\t'',medium\t'',m\t'',quartile\t'',q\t'',high\t'',h\t''}"
complete -c qrrs -s r -l read -d 'Read the qrcode instead of generating it'
complete -c qrrs -s t -l terminal -d 'Display code in terminal'
complete -c qrrs -s i -l invert_colors -d 'Invert qrcode colors'
complete -c qrrs -s h -l help -d 'Print help'
complete -c qrrs -s V -l version -d 'Print version'
