
# Deeplinker

A CLI tool, and maybe a library, to use applescript to detect frontmost app in MacOS, a deeplink to its current state, and some descriptive context for the current state.

The goal is a utility for quickly capturing a linkl to current place for notes, reminders, and things like that.

## Status

Proof of concept, playing around with approaches. Not actually very useful... yet :-)

Eventually will provide an easy mechanism to add support for different scriptable apps (hence making it OSS, and keeping it really simple.

Currently in rust, because I like rust. May switch to swift if rust starts getting in the way as it is a more natural language for this tool.

## Non-Universal Link Apps

When an app does not support deep links, but can be forced to open to a particular state (doc, etc), we may want to support an "open" mode which takes URLs generated by deeplinker and opens up the application to the correct state.

This implies some non-portable `deeplinker:com.googlecode.iterm2:/etc` scheme which then leads to something like:

```applescript
on run argv
	tell application "iTerm"
		set newWindow to (create window with default profile)
		tell newWindow
			tell current session
				write text "cd " & quoted form of (item 1 of argv)
			end tell
		end tell
	end tell
end run
```

With the argv a *thoroughly* scrubbed and validated path, as this particular scheme opens up some deeply scary abuse potential.

## Specific Apps

* [Apple Mail](https://apple.stackexchange.com/questions/300437/is-it-possible-to-deep-link-to-a-specific-email-in-mail-app-on-mac-os-x)
* [Apple Mail](https://nshipster.com/message-id/)
* [Outlook](https://answers.microsoft.com/en-us/msoffice/forum/msoffice_outlook-mso_mac-mso_mac2016/linking-directly-to-messages-on-outlook-2016-for/c616cd42-098e-43b5-be86-7c04bf117ba0)

### iTerm2

Gross script to capture PWD:

```applescript
tell application id "com.googlecode.iterm2"
	tell the current window
		tell the current session
			write text "pwd"
			set sessionContents to its text -- Grammar Police 
		end tell
	end tell
	
	set myPath to paragraph -2 of (do shell script "grep . <<< " & quoted form of sessionContents)
end tell

return {path:myPath}
```
