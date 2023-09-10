import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

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

@Component({
    selector: "home",
    templateUrl: "home.component.html",
    styleUrls: ["home.component.css"],
    standalone: true,
    imports: [CommonModule, FontAwesomeModule, RouterModule]
})
export class HomeComponent {
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

    

    
}