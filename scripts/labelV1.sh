# delete default label
gh label delete "bug" --yes || true
gh label delete "enhancement" --yes || true

# create new label
gh label create "type: bug" --color "d73a4a" --description "Something isn't working"
gh label create "type: feature" --color "a2eeef" --description "New feature or request"
gh label create "type: refactor" --color "e99695" --description "Code improvement"
gh label create "priority: high" --color "b60205" --description "Must be addressed ASAP"
gh label create "status: help wanted" --color "008672" --description "Community help requested"
gh label create "size: large" --color "ee9900" --description "Complex task requiring time"
gh label create "epic" --color "3E4B9E" --description "Large scope feature"