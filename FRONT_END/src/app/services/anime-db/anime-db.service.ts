import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Observable } from "rxjs";
import { env } from "src/app/environment/environment";

@Injectable({
    providedIn: "root"
})
/**
 * 
 * @classdesc - This class handles requests from the AnimeDB API
 * 
 */
export class AnimeDbService {
    private readonly API_KEY: string = env.RapidAPIKey;
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

    req_GET_NEW_SHOWS(): Observable<any> {
        // TEMP QUERY STRING
        const URL = `${this.BASE_URL}?page=1&size=5&genres=Award%20Winning&sortBy=ranking&sortOrder=asc`;

        const headers = new HttpHeaders({
            "X-RapidAPI-Key": this.API_KEY,
            "X-RapidAPI-Host": this.HOST
        });
        return this.http.get(URL, { headers });
    }
}