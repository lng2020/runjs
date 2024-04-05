globalThis.extension = {
    readFile: (path) => {
        return ops.op_read_file(path);
    },
    writeFile: (path, contents) => {
        return ops.op_write_file(path, contents);  
    },
    removeFile: (path) => {
        return ops.op_remove_file(path);
    },
    fetch: (url) => {
        return ops.op_fetch(url);
    }
}