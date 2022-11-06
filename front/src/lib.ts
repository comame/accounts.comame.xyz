export function diffArray<T>(a: T[], b: T[]): { add: T[]; del: T[] } {
    const adds: T[] = []
    const dels: T[] = []
    for (const va of a) {
        if (!b.includes(va)) {
            dels.push(va)
        }
    }
    for (const vb of b) {
        if (!a.includes(vb)) {
            adds.push(vb)
        }
    }

    return {
        add: adds,
        del: dels,
    }
}
