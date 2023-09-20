import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { env } from "src/app/environment/environment";
import { ShowModel } from "src/app/models/show";

@Injectable({
    providedIn: "root"
})
export class ShowService {
    /**
     * @classdesc - This class handles all the CRUD operations for shows
     */

    // BASE URL
    private BASE_URL = env.WatchListrAPIurl;

    constructor(private http: HttpClient) { }

    // Get added shows
    getAddedShows(): Observable<ShowModel> {
        return this.http.get<ShowModel>(this.BASE_URL + "/shows");
    }

    // Add show
    addShow(payload: ShowModel): Observable<ShowModel> {

        const headers = new HttpHeaders({
            "Content-Type": "application/json"
        });

        return this.http.post<ShowModel>(this.BASE_URL + "/show", payload, { headers });
    }
}