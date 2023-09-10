import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Observable } from "rxjs";

@Injectable({
    providedIn: "root"
})
export class AnimeDbService {
    private readonly API_KEY: string = "2254de9a0amsh736177c36efdcfdp104251jsn7005775bb33b";
    private readonly HOST: string = "anime-db.p.rapidapi.com";
    private readonly BASE_URL: string = "https://anime-db.p.rapidapi.com/anime";

    constructor(private http: HttpClient) { }

    req_GET_SHOW(SEARCH_QUERY: string): Observable<any> {
        const URL: string = `${this.BASE_URL}?page=1&size=10&search=${SEARCH_QUERY}`;

        const headers = new HttpHeaders({
            "X-RapidAPI-Key": this.API_KEY,
            "X-RapidAPI-Host": this.HOST
        });
        return this.http.get(URL, { headers });
    }
}