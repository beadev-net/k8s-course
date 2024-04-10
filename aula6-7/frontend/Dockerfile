# Use the official Nginx image as a base
FROM nginx:latest

ARG API_URL=http://localhost:8000

RUN rm -rf /usr/share/nginx/html/*

COPY src/ /usr/share/nginx/html

RUN sed -i "s|\${API_URL}|$API_URL|g" /usr/share/nginx/html/index.html
RUN sed -i "s|\${API_URL}|$API_URL|g" /usr/share/nginx/html/vote.html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

####
# Usage Example:
# > docker compose build frontend --build-arg API_URL=http://localhost:8888
####