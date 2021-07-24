$Uri = 'http://127.0.0.1:8080/posttest'
$Body = '{"id":1,"name":"test"}'
Invoke-RestMethod -Uri $Uri -Method Post -ContentType "application/json" -Body $Body