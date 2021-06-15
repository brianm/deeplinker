
tell application id "com.googlecode.iterm2"
	tell the current window
		tell the current session
			set sessionName to get name
            write text "pwd"
			set sessionContents to its text -- Grammar Police
		end tell
	end tell
    set myPath to paragraph -2 of (do shell script "grep . <<< " & quoted form of sessionContents)
end tell

return {link: "deeplinker:com.googlecode.iterm2:/" & myPath, title:sessionName}