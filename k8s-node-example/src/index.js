const http = require("http");

const port = process.env.PORT || 7878;

const server = http.createServer((req, res) => {
  const datetime = new Date().toISOString();
  console.log(`${datetime} - INFO - Request received`);

  if (req.url == "/turma") {
    fibonacci(34);

    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end("Hello turma!\n");
  } else {
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end("Hello World!\n");
  }
});

function fibonacci(n) {
  if (n <= 1) return 1;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

server.listen(port, () => {
  console.log(`Server running on port: ${port}`);
});

module.exports = server;
