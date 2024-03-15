curl -v --header "Content-Type: application/json" \
  --request POST \
  --data '{"topicName":"abc.com","message":{"username":"xyz","password":"xyz"}}' \
  http://localhost:8888/post_message
