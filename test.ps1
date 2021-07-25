$Uri = 'http://127.0.0.1:8080/redis'
$Body = '{"key":"testkey", "value": "testvalue"}'
Invoke-WebRequest -Uri "http://127.0.0.1:8080/redis&key=testkey"
Start-Sleep 1
Invoke-RestMethod -Uri $Uri -Method Post -ContentType "application/json" -Body $Body
