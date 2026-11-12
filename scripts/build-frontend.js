const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const output = path.join(root, "dist");

fs.mkdirSync(output, { recursive: true });
fs.copyFileSync(path.join(root, "index.html"), path.join(output, "index.html"));
fs.copyFileSync(path.join(root, "control-center.html"), path.join(output, "control-center.html"));
fs.copyFileSync(path.join(root, "toggle.html"), path.join(output, "toggle.html"));
fs.copyFileSync(path.join(root, "overlay.html"), path.join(output, "overlay.html"));
fs.copyFileSync(path.join(root, "settings.html"), path.join(output, "settings.html"));
