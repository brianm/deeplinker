# adapted from # adapted from https://gist.github.com/dongyuwei/a1c9d67e4af6bbbd999c

tell application "System Events" to set frontApp to name of first process whose frontmost is true

using terms from application "Safari"
    tell application frontApp to set currentTabUrl to URL of front document
    tell application frontApp to set currentTabTitle to name of front document
end using terms from

return currentTabUrl & "\n" & currentTabTitle