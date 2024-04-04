my_console.log("Hello", "runjs!");
my_console.error("Error", "runjs!");

const path = "./log.txt";
try {
    const contents = await extension.readFile(path);
    my_console.log("Read from a file", contents);
} catch (e) {
    my_console.error("Error reading from a file", path, e);
}

await extension.writeFile(path, "I can write to a file!");
const contents = await extension.readFile(path);
my_console.log("Read from a file", path, "contents:", contents);
my_console.log("Removing file", path);
extension.removeFile(path);
my_console.log("File removed", path);