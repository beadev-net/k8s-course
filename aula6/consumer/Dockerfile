# Use a imagem oficial do Python como base
FROM python:3.9-slim

# Define variáveis de ambiente
ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

# Define o diretório de trabalho no contêiner
WORKDIR /app

# Copia o arquivo de dependências para o diretório de trabalho
COPY requirements.txt .

# Instala as dependências
RUN pip install --no-cache-dir -r requirements.txt

# Copia todo o conteúdo do diretório atual para o diretório de trabalho
COPY ./src .

# Comando para iniciar a aplicação
CMD ["python", "app.py"]
