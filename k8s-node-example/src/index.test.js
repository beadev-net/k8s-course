const server = require("./index.js");
const supertest = require("supertest");

const request = supertest(server);

describe("HTTP Server Test", () => {
  afterAll((done) => {
    server.close(done); // Garanta que o servidor é fechado após os testes
  });

  test("Deve retornar Hello World!", async () => {
    const response = await request.get("/");
    expect(response.statusCode).toBe(200);
    expect(response.text).toBe("Hello World!\n");
  });
});
