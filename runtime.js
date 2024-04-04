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
})