import {Component,Input, ViewContainerRef} from '@angular/core'; 

import { Router, RouterLink, RouterLinkActive, RouterModule, RouterOutlet } from '@angular/router';
import { GalleryGridComponent } from './gallery-grid/gallery-grid.component';
import { trigger, state, style, transition, animate } from '@angular/animations';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { BrowserModule } from '@angular/platform-browser';
import { CommonModule } from '@angular/common';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    imports: [

        RouterModule,GalleryGridComponent, RouterOutlet, RouterLink, RouterLinkActive, 
      ],
    standalone: true,
   
})

export class AppComponent {
    constructor(private router: Router) {}
}

