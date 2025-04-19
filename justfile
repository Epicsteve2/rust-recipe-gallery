# [working-directory: 'backend']
# not in version 1.35, but will be merged soon I think
# ok idk. releases are weird. taskfile is better i guess!
run-backend:
  #!/usr/bin/env bash
  set -eux -o pipefail
  cd go-backend
  go run ./src

create-recipe:
  #!/usr/bin/env bash
  set -eux -o pipefail
  curl --request POST \
    --header "Content-Type: application/json" \
    --data '{"title":"food","steps":"cook"}' http://localhost:3333/recipes

get-recipe:
  #!/usr/bin/env bash
  set -eux -o pipefail
  curl --request GET \
    http://localhost:3333/recipes

get-recipe-by-id:
  #!/usr/bin/env bash
  set -eux -o pipefail
  curl --request GET \
    http://localhost:3333/recipes/01947b35-2a34-78ef-8119-1f39ff636ff8

delete-recipe-by-id:
  #!/usr/bin/env bash
  set -eux -o pipefail
  curl --request DELETE \
    http://localhost:3333/recipes/01947b35-2a34-78ef-8119-1f39ff636ff8

update-recipe-by-id:
  #!/usr/bin/env bash
  set -eux -o pipefail
  curl --request POST \
    --header "Content-Type: application/json" \
    --data '{"title":"lebron","steps":"james"}' http://localhost:3333/recipes/01947b35-2a34-78ef-8119-1f39ff636ff8
