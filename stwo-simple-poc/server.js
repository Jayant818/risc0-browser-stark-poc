const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 8080;
const ROOT = process.cwd();

const MIMES = {
    '.html': 'text/html',
    '.js': 'application/javascript',
    '.wasm': 'application/wasm',
    '.css': 'text/css',
    '.json': 'application/json'
};

const server = http.createServer((req, res) => {
    console.log(`${req.method} ${req.url}`);
    
    // Normalize path
    let safePath = path.normalize(req.url).replace(/^(\.\.[/\\])+/, '');
    if (safePath === '/' || safePath === '/www/') safePath = '/www/index.html';
    
    const filePath = path.join(ROOT, safePath);
    
    fs.stat(filePath, (err, stats) => {
        if (err || !stats.isFile()) {
            res.statusCode = 404;
            res.end(`File not found: ${filePath}`);
            return;
        }

        const ext = path.extname(filePath);
        const mime = MIMES[ext] || 'application/octet-stream';
        
        // Add headers for SharedArrayBuffer / WASM threads support if needed in future
        res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
        res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
        res.setHeader('Content-Type', mime);
        
        fs.createReadStream(filePath).pipe(res);
    });
});

server.listen(PORT, () => {
    console.log(`Server running at http://localhost:${PORT}/www/`);
});

server.on('error', (e) => {
    console.error('Server error:', e);
});

