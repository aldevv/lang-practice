import http from "http";

const server = http.createServer((req, res, test) => {
    if (req.method === "GET" && req.url === "/") {
        res.end();
    }
});

server.listen(3001, () => {
    console.log("server on http://localhost:3001");
});
