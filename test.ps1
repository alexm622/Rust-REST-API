$Uri = 'http://127.0.0.1:8080/next'
$Body = '{"last_track":"7ouMYWpwJ422jRcDASZB7P", "current_track":"4VqPOruhp5EdPBeR92t6lQ", "preference":true, "uid":"2", "user_token":"token"}'
#Invoke-WebRequest -Uri "http://127.0.0.1:8080/redis&key=testkey"
Invoke-RestMethod -Uri $Uri -Method Post -ContentType "application/json" -Body $Body
