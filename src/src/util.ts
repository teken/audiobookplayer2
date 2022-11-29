export function secondsToFormatted(value: number) {
    let string = "";
    let s = Math.floor(value % 3600 % 60);
    let m = Math.floor(value % 3600 / 60);
    let h = Math.floor(value / 3600);

    string = (s < 10 ? "0" : "") + s + string;
    string = (m < 10 ? "0" : "") + m + ":" + string;
    if (h > 0)
        string = h + ":" + string;
    return string;
}

export const groupBy = <K, T>(list: T[], selector: (x: T) => K) => {
    return list.reduce((a, v) => {
        const sv = selector(v);
        if (!a.has(sv)) a.set(sv, [v]);
        else {
            let nv = a.get(sv);
            a.set(sv, [...nv, v]);
        }

        return a;
    }, new Map<K, T[]>());
}