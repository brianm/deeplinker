tell application id "com.apple.mail"
	set theSelectedMessages to selection
	set the selected_message to item 1 of the theSelectedMessages
	set message_id to the message id of the selected_message
	set message_title to the subject of the selected_message
end tell

return {link:"message://%3c" & message_id & "%3E", title:message_title}