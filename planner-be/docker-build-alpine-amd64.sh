docker build -t planner-be:$(git rev-parse --short HEAD) -f Dockerfile.alpine.amd64 .
