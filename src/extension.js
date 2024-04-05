import { op_read_file, op_write_file, op_remove_file, op_fetch } from "ext:core/ops";
globalThis.extension = {
    readFile: (path) => {
        return op_read_file(path);
    },
    writeFile: (path, contents) => {
        return op_write_file(path, contents);  
    },
    removeFile: (path) => {
        return op_remove_file(path);
    },
    fetch: (url) => {
        return op_fetch(url);
    }
}