export tag=`git rev-parse --short HEAD`
docker tag planner-be:$tag jayliaoo.azurecr.io/planner-be:$tag
docker push jayliaoo.azurecr.io/planner-be:$tag
