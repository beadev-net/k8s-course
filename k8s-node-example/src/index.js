const http = require("http");

const port = process.env.PORT || 7878;

const server = http.createServer((req, res) => {
  const datetime = new Date().toISOString();
  console.log(`${datetime} - INFO - Request received`);

  res.writeHead(200, { "Content-Type": "text/plain" });
  res.end("Hello World!\n");
});

server.listen(port, () => {
  console.log(`Server running on port: ${port}`);
});

module.exports = server;
