tell application "System Events" to set frontApp to name of first process whose frontmost is true
return frontApp