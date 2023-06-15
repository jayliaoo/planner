docker build -t planner-be:$(git rev-parse --short HEAD) -f Dockerfile.alpine.amd64 .
docker tag planner-be:$tag jayliaoo.azurecr.io/planner-be:$tag
docker push jayliaoo.azurecr.io/planner-be:$tag
