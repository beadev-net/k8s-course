# Study Case

## Description
Possuimos uma aplicação de votação simples que permite aos usuários escolher entre duas opções de resposta.  

Nossa aplicação hoje roda com Docker e é composta por dois containers:
- `app`: Container que roda a aplicação em Flask (Python).
- `nginx`: Container que roda o servidor web Nginx (HTML e JS).

Nosso Banco de Dados é o MySQL, que roda em um serviço da nuvem, porém, queremos que ele rode em um container Docker,
pois o custo de manter o banco de dados na nuvem está muito alto.

Hoje nós usamos o Docker Compose para subir a aplicação, porém, queremos que você nos ajude a migrar para o Kubernetes.

Nossa aplicação backend necessita de environment variables para funcionar corretamente. As variáveis de ambiente necessárias são:
- `MYSQL_HOST`: Host do banco de dados MySQL.
- `MYSQL_USER`: Usuário do banco de dados MySQL.
- `MYSQL_PASSWORD`: Senha do banco de dados MySQL.
- `MYSQL_DATABASE`: Nome do banco de dados MySQL.

Gostariamos de aplicar o Banco de Dados MySQL na arquitetura Master-Slave, para garantir a alta disponibilidade.

## Requisitos
- Back-end: Flask para criar a API.
- Front-end: HTML e JavaScript para a interface do usuário.
- Banco de Dados: MySQL para armazenar os votos.
- Docker: Para containerizar a aplicação e o banco de dados.

A aplicação deve registrar os votos em um banco de dados MySQL e permitir a visualização dos resultados em tempo real.