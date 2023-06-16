export tag=`git rev-parse --short HEAD`
docker build -t planner-be-amd64:$tag -f Dockerfile.azure .
docker tag planner-be-amd64:$tag jayliaoo.azurecr.io/planner-be:$tag
docker push jayliaoo.azurecr.io/planner-be:$tag
