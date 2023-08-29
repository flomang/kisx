import { writable } from 'svelte/store';

export class Filter {
    key: number;
    value: string;
    type: string;
    constructor(key: number, name: string, value: string) {
        this.key = key;
        this.type = name;
        this.value = value;
    }

    toString() {
        return `${this.type}: ${this.value}`;
    }
}

const initialArray: Filter[] = [];

export const count = writable(0);
export const filters = writable(initialArray);
