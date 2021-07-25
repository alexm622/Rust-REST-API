$Uri = 'http://127.0.0.1:8080/redis'
$Body = '{"key":"testkey", "value": "testvalue"}'
Invoke-RestMethod -Uri $Uri -Method Post -ContentType "application/json" -Body $Body
Invoke-WebRequest -Uri "http://127.0.0.1:8080/redis&key=test"