# VHS documentation
#
# Output:
#   Output <path>.gif               Create a GIF output at the given <path>
#   Output <path>.mp4               Create an MP4 output at the given <path>
#   Output <path>.webm              Create a WebM output at the given <path>
#
# Settings:
#   Set FontSize <number>           Set the font size of the terminal
#   Set FontFamily <string>         Set the font family of the terminal
#   Set Height <number>             Set the height of the terminal
#   Set Width <number>              Set the width of the terminal
#   Set LetterSpacing <float>       Set the font letter spacing (tracking)
#   Set LineHeight <float>          Set the font line height
#   Set Theme <json|string>         Set the theme of the terminal
#   Set Padding <number>            Set the padding of the terminal
#   Set Framerate <number>          Set the framerate of the recording
#   Set PlaybackSpeed <float>       Set the playback speed of the recording
#

Output demo.gif

Set FontSize 14
Set Width 1200
Set Height 800
Set Padding 20
Set Theme "Dracula"
Set Framerate 30

# Start of the demo
Type "nman"
Sleep 500ms
Enter

Sleep 3s

# Toggle Tree View
Type "T"
Sleep 2s

# Toggle back
Type "T"
Sleep 1s

# Cycle to Services view
Type "V"
Sleep 2s

# Cycle to Docker view
Type "V"
Sleep 2s

# Search for something
Type "/"
Sleep 500ms
Type "node"
Sleep 2s
Escape

# Open Command Palette
Type ":"
Sleep 500ms
Type "logs"
Sleep 2s
Escape
Sleep 1s

# Quit
Type "q"
Sleep 1s
