# adapted from https://gist.github.com/dongyuwei/a1c9d67e4af6bbbd999c

tell application "System Events" to set frontApp to name of first process whose frontmost is true

using terms from application "Google Chrome"
    tell application frontApp to set currentTabUrl to URL of active tab of front window
    tell application frontApp to set currentTabTitle to title of active tab of front window
end using terms from

return {link: currentTabUrl, title: currentTabTitle}