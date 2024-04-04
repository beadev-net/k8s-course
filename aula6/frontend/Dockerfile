# Use the official Nginx image as a base
FROM nginx:latest

# Remove default Nginx static assets
RUN rm -rf /usr/share/nginx/html/*

# Copie o index.html para o diretório de Nginx static assets
COPY src/ /usr/share/nginx/html

# Exponha a porta 80 para acesso externo
EXPOSE 80

# Comando para iniciar o Nginx quando o contêiner for iniciado
CMD ["nginx", "-g", "daemon off;"]
