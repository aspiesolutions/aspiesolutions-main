// typings  will need to be manually created for now

export type ExecutionArgs = {
    source: string,
    operationName?: string
}
export type ExecutionResult = {
    errors:string,
    data:string
}
//** executes a graphql schema with a string as input */
export function execute_juniper(args:ExecutionArgs): string;
// prints the internal graphql SDL representation
export function get_sdl(): string;