# create question
curl --location --request POST 'localhost:3030/questions' \
     --header 'Content-Type: application/json' \
     --data-raw '{
        "title": "Do you know Perl 6?",
        "content": "Grammar is awesome",
        "id": "dev"
     }'

# list all question
curl --location --request GET 'localhost:3030/questions'

# update question
curl --location --request PUT 'localhost:3030/questions/dev' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "id": "dev",
        "title": "Do you know Raku?",
        "content": "Grammar is awesome"
    }'

# delete question
curl --location --request DELETE 'localhost:3030/questions/dev'

# post answer
curl --location --request POST 'localhost:3030/answers' \
     --header 'Content-Type: application/x-www-form-urlencoded' \
     --data-urlencode 'id=raku' \
     --data-urlencode 'questionId=dev' \
     --data-urlencode 'content=This is the question I had.'

# post question with bad words
curl --location --request POST 'localhost:3030/questions' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "title": "NEW ass  TITLE",
      "content": "OLD CONTENT shit"
}'