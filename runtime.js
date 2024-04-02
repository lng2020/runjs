import { op_read_file, op_write_file, op_remove_file } from "ext:core/ops";
((globalThis) => {
    const core = Deno.core;

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args) => {
            core.print(`[OUT]: ${argsToMessage(...args)}\n`, false);
        },
        error: (...args) => {
            core.print(`[ERR]: ${argsToMessage(...args)}\n`, true);
        },
    }

    globalThis.runjs = {
        readFile: (path) => {
            return op_read_file(path);
        },
        writeFile: (path, contents) => {
            return op_write_file(path, contents);  
        },
        removeFile: (path) => {
            return op_remove_file(path);
        },
    }
})