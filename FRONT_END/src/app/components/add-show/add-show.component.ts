import { CommonModule } from "@angular/common";
import { Component, ElementRef, OnInit, ViewChild } from "@angular/core";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { faBookmark } from "@fortawesome/free-regular-svg-icons";
import {
    faHome,
    faGear,
    faPlusCircle,
    faWindowRestore
} from "@fortawesome/free-solid-svg-icons";
import {
    faInstagram,
    faFacebookF,
    faLinkedinIn,
    faXTwitter
} from "@fortawesome/free-brands-svg-icons";
import { RouterModule } from "@angular/router";
import {
    FormBuilder,
    FormGroup,
    FormsModule,
    ReactiveFormsModule,
    Validators
} from "@angular/forms";
import { AnimeDbService } from "src/app/services/anime-db/anime-db.service";

@Component({
    selector: "add-show",
    templateUrl: "add-show.component.html",
    styleUrls: ["add-show.component.css"],
    standalone: true,
    imports: [CommonModule, RouterModule, FontAwesomeModule, FormsModule, ReactiveFormsModule]
})
export class AddShowComponent implements OnInit {
    // ICONS
    Home = faHome;
    Shows = faBookmark;
    Settings = faGear;
    Add = faPlusCircle;
    More = faWindowRestore;
    // SOCIAL MEDIA
    Instagram = faInstagram;
    Facebook = faFacebookF;
    Linkedin = faLinkedinIn;
    XTwitter = faXTwitter;

    ADD_SHOW!: FormGroup;
    SEARCH_RESULTS: any[] = [];

    /**
    * 
    * @description - Selectors i.e select native HTML element  | showName, additionalInfo
    * 
    */
    @ViewChild("showName", { static: false }) showName = {} as ElementRef;
    @ViewChild("additionalInfo", { static: false }) additionalInfo = {} as ElementRef;

    // IJ=NJECT SERVICES
    constructor(
        private formBuilder: FormBuilder,
        private animeDb: AnimeDbService) { }

    /**
     * 
     * @description - CREATE NEW FORM GROUP ON APP INITIALIZATION
     * 
     */
    ngOnInit() {
        this.ADD_SHOW = this.formBuilder.group({
            show_name: ["", Validators.required],
            additional_info: ["", Validators.required],
        });
    }

    req_GET_SHOW_DETAILS() {
        let SEARCH_QUERY: string = this.showName.nativeElement.value
        this.animeDb.req_GET_SHOW(SEARCH_QUERY);

        this.animeDb.req_GET_SHOW(SEARCH_QUERY).subscribe((response) => {
            this.SEARCH_RESULTS = response.data;
        },
            (error) => {
                console.error(error);
            }
        );
    }
}