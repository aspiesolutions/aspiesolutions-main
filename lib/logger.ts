import pino from "pino";
export const console = pino({
    transport:{
        target:'pino-pretty'
    }
})
export default console;