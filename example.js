console.log("Hello", "runjs!");
console.error("Error", "runjs!");

const path = "./log.txt";
try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (e) {
    console.error("Error reading from a file", path, e);
}

await runjs.writeFile(path, "I can write to a file!");
const contents = await runjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
runjs.removeFile(path);
console.log("File removed", path);