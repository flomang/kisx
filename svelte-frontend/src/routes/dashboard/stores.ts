import { writable } from 'svelte/store';

export class Filter {
    k: number;
    value: string;
    type: string;
    constructor(k: number, name: string, v: string) {
        this.k = k;
        this.type = name;
        this.value = v;
    }

    toString() {
        return `${this.type}: ${this.value}`;
    }
}

const initialArray: Filter[] = [
    // { k: 0, v: "keyword" },
    // { k: 1, v: "category" },
    // { k: 2, v: "condition" },
    // { k: 3, v: "status" },
];

export const count = writable(0);
export const filters = writable(initialArray);
