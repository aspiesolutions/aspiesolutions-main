// typings  will need to be manually created for now
//** executes a graphql schema with a string as input */
export function execute_juniper(source:string): string;
// prints the internal graphql SDL representation
export function get_sdl(): string;