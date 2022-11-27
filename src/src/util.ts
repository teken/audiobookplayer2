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