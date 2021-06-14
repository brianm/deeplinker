
tell application "System Events" 
    set frontApp to first application process whose frontmost is true
    set frontAppName to name of frontApp    
    tell process frontAppName
        tell (1st window whose value of attribute "AXMain" is true)
            set windowTitle to value of attribute "AXTitle"
        end tell
    end tell
end tell

set frontAppId to id of application frontAppName

return {id: frontAppId, name: frontAppName, title: windowTitle}