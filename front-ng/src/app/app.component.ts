import {Component,Input, ViewContainerRef} from '@angular/core'; 

import { Router, RouterLink, RouterLinkActive, RouterModule, RouterOutlet } from '@angular/router'; 
import { trigger, state, style, transition, animate } from '@angular/animations';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { BrowserModule } from '@angular/platform-browser';
import { CommonModule } from '@angular/common';
import { AlbumGridComponent } from './album-grid/album-grid.component';
import { NavbarComponent } from "./navbar/navbar.component";
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    standalone: true,
    imports: [
        RouterModule, AlbumGridComponent, RouterOutlet, RouterLink, RouterLinkActive,
        NavbarComponent
    ]
})

export class AppComponent {
    constructor(private router: Router) {}
}

