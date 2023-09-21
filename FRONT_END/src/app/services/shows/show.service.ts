import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { env } from "src/app/environment/environment";
import { AddShowModel, ShowModel } from "src/app/models/show";

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
    addShow(payload: AddShowModel): Observable<AddShowModel> {

        const headers = new HttpHeaders({
            "Content-Type": "application/json"
        });

        return this.http.post<AddShowModel>(this.BASE_URL + "/show", payload, { headers });
    }
}